use std::error::Error;
use std::{env, io};

pub mod help;
pub mod main_actions;
use main_actions::*;

#[derive(Debug)]
struct Config {
  version: Option<String>,
  help: bool,
  yes_to_all: bool,
}

impl Config {
  fn from_iter(args: &mut impl Iterator<Item = String>) -> Self {
    let mut config = Self {
      version: None,
      help: false,
      yes_to_all: false,
    };

    for arg in args.by_ref() {
      match arg.as_str() {
        "-h" | "--help" => config.help = true,
        "-y" | "--yes" => config.yes_to_all = true,
        other => config.version = Some(String::from(other)),
      };
    }

    config
  }
}

fn main() {
  let mut args = env::args().skip(2);
  let config = Config::from_iter(&mut args);

  dbg!(&config);

  if config.help {
    help::usage();
  }

  let version = match config.version {
    Some(version) => version,
    None => help::usage(),
  };

  let cargo_toml = get_cargo_file("./Cargo.toml");
  let cargo_lock = get_cargo_file("./Cargo.lock");
  let new_version = get_new_version(&version, &cargo_toml);

  should_show_questions(config.yes_to_all);

  let cargo_toml_updated = get_cargo_toml_updated(&cargo_toml, &new_version);
  let cargo_lock_updated =
    get_cargo_lock_updated(&cargo_lock, &cargo_toml, &new_version);

  try_to_update_cargo_toml(&cargo_toml_updated);
  try_to_update_cargo_lock(&cargo_lock_updated);

  try_to_git_add();
  try_to_git_commit(&new_version);
  try_to_git_tag(&new_version);
}

fn should_show_questions(yes_to_all: bool) {
  if yes_to_all {
    return;
  }

  match questions() {
    Ok(_) => (),
    Err(e) => help::error_exit(&e.to_string()),
  };
}

fn questions() -> Result<(), Box<dyn Error>> {
  let mut answer1 = String::new();
  let mut answer2 = String::new();

  println!("Have you already run the command `cargo build` and your application has no erros? [Y]es [N]o:");
  io::stdin().read_line(&mut answer1)?;
  let answer1 = answer1.to_lowercase();

  if answer1 != "y" || answer1 != "yes" {
    println!("Answer 1: {}", answer1);
    println!("WARN: run `cargo build` and make sure the project has no errors before using `cargo-v`");
    return Ok(());
  }

  println!("All important files are \"git added\" and \"commited\" (specially `Cargo.toml` and `Cargo.lock`)? [Y]es [N]o:");
  io::stdin().read_line(&mut answer2)?;
  let answer2 = answer2.to_lowercase();
  if answer2 != "y" || answer2 != "yes" {
    println!("Answer 2: {}", answer2);
    println!("WARN: please, commit all important files before run `cargo-v`");
    return Ok(());
  }

  Ok(())
}
