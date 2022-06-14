use cargo_v;
use std::process::Command;
use std::{env, fs, io};

enum Operation {
  Help,
  Version(String),
}

fn main() {
  let mut args = env::args().skip(2);

  let operation = match args.next() {
    Some(argument) => parse_cli(argument),
    None => usage(),
  };

  let cargo_toml = match fs::read_to_string("./Cargo.toml") {
    Ok(toml) => toml,
    Err(e) => error_exit(&format!("failed to read Cargo.toml file: {}", e)),
  };

  let new_version = match operation {
    Operation::Version(v) => match v.as_str() {
      "patch" => cargo_v::VersionLabel::Patch,
      "minor" => cargo_v::VersionLabel::Minor,
      "major" => cargo_v::VersionLabel::Major,
      version => cargo_v::VersionLabel::NumericVersion(String::from(version)),
    },
    Operation::Help => usage(),
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

fn usage() -> ! {
  let usage = concat!(
    "USAGE:\n",
    "    cargo v <version>\n",
    "    cargo v [options]\n",
    "ARGUMENTS:\n",
    "    version       Can be one of \"patch\", \"minor\", \"major\" or a string like \"v1.2.5\"\n",
    "OPTIONS:\n",
    "    -h, --help    Prints this message."
  );

  println!("{}", usage);
  std::process::exit(0)
}

fn operation_of_string(arg: &str) -> Operation {
  match arg {
    "h" | "help" => Operation::Help,
    _ => {
      eprintln!("ERROR: invalid argument \"{}\"", arg);
      usage();
    }
  }
}

fn parse_cli(argument: String) -> Operation {
  if argument.starts_with('-') || argument.starts_with("--") {
    let arg_name = argument
      .chars()
      .filter(|char| char != &'-')
      .collect::<String>();

    operation_of_string(&arg_name)
  } else {
    Operation::Version(argument)
  }
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
