use cargo_v;
use std::process::Command;
use std::{env, fs, io};

fn main() {
  let mut args = env::args().skip(2);

  let new_version = match args.next() {
    Some(v) => v,
    None => error_exit(
      "failed to read the version: you must pass (patch, minor, major, etc.)",
    ),
  };

  let cargo_toml = match fs::read_to_string("./Cargo.toml") {
    Ok(toml) => toml,
    Err(e) => error_exit(&format!("failed to read Cargo.toml file: {}", e)),
  };

  let new_version = match new_version.as_str() {
    "patch" => cargo_v::VersionLabel::Patch,
    "minor" => cargo_v::VersionLabel::Minor,
    "major" => cargo_v::VersionLabel::Major,
    v => cargo_v::VersionLabel::NumericVersion(String::from(v)),
  };

  let cargo_toml_updated =
    match cargo_v::update_version(&cargo_toml, &new_version) {
      Ok(toml) => toml,
      Err(e) => error_exit(&e.to_string()),
    };

  let new_version = match cargo_v::get_version(&cargo_toml_updated) {
    Ok(v) => v,
    Err(e) => error_exit(&e.to_string()),
  };
  let new_version = cargo_v::tuple_version_to_string(&new_version);

  if let Err(e) = update_cargo_toml(&cargo_toml_updated) {
    error_exit(&format!("failed to write on file Cargo.toml: {e}"));
  }

  if let Err(e) = run_build() {
    error_exit(&format!("failed to run build: {e}"));
  }

  if let Err(e) = git_add() {
    error_exit(&format!("failed to run git add: {e}"));
  }

  if let Err(e) = git_commit(&new_version) {
    error_exit(&format!("failed to run git commit: {e}"));
  }

  if let Err(e) = git_tag(&new_version) {
    error_exit(&format!("failed to run git tag: {e}"));
  }
}

fn error_exit(msg: &str) -> ! {
  eprintln!("ERROR: {}", msg);
  std::process::exit(1);
}

fn update_cargo_toml(new_cargo_toml: &str) -> io::Result<()> {
  fs::write("./Cargo.toml", new_cargo_toml)?;
  Ok(())
}

fn run_build() -> io::Result<()> {
  Command::new("cargo")
    .args(["build", "--release"])
    .output()?;
  Ok(())
}

fn git_add() -> io::Result<()> {
  Command::new("git")
    .args(["add", "Cargo.toml", "Cargo.lock"])
    .output()?;
  Ok(())
}

fn git_commit(version: &str) -> io::Result<()> {
  let version = &format!("v{}", version);
  Command::new("git")
    .args(["commit", "-m", version])
    .output()?;
  Ok(())
}

fn git_tag(version: &str) -> io::Result<()> {
  let version = &format!("v{}", version);
  Command::new("git")
    .args(["tag", "-a", version, "-m", version])
    .output()?;
  Ok(())
}
