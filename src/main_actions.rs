use crate::cli_parser::Operation;
use crate::help;
use cargo_v;
use std::fs;
use std::process::Command;

pub fn get_cargo_toml_updated(cargo_toml: &str, new_version: &str) -> String {
  cargo_v::set_version_in_cargo_toml(cargo_toml, new_version)
}

pub fn get_cargo_lock_updated(
  cargo_lock: &str,
  cargo_toml: &str,
  new_version: &str,
) -> String {
  let project_name = match cargo_v::get_name_from_cargo_toml(cargo_toml) {
    Ok(name) => name,
    Err(e) => help::error_exit(&e.to_string()),
  };

  cargo_v::set_version_in_cargo_lock(cargo_lock, &project_name, new_version)
}

pub fn get_new_version(operation: &Operation, cargo_toml: &str) -> String {
  let new_version = match operation {
    Operation::Help => help::usage(),
    Operation::Version(version) => {
      match cargo_v::parse_string_to_version_label(&version) {
        Ok(version) => version,
        Err(e) => help::usage_error(&e.to_string()),
      }
    }
  };

  match cargo_v::get_updated_version(cargo_toml, &new_version) {
    Ok(version) => version,
    Err(e) => help::error_exit(&e.to_string()),
  }
}

pub fn get_cargo_file(file: &str) -> String {
  match fs::read_to_string(format!("./{file}")) {
    Ok(toml) => toml,
    Err(e) => help::error_exit(&format!("failed to read {file} file: {e}")),
  }
}

pub fn try_to_update_cargo_toml(new_cargo_toml: &str) {
  if let Err(e) = fs::write("./Cargo.toml", new_cargo_toml) {
    help::error_exit(&format!("failed to write on file Cargo.toml: {e}"));
  }
}

pub fn try_to_update_cargo_lock(new_cargo_lock: &str) {
  if let Err(e) = fs::write("./Cargo.lock", new_cargo_lock) {
    help::error_exit(&format!("failed to write on file Cargo.lock: {e}"));
  };
}

pub fn try_to_git_add() {
  let result = Command::new("git")
    .args(["add", "Cargo.toml", "Cargo.lock"])
    .output();

  if let Err(e) = result {
    help::error_exit(&format!("failed to run git add: {e}"));
  }
}

pub fn try_to_git_commit(version: &str) {
  let version = &format!("v{}", version);
  let result = Command::new("git").args(["commit", "-m", version]).output();

  if let Err(e) = result {
    help::error_exit(&format!("failed to run git commit: {e}"));
  }
}

pub fn try_to_git_tag(version: &str) {
  let version = &format!("v{}", version);
  let result = Command::new("git")
    .args(["tag", "-a", version, "-m", version])
    .output();

  if let Err(e) = result {
    help::error_exit(&format!("failed to run git tag: {e}"));
  }
}
