use std::fmt::{Display, Formatter};
use serde::Serialize;
use std::io::{BufRead, BufReader, Lines};
use std::process::{Child, ChildStdout, Command, ExitStatus, Stdio};
use crate::process_manager::process::ProcessState::{NotStarted, Running, Stopped};

pub enum ProcessState {
    NotStarted,
    Running,
    Stopped(i32),
}

pub struct Process {
    program: String,
    cmd: Command,
    child: Option<Child>,
    reader: Option<Lines<BufReader<ChildStdout>>>,
}

impl Process {
    pub fn new(program: String) -> Self {
        let mut cmd = Command::new(&program);
        cmd.stdout(Stdio::piped());

        Process { program, cmd, child: None, reader: None }
    }

    pub fn start(&mut self) -> Result<(), &'static str> {
        if self.child.is_some() {
            return Err("already started").into();
        }

        let mut child = self.cmd.spawn()
            .expect("failed to spawn command");

        let stdout = child.stdout.take()
            .expect("child did not have stdout");

        let reader = BufReader::new(stdout).lines();

        self.child = Some(child);
        self.reader = Some(reader);

        return Ok(());
    }

    pub fn program(&self) -> String {
        return self.program.clone();
    }

    pub fn state(&mut self) -> ProcessState {
        if self.child.is_none() {
            return NotStarted;
        }

        let child = self.child.as_mut().unwrap();
        let exit_code = child.try_wait().unwrap_or(None);

        if exit_code.is_none() {
            return Running;
        }

        return Stopped(exit_code.unwrap().code().unwrap());
    }

    pub fn details(&mut self) -> (Vec<String>, Option<ExitStatus>) {
        let mut lines = vec![];
        for line_result in self.reader.as_mut().unwrap() {
            match line_result {
                Ok(line) => lines.push(line),
                Err(_) => ()
            }
        }
        let child = self.child.as_mut().unwrap();
        let exit_code = child.try_wait().unwrap_or(None);

        return (lines, exit_code);
    }

    pub fn stop(&mut self) -> Result<i32, &'static str> {
        if self.child.is_none() {
            return Err("not started").into();
        }

        let mut child = self.child.take().unwrap();
        self.reader.take().unwrap();

        if let Ok(exit_code) = child.try_wait() {
            return Ok(exit_code.unwrap().code().unwrap());
        }

        if let Err(_) = child.kill() {
            return Err("killing error");
        }
        Ok(child.wait().unwrap().code().unwrap())
    }
}