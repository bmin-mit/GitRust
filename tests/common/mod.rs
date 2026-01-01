use assert_cmd::{Command, cargo_bin_cmd};
use tempfile::TempDir;

pub struct TestCommand {
    _pwd: TempDir,
    cmd: Command,
}

impl TestCommand {
    pub fn new(mut cmd: Command) -> Self {
        let mut pwd = TempDir::with_prefix("gitrust").unwrap();
        pwd.disable_cleanup(true);
        cmd.current_dir(pwd.path());
        Self { _pwd: pwd, cmd }
    }

    pub fn cmd(&mut self) -> &mut Command {
        &mut self.cmd
    }
}

pub fn create_cmd() -> TestCommand {
    TestCommand::new(cargo_bin_cmd!())
}

pub fn create_git_cmd() -> TestCommand {
    let cmd = Command::new("git");
    TestCommand::new(cmd)
}
