mod process;

use std::process::ExitStatus;
use crate::process_manager::process::{Process, ProcessState};

struct ProcessManager {
    processes: Vec<Process>,
}

impl ProcessManager {
    pub fn new() -> Self {
        let processes = vec![];

        ProcessManager { processes }
    }

    pub fn add(&mut self, program: String) -> Vec<(String, ProcessState)> {
        self.processes.push(Process::new(program));
        return self.list_all();
    }

    pub fn start(&mut self, n: usize) -> Result<(), &'static str> {
        self.processes[n].start()
    }

    pub fn stop(&mut self, n: usize) -> Result<i32, &'static str> {
        self.processes[n].stop()
    }

    pub fn details(&mut self, n: usize) -> (Vec<String>, Option<ExitStatus>) {
        let process = &mut self.processes[n];
        return process.details();
    }

    pub fn list_all(&mut self) -> Vec<(String, ProcessState)> {
        let mut states = vec![];
        for process in self.processes.iter_mut() {
            states.push((process.program(), process.state()));
        }

        return states;
    }
}
