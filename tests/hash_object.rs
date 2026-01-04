mod common;

use common::TestCommand;
use std::fmt::format;

fn create_cmd() -> TestCommand {
    let mut cmd = common::create_cmd();
    cmd.cmd().arg("hash-object");
    cmd
}
fn create_git_cmd() -> TestCommand {
    let mut cmd = common::create_git_cmd();
    cmd.cmd().arg("hash-object");
    cmd
}

fn test_hash_object_from_stdin(stdin_string: &str) {
    let mut test_cmd = create_cmd();
    let cmd = test_cmd.cmd();
    cmd.arg("--stdin").write_stdin(stdin_string);

    let mut test_git_cmd = create_git_cmd();
    let git_cmd = test_git_cmd.cmd();
    git_cmd.arg("--stdin").write_stdin(stdin_string);

    let git_output = git_cmd.unwrap();
    cmd.assert().stdout(git_output.stdout);
}

#[test]
fn test_hash_object_from_stdin_1() {
    test_hash_object_from_stdin("abcdef");
}

#[test]
fn test_hash_object_from_stdin_2() {
    test_hash_object_from_stdin("lorem ipsum dolor sit amet");
}

#[test]
fn test_hash_object_from_stdin_3() {
    test_hash_object_from_stdin("123567890");
}

#[test]
fn test_hash_object_from_stdin_4() {
    test_hash_object_from_stdin("\n\t\0\n");
}
