# Welcome to cargo-v 👋

[![Documentation](https://img.shields.io/badge/documentation-yes-brightgreen.svg)](https://github.com/fdaciuk/cargo-v#readme)
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
[![Twitter: fdaciuk](https://img.shields.io/twitter/follow/fdaciuk.svg?style=social)](https://twitter.com/fdaciuk)

> An easy way to update the version of your package

### 🏠 [Homepage](https://github.com/fdaciuk/cargo-v)

## Install

```sh
cargo install cargo-v
```

## Usage

You can update the version of your project using the command:

```sh
cargo v patch
```

The above command will update the `patch` part of version from your `Cargo.toml`, 
it will run `cargo build` to update `Cargo.lock` file, and create a commit and 
a git tag with the new version, if you are using git.

You can use four options to update the version: `patch`, `minor`, `major` or a 
manually typed version (like `1.0.1`).

### patch

will change the latest number of version:

**Command:**

```sh
cargo v patch
```

**Update in Cargo.toml:**

```toml
[package]
name = "cargo-v"
version = "0.0.2"
#              ^ This number will be updated
```

### minor

will change the middle number of version:

**Command:**

```sh
cargo v minor
```

**Update in Cargo.toml:**

```toml
[package]
name = "cargo-v"
version = "0.1.0"
#            ^ This number will be updated
```

### major

will change the first number of version:

**Command:**

```sh
cargo v major
```

**Update in Cargo.toml:**

```toml
[package]
name = "cargo-v"
version = "1.0.0"
#          ^ This number will be updated
```

### manually typed version

will change the version with the exact entry:

**Command:**

```sh
cargo v 1.0.1
```

**Update in Cargo.toml:**

```toml
[package]
name = "cargo-v"
version = "1.0.1"
```

## Run tests

```sh
cargo test
```

## Author

👤 **Fernando Daciuk**

* Website: https://daciuk.dev
* Twitter: [@fdaciuk](https://twitter.com/fdaciuk)
* Github: [@fdaciuk](https://github.com/fdaciuk)
* LinkedIn: [@fdaciuk](https://linkedin.com/in/fdaciuk)

## 🤝 Contributing

Contributions, issues and feature requests are welcome!

Feel free to check [issues page](https://github.com/fdaciuk/cargo-v/issues). 

## Show your support

Give a ⭐️ if this project helped you!


## 📝 License

Copyright © 2022 [Fernando Daciuk](https://github.com/fdaciuk).

This project is **MIT** licensed.

***

_This README was generated with ❤️ by [readme-md-generator](https://github.com/kefranabg/readme-md-generator)_
