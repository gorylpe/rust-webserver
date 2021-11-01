use std::io::{BufRead, BufReader, Lines};
use std::process::{Child, ChildStdout, Command, ExitStatus, Stdio};

struct Process {
    cmd: Command,
    child: Option<Child>,
    reader: Option<Lines<BufReader<ChildStdout>>>,
}

impl Process {
    pub fn new() -> Process {
        let mut cmd = Command::new("echo 123");
        cmd.stdout(Stdio::piped());

        Process { cmd, child: None, reader: None }
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

    pub fn state(&mut self) -> (Vec<String>, Option<ExitStatus>) {
        let mut lines = vec![];
        for line_result in self.reader.as_mut().unwrap() {
            match line_result {
                Ok(line) => lines.push(line),
                Err(_) => ()
            }
        }
        let child = self.child.as_mut().unwrap();
        let exit_code = child.try_wait().unwrap_or(None);

        return (lines, exit_code)
    }

    pub fn stop(&mut self) -> Result<i32, &'static str> {
        if self.child.is_none() {
            return Err("not started").into();
        }

        let mut child = self.child.take().unwrap();
        self.reader.take().unwrap();

        if let Err(_) = child.kill() {
            return Err("killing error");
        }
        Ok(child.wait().unwrap().code().unwrap())
    }
}