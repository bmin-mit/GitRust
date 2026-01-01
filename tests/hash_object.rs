use assert_cmd::cargo::cargo_bin_cmd;
use assert_cmd::Command;

fn create_cmd() -> Command {
    let mut cmd = cargo_bin_cmd!();
    cmd.arg("hash-object");

    cmd
}

fn create_git_cmd() -> Command {
    let mut cmd = Command::new("git");
    cmd.arg("hash-object");

    cmd
}

fn test_hash_object_from_stdin(stdin_string: &str) {
    let mut cmd = create_cmd();
    cmd.write_stdin(stdin_string);

    let mut git_cmd = create_git_cmd();
    git_cmd.arg("--stdin")
        .write_stdin(stdin_string);
    let git_output = git_cmd.unwrap();

    cmd.assert().stdout(git_output.stdout).success();
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