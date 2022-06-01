use cargo_v;
use std::{env, error::Error, fs, process::Command};

fn main() {
  let mut args = env::args();
  args.next();

  let new_version_input = args
    .next()
    .expect("You must pass the version (patch, minor, major)");

  let file =
    fs::read_to_string("./Cargo.toml").expect("Can't read Cargo.toml file.");

  let new_version_enum = match new_version_input.as_str() {
    "patch" => cargo_v::VersionLabel::Patch,
    "minor" => cargo_v::VersionLabel::Minor,
    "major" => cargo_v::VersionLabel::Major,
    v => cargo_v::VersionLabel::NumericVersion(String::from(v)),
  };

  let cargo_toml_updated = cargo_v::update_version(&file, new_version_enum);
  let new_version = cargo_v::get_version(&cargo_toml_updated);
  let new_version = cargo_v::tuple_version_to_string(new_version);

  if update_cargo_toml(&cargo_toml_updated).is_err() {
    println!("Error trying to write on file Cargo.toml");
  }

  git_add();
  git_commit(&new_version);
  git_tag(&new_version);
}

fn update_cargo_toml(new_content: &str) -> Result<(), Box<dyn Error>> {
  fs::write("./Cargo.toml", new_content)?;
  Command::new("cargo").args(["build"]).spawn()?;
  Ok(())
}

fn git_add() {
  let _ = Command::new("git")
    .args(["add", "Cargo.toml", "Cargo.lock"])
    .spawn();
}

fn git_commit(version: &str) {
  let version = &format!("'v{}'", version);
  let _ = Command::new("git").args(["commit", "-m", version]).spawn();
}

fn git_tag(version: &str) {
  let version = &format!("v{}", version);
  let message = &format!("'{}'", version);
  let _ = Command::new("git")
    .args(["tag", "-a", version, "-m", message])
    .spawn();
}
