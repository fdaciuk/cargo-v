use std::error::Error;

mod parser;
pub use parser::*;

#[derive(Debug, PartialEq)]
pub enum VersionLabel {
  Major,
  Minor,
  Patch,
  NumericVersion(String),
}

pub fn parse_string_to_version_label(
  string: &str,
) -> Result<VersionLabel, Box<dyn Error>> {
  match string {
    "patch" => Ok(VersionLabel::Patch),
    "minor" => Ok(VersionLabel::Minor),
    "major" => Ok(VersionLabel::Major),
    string => {
      if is_valid_numeric_version(string) {
        return Ok(VersionLabel::NumericVersion(String::from(string)));
      }

      Err("invalid string version")?
    }
  }
}

fn is_valid_numeric_version(string: &str) -> bool {
  string.split('.').take(3).all(|item| {
    let number: Result<u32, _> = item.parse();
    match number {
      Ok(_) => true,
      Err(_) => false,
    }
  })
}

pub fn get_updated_version(
  cargo_toml_content: &str,
  label: &VersionLabel,
) -> Result<String, Box<dyn Error>> {
  let current_version_tuple = get_version(&cargo_toml_content)?;
  let (major, minor, patch) = current_version_tuple;
  let new_version = match label {
    VersionLabel::Major => format!("{}.0.0", major + 1),
    VersionLabel::Minor => format!("{}.{}.0", major, minor + 1),
    VersionLabel::Patch => format!("{}.{}.{}", major, minor, patch + 1),
    VersionLabel::NumericVersion(v) => {
      parse_numeric_version(&current_version_tuple, v)?
    }
  };

  Ok(new_version)
}

pub fn tuple_version_to_string(tuple_version: &(u32, u32, u32)) -> String {
  format!(
    "{}.{}.{}",
    tuple_version.0, tuple_version.1, tuple_version.2,
  )
}

pub fn get_version(
  cargo_toml_content: &str,
) -> Result<(u32, u32, u32), Box<dyn Error>> {
  let version = parser::get_version_from_cargo_toml(cargo_toml_content)?;
  let mut version_split = version.split('.');
  let major = version_split.next().unwrap().parse()?;
  let minor = version_split.next().unwrap().parse()?;
  let patch = version_split.next().unwrap().parse()?;
  Ok((major, minor, patch))
}

fn parse_numeric_version(
  current_version_tuple: &(u32, u32, u32),
  numeric_version: &str,
) -> Result<String, Box<dyn Error>> {
  let new_version = numeric_version.replace("v", "");
  let current_version_string = tuple_version_to_string(current_version_tuple);
  let current_version_number =
    string_version_to_number(&current_version_string)?;
  let new_version_number = string_version_to_number(&new_version)?;

  if new_version_number == current_version_number {
    return Err("new version should not be the same as current version")?;
  }

  if new_version_number < current_version_number {
    return Err("you can not set a version lower than the current version")?;
  }

  Ok(new_version)
}

fn string_version_to_number(version: &str) -> Result<u32, Box<dyn Error>> {
  Ok(version.replace(".", "").parse()?)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_cargo_toml(version: &str) -> String {
    format!("\
[dependencies]
tokio = {{ version = \"1.1.1\" }}

[package]
name = \"cargo-v\"
version = \"{}\"
edition = \"2021\"
description = \"Update the version of your package easily\"
license = \"MIT\"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
[dependencies.dev]
other = {{ version = \"1.1.8\" }}
      ",
    version)
  }

  #[test]
  fn should_parse_string_patch_to_version_label() {
    let actual = parse_string_to_version_label("patch").unwrap();
    let expected = VersionLabel::Patch;
    assert_eq!(actual, expected);
  }

  #[test]
  fn should_parse_string_minor_to_version_label() {
    let actual = parse_string_to_version_label("minor").unwrap();
    let expected = VersionLabel::Minor;
    assert_eq!(actual, expected);
  }

  #[test]
  fn should_parse_string_major_to_version_label() {
    let actual = parse_string_to_version_label("major").unwrap();
    let expected = VersionLabel::Major;
    assert_eq!(actual, expected);
  }

  #[test]
  fn should_parse_numeric_string_to_version_label() {
    let actual = parse_string_to_version_label("1.0.0").unwrap();
    let expected = VersionLabel::NumericVersion(String::from("1.0.0"));
    assert_eq!(actual, expected);
  }

  #[test]
  fn should_fail_to_parse_string_to_version_label() {
    let actual = parse_string_to_version_label("rice");
    match actual {
      Err(e) => assert!(e.to_string().contains("invalid string version")),
      Ok(_) => unreachable!(),
    };
  }

  #[test]
  fn should_get_version_from_cargo_toml() {
    let cargo_toml = get_cargo_toml("2.8.1");
    let actual = get_version(&cargo_toml).unwrap();
    let expected = (2, 8, 1);
    assert_eq!(actual, expected);
  }

  #[test]
  fn should_update_version_by_patch_label() {
    let cargo_toml = get_cargo_toml("0.0.1");
    let new_version =
      get_updated_version(&cargo_toml, &VersionLabel::Patch).unwrap();
    let actual = set_version_in_cargo_toml(&cargo_toml, &new_version);
    let expected = get_cargo_toml("0.0.2");
    assert_eq!(actual, expected);
  }

  #[test]
  fn should_update_version_by_minor_label() {
    let cargo_toml = get_cargo_toml("0.0.2");
    let new_version =
      get_updated_version(&cargo_toml, &VersionLabel::Minor).unwrap();
    let actual = set_version_in_cargo_toml(&cargo_toml, &new_version);
    let expected = get_cargo_toml("0.1.0");
    assert_eq!(actual, expected);
  }

  #[test]
  fn should_update_version_by_major_label() {
    let cargo_toml = get_cargo_toml("0.1.8");
    let new_version =
      get_updated_version(&cargo_toml, &VersionLabel::Major).unwrap();
    let actual = set_version_in_cargo_toml(&cargo_toml, &new_version);
    let expected = get_cargo_toml("1.0.0");
    assert_eq!(actual, expected);
  }

  #[test]
  fn should_update_patch_version_by_hand() {
    let cargo_toml = get_cargo_toml("0.0.1");
    let new_version = get_updated_version(
      &cargo_toml,
      &VersionLabel::NumericVersion(String::from("0.0.2")),
    )
    .unwrap();
    let actual = set_version_in_cargo_toml(&cargo_toml, &new_version);
    let expected = get_cargo_toml("0.0.2");
    assert_eq!(actual, expected);
  }

  #[test]
  fn should_update_minor_version_by_hand() {
    let cargo_toml = get_cargo_toml("0.0.7");
    let new_version = get_updated_version(
      &cargo_toml,
      &VersionLabel::NumericVersion(String::from("0.1.0")),
    )
    .unwrap();
    let actual = set_version_in_cargo_toml(&cargo_toml, &new_version);
    let expected = get_cargo_toml("0.1.0");
    assert_eq!(actual, expected);
  }

  #[test]
  fn should_update_major_version_by_hand() {
    let cargo_toml = get_cargo_toml("2.8.1");
    let new_version = get_updated_version(
      &cargo_toml,
      &VersionLabel::NumericVersion(String::from("3.0.0")),
    )
    .unwrap();
    let actual = set_version_in_cargo_toml(&cargo_toml, &new_version);
    let expected = get_cargo_toml("3.0.0");
    assert_eq!(actual, expected);
  }

  #[test]
  fn should_accept_v_char_in_front_of_version() {
    let cargo_toml = get_cargo_toml("2.8.1");
    let new_version = get_updated_version(
      &cargo_toml,
      &VersionLabel::NumericVersion(String::from("v3.0.0")),
    )
    .unwrap();
    let actual = set_version_in_cargo_toml(&cargo_toml, &new_version);
    let expected = get_cargo_toml("3.0.0");
    assert_eq!(actual, expected);
  }

  #[test]
  fn should_not_set_a_new_version_equal_to_the_current_version() {
    let cargo_toml = get_cargo_toml("2.2.0");
    match get_updated_version(
      &cargo_toml,
      &VersionLabel::NumericVersion(String::from("2.2.0")),
    ) {
      Err(e) => {
        assert!(e
          .to_string()
          .contains("new version should not be the same as current version"));
      }
      _ => unreachable!(),
    };
  }

  #[test]
  fn should_not_set_a_new_version_lower_than_current_version() {
    let cargo_toml = get_cargo_toml("2.1.2");
    match get_updated_version(
      &cargo_toml,
      &VersionLabel::NumericVersion(String::from("2.1.1")),
    ) {
      Err(e) => {
        assert!(e.to_string().contains(
          "you can not set a version lower than the current version"
        ));
      }
      _ => unreachable!(),
    }
  }

  #[test]
  // TODO: Give a more friendly error message
  fn should_not_allow_set_a_negative_version() {
    let cargo_toml = get_cargo_toml("2.2.0");
    match get_updated_version(
      &cargo_toml,
      &VersionLabel::NumericVersion(String::from("-2.2.1")),
    ) {
      Err(e) => {
        assert_eq!(e.to_string(), "invalid digit found in string");
      }
      _ => unreachable!(),
    }
  }
}
