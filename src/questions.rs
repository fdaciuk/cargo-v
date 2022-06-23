use crate::help;
use std::error::Error;
use std::io::{self, Write};

pub fn ask_questions(yes_to_all: bool) {
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

  print!("Have you run the command `cargo build` already to make sure your application has no erros? [Y]es [N]o: ");
  let _ = io::stdout().flush();
  io::stdin().read_line(&mut answer1)?;
  let answer1 = answer1.trim().to_lowercase();

  if answer1 != "y" && answer1 != "yes" {
    return Err("run `cargo build` and make sure the project has no errors before using `cargo-v`")?;
  }

  print!("All important files are \"git added\" and \"commited\" (specially `Cargo.toml` and `Cargo.lock`)? [Y]es [N]o: ");
  let _ = io::stdout().flush();
  io::stdin().read_line(&mut answer2)?;
  let answer2 = answer2.trim().to_lowercase();
  if answer2 != "y" && answer2 != "yes" {
    return Err("please, commit all important files before run `cargo-v`")?;
  }

  Ok(())
}
