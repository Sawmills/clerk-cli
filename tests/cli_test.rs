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

#[test]
fn cli_orgs_sso_help() {
    Command::cargo_bin("clerk")
        .unwrap()
        .args(["orgs", "test-org", "sso", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("add"))
        .stdout(predicate::str::contains("update"));
}

#[test]
fn cli_orgs_sso_list_help() {
    Command::cargo_bin("clerk")
        .unwrap()
        .args(["orgs", "test-org", "sso", "list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("clerk orgs sso list"));
}

#[test]
fn cli_orgs_sso_add_help() {
    Command::cargo_bin("clerk")
        .unwrap()
        .args(["orgs", "test-org", "sso", "add", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--name"))
        .stdout(predicate::str::contains("--provider"))
        .stdout(predicate::str::contains("--domain"))
        .stdout(predicate::str::contains("--metadata-url"));
}

#[test]
fn cli_orgs_sso_add_shows_providers() {
    Command::cargo_bin("clerk")
        .unwrap()
        .args(["orgs", "test-org", "sso", "add", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("saml_okta"))
        .stdout(predicate::str::contains("saml_google"))
        .stdout(predicate::str::contains("saml_microsoft"))
        .stdout(predicate::str::contains("saml_custom"));
}

#[test]
fn cli_orgs_sso_update_help() {
    Command::cargo_bin("clerk")
        .unwrap()
        .args(["orgs", "test-org", "sso", "update", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--name"))
        .stdout(predicate::str::contains("--active"))
        .stdout(predicate::str::contains("--metadata-url"));
}

#[test]
fn cli_orgs_sso_add_requires_name() {
    Command::cargo_bin("clerk")
        .unwrap()
        .args([
            "orgs",
            "test-org",
            "sso",
            "add",
            "--provider",
            "saml_okta",
            "--domain",
            "test.com",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--name"));
}

#[test]
fn cli_orgs_sso_add_requires_provider() {
    Command::cargo_bin("clerk")
        .unwrap()
        .args([
            "orgs", "test-org", "sso", "add", "--name", "Test", "--domain", "test.com",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--provider"));
}

#[test]
fn cli_orgs_sso_add_requires_domain() {
    Command::cargo_bin("clerk")
        .unwrap()
        .args([
            "orgs",
            "test-org",
            "sso",
            "add",
            "--name",
            "Test",
            "--provider",
            "saml_okta",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--domain"));
}

#[test]
fn cli_orgs_sso_add_validates_provider() {
    Command::cargo_bin("clerk")
        .unwrap()
        .args([
            "orgs",
            "test-org",
            "sso",
            "add",
            "--name",
            "Test",
            "--provider",
            "invalid_provider",
            "--domain",
            "test.com",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid"));
}

#[test]
fn cli_orgs_sso_update_requires_connection() {
    Command::cargo_bin("clerk")
        .unwrap()
        .args(["orgs", "test-org", "sso", "update"])
        .assert()
        .failure();
}

#[test]
fn cli_orgs_sso_requires_org() {
    Command::cargo_bin("clerk")
        .unwrap()
        .args(["orgs", "sso", "list"])
        .assert()
        .failure();
}

#[test]
fn cli_orgs_sso_delete_help() {
    Command::cargo_bin("clerk")
        .unwrap()
        .args(["orgs", "test-org", "sso", "delete", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--force"));
}

#[test]
fn cli_orgs_sso_delete_requires_connection() {
    Command::cargo_bin("clerk")
        .unwrap()
        .args(["orgs", "test-org", "sso", "delete"])
        .assert()
        .failure();
}

#[test]
fn cli_sso_help() {
    Command::cargo_bin("clerk")
        .unwrap()
        .args(["sso", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("SSO"));
}

#[test]
fn cli_sso_list_help() {
    Command::cargo_bin("clerk")
        .unwrap()
        .args(["sso", "list", "--help"])
        .assert()
        .success();
}

#[test]
fn cli_sso_missing_api_key() {
    Command::cargo_bin("clerk")
        .unwrap()
        .env_remove("CLERK_API_KEY")
        .args(["sso", "list"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("CLERK_API_KEY"));
}
