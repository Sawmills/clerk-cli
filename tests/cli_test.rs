use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn cli_help() {
    Command::cargo_bin("clerk")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Unofficial Clerk CLI"));
}

#[test]
fn cli_version() {
    Command::cargo_bin("clerk")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("clerk"));
}

#[test]
fn cli_users_help() {
    Command::cargo_bin("clerk")
        .unwrap()
        .args(["users", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("create"));
}

#[test]
fn cli_users_list_help() {
    Command::cargo_bin("clerk")
        .unwrap()
        .args(["users", "list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--limit"))
        .stdout(predicate::str::contains("--query"));
}

#[test]
fn cli_users_create_help() {
    Command::cargo_bin("clerk")
        .unwrap()
        .args(["users", "create", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--email"))
        .stdout(predicate::str::contains("--first-name"));
}

#[test]
fn cli_orgs_create_help() {
    Command::cargo_bin("clerk")
        .unwrap()
        .args(["orgs", "create", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--name"))
        .stdout(predicate::str::contains("--slug"));
}

#[test]
fn cli_orgs_help() {
    Command::cargo_bin("clerk")
        .unwrap()
        .args(["orgs", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("pick"))
        .stdout(predicate::str::contains("members"));
}

#[test]
fn cli_orgs_list_help() {
    Command::cargo_bin("clerk")
        .unwrap()
        .args(["orgs", "list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--limit"))
        .stdout(predicate::str::contains("--fuzzy"));
}

#[test]
fn cli_orgs_members_help() {
    Command::cargo_bin("clerk")
        .unwrap()
        .args(["orgs", "members", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("List members"));
}

#[test]
fn cli_impersonate_help() {
    Command::cargo_bin("clerk")
        .unwrap()
        .args(["impersonate", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("USER_ID"));
}

#[test]
fn cli_completions_bash() {
    Command::cargo_bin("clerk")
        .unwrap()
        .args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("_clerk"));
}

#[test]
fn cli_completions_zsh() {
    Command::cargo_bin("clerk")
        .unwrap()
        .args(["completions", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("#compdef clerk"));
}

#[test]
fn cli_completions_fish() {
    Command::cargo_bin("clerk")
        .unwrap()
        .args(["completions", "fish"])
        .assert()
        .success()
        .stdout(predicate::str::contains("complete -c clerk"));
}

#[test]
fn cli_missing_api_key() {
    Command::cargo_bin("clerk")
        .unwrap()
        .env_remove("CLERK_API_KEY")
        .arg("users")
        .assert()
        .failure()
        .stderr(predicate::str::contains("CLERK_API_KEY"));
}

#[test]
fn cli_unknown_command() {
    Command::cargo_bin("clerk")
        .unwrap()
        .arg("unknown")
        .assert()
        .failure();
}
