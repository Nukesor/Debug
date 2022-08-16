use std::process::{Command, Stdio};

use anyhow::{Context, Result};
use procfs::process::Process;

fn main() -> Result<()> {
    // Create a process that sleeps for 2 seconds.
    let child = Command::new("sleep")
        .arg("2")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Try to read the process. If this fails, the daemon already exited.
    let pid = child.id() as i32;
    let process = Process::new(pid).context("Failed to get proces")?;

    loop {
        let state = process.stat()?.state()?;
        let alive = if process.is_alive() { "alive" } else { "dead" };
        println!("Process {pid} is {alive} with state {:?}", state);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
