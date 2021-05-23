<!-- DO NOT REMOVE - contributor_list:data:start:["Matt-Gleich"]:end -->

# resin [![crates.io](https://img.shields.io/crates/v/resin.svg)](https://crates.io/crates/resin)

[![lint](https://github.com/Matt-Gleich/resin/actions/workflows/lint.yml/badge.svg)](https://github.com/Matt-Gleich/resin/actions/workflows/lint.yml)
[![build](https://github.com/Matt-Gleich/resin/actions/workflows/build.yml/badge.svg)](https://github.com/Matt-Gleich/resin/actions/workflows/build.yml)
[![test](https://github.com/Matt-Gleich/resin/actions/workflows/test.yml/badge.svg)](https://github.com/Matt-Gleich/resin/actions/workflows/test.yml)

⚗️ Superfast CLI interface for the conventional commits commit format

## ❓ What is resin?

resin is a CLI (command-line interface) tool that makes it easy to create commit messages that follow the [conventional commit format](https://www.conventionalcommits.org/). Here is a little demo:

![demo](demo.gif)

This demo will create the following commit message:

```txt
feat(config): add crates.io fields
```

## ✨ Features

### 🚩 Flags

resin has three flags:

1. --help (-h) -> display a help message to the terminal
2. --all (-a) -> run `git add .` before committing the changes
3. --push (-p) -> run `git push` after committing the changes

Super simple and easy to use!

### ⚙️ Configuration

#### 📖 Scopes

You can configure resin to have your custom scopes. Below is an example config:

```toml
scopes = ['docker', 'github actions']
```

#### ✍️ Sign-off message

You can also have a sign-off message that is based off the contents of your `~/.gitconfig` file:

```toml
sign = true
```

This will create a message that will automatically be added to the bottom of your commit message:

```txt
Signed-off-by: Matthew Gleich <git@mattglei.ch>
```

#### 📁 Location

This file can be stored in `~/.config/resin/config.toml` or on a per-project basis by putting it at the root of the project with the same name. You can see a demo of this [for this project](resin.toml)

## 🚀 Install

You can install resin using [cargo](https://doc.rust-lang.org/cargo/index.html):

```bash
cargo install resin
```

## 🙌 Contributing

We would love to have you contribute! Please read the [contributing guide](CONTRIBUTING.md) before submitting a pull request. Thank you in advance!

<!-- prettier-ignore-start -->
<!-- DO NOT REMOVE - contributor_list:start -->
## 👥 Contributors


- **[@Matt-Gleich](https://github.com/Matt-Gleich)**

<!-- DO NOT REMOVE - contributor_list:end -->
<!-- prettier-ignore-end -->
