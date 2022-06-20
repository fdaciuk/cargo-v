use std::env;

pub mod cli_parser;
use cli_parser::parse_cli;

pub mod help;
pub mod main_actions;
use main_actions::*;

fn main() {
  let mut args = env::args().skip(2);

  let operation = match args.next() {
    Some(argument) => parse_cli(argument),
    None => help::usage(),
  };

  let cargo_toml = get_cargo_file("./Cargo.toml");
  let cargo_lock = get_cargo_file("./Cargo.lock");
  let new_version = get_new_version(&operation, &cargo_toml);

  let cargo_toml_updated = get_cargo_toml_updated(&cargo_toml, &new_version);
  let cargo_lock_updated =
    get_cargo_lock_updated(&cargo_lock, &cargo_toml, &new_version);

  try_to_update_cargo_toml(&cargo_toml_updated);
  try_to_update_cargo_lock(&cargo_lock_updated);

  try_to_git_add();
  try_to_git_commit(&new_version);
  try_to_git_tag(&new_version);
}
