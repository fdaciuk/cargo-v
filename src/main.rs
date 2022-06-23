pub mod help;
pub mod main_actions;
mod parse_cli;
mod questions;

use main_actions::*;
use parse_cli::*;
use std::env;

fn main() {
  let mut args = env::args().skip(2);
  let config = CliConfig::from_iter(&mut args);

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

  questions::ask_questions(config.yes_to_all);

  let cargo_toml_updated = get_cargo_toml_updated(&cargo_toml, &new_version);
  let cargo_lock_updated =
    get_cargo_lock_updated(&cargo_lock, &cargo_toml, &new_version);

  try_to_update_cargo_toml(&cargo_toml_updated);
  try_to_update_cargo_lock(&cargo_lock_updated);

  try_to_git_add();
  try_to_git_commit(&new_version);
  try_to_git_tag(&new_version);
}
