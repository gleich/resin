# resin

[![crates.io](https://img.shields.io/crates/v/resin.svg)](https://crates.io/crates/resin)
[![lint](https://github.com/gleich/resin/actions/workflows/lint.yml/badge.svg)](https://github.com/gleich/resin/actions/workflows/lint.yml)
[![build](https://github.com/gleich/resin/actions/workflows/build.yml/badge.svg)](https://github.com/gleich/resin/actions/workflows/build.yml)
[![test](https://github.com/gleich/resin/actions/workflows/test.yml/badge.svg)](https://github.com/gleich/resin/actions/workflows/test.yml)

Fast CLI for conventional commits

## What is resin?

resin is a CLI (command-line interface) tool that makes it easy to create commit messages that follow the [conventional commit format](https://www.conventionalcommits.org/). Here is a little demo:

![demo](demo.gif)

This demo will create the following commit message:

```txt
feat[config]: add crates.io fields
```

## Install

You can install resin using [cargo](https://doc.rust-lang.org/cargo/index.html):

```bash
cargo install resin
```

## Features

### Flags

resin has three flags:

1. --help (-h) -> display a help message to the terminal
2. --all (-a) -> run `git add .` before committing the changes
3. --push (-p) -> run `git push` after committing the changes

Super simple and easy to use!

### Configuration

Configuration is stored in `~/.config/resin/config.toml` or on a per-project basis by putting it at the root of the project with the same name. You can see a demo of this [for this project](resin.toml)

#### Scopes

You can configure resin to have your custom scopes. Below is an example config:

```toml
scopes = ['docker', 'github actions']
```

#### Sign-off message

You can also have a sign-off message that is based off the contents of your `~/.gitconfig` file:

```toml
sign = true
```

This will create a message that will automatically be added to the bottom of your commit message:

```txt
Signed-off-by: Matt Gleich <git@mattglei.ch>
```

#### Parentheses

To have the scope of the commit message be in parentheses instead of square brackets:

```toml
parentheses = true
```
