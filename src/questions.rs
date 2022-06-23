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
  let question1 = create_question("Have you run the command `cargo build` already to make sure your application has no erros? [Y]es [N]o: ")?;
  if not(question1.yes) {
    return Err("run `cargo build` and make sure the project has no errors before using `cargo-v`")?;
  }

  let question2 = create_question("All important files are \"git added\" and \"commited\" (specially `Cargo.toml` and `Cargo.lock`)? [Y]es [N]o: ")?;
  if not(question2.yes) {
    return Err("please, commit all important files before run `cargo-v`")?;
  }

  Ok(())
}

struct Answer {
  yes: bool,
}

fn create_question(question: &str) -> Result<Answer, Box<dyn Error>> {
  let mut answer = String::new();
  print!("{question}");
  io::stdout().flush().unwrap();
  io::stdin().read_line(&mut answer)?;
  let answer = answer.trim().to_lowercase();

  if answer == "y" || answer == "yes" {
    return Ok(Answer { yes: true });
  }

  Ok(Answer { yes: false })
}

fn not(value: bool) -> bool {
  !value
}
