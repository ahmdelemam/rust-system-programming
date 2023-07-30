use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("rust-in-action").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}

#[test]
fn true_works(){
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}
#[test]
fn false_works(){
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}