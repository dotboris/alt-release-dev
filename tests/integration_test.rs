extern crate assert_cmd;

use assert_cmd::prelude::*;
mod test_env;
use test_env::TestEnv;

fn def_all(env: &TestEnv) {
    for command in &["alfa", "bravo", "charlie"] {
        for version in &[1, 2, 3] {
            env.def(
                command, &version.to_string(),
                &env.fixture_path(command, version.clone())
            )
                .assert()
                .success();
        }
    }
}

#[test]
fn def_and_use() {
    let env = TestEnv::new();
    def_all(&env);

    env._use("alfa", "1")
        .assert()
        .success();

    env.command("alfa")
        .assert()
        .success()
        .stdout("alfa1\n");
}

#[test]
fn system_with_no_use() {
    let env = TestEnv::new();
    def_all(&env);

    env.command("bravo")
        .assert()
        .success()
        .stdout("bravo system\n");
}

#[test]
fn reset_with_use_system() {
    let env = TestEnv::new();
    def_all(&env);

    env._use("charlie", "3")
        .assert()
        .success();

    env.command("charlie")
        .assert()
        .success()
        .stdout("charlie3\n");

    env._use("charlie", "system")
        .assert()
        .success();

    env.command("charlie")
        .assert()
        .success()
        .stdout("charlie system\n");
}
