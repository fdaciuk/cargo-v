pub fn update_version(cargo_toml_content: String, version: String) -> String {
  let current_version = get_version(&cargo_toml_content);
  cargo_toml_content.replace(&current_version, &version)
}

fn get_version(cargo_toml_content: &str) -> String {
  let version = cargo_toml_content
    .lines()
    .filter(|line| line.contains("version"))
    .map(|line| {
      line
        .replace("version", "")
        .replace("=", "")
        .replace("\"", "")
        .trim()
        .to_string()
    })
    .next();

  version.expect("Your Cargo.toml file does not have a \"version\" entry")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn try_update_version() {
    let cargo_toml =
      String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.1\"\n");
    let new_version = String::from("0.0.2");
    let expected = "[package]\n name = \"cargo-v\"\n version = \"0.0.2\"\n";

    assert_eq!(update_version(cargo_toml, new_version), expected);
  }

  #[test]
  fn try_update_version_again() {
    let cargo_toml =
      String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.1\"\n");
    let new_version = String::from("0.1.0");
    let expected = "[package]\n name = \"cargo-v\"\n version = \"0.1.0\"\n";

    assert_eq!(update_version(cargo_toml, new_version), expected);
  }

  #[test]
  #[ignore]
  fn should_fail_if_cargo_toml_does_not_have_version() {
    let cargo_toml = String::from("[package]\n name = \"cargo-v\"\n");
    let new_version = String::from("0.1.0");
    let expected = "[package]\n name = \"cargo-v\"\n version = \"0.1.0\"\n";

    assert_eq!(update_version(cargo_toml, new_version), expected);
  }
}
