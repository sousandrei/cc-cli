# cc-cli

**Conventional Commits hook and cli**

[![Build Status](https://img.shields.io/github/actions/workflow/status/sousandrei/cc-cli/main.yaml?branch=main)](https://github.com/sousandrei/cc-cli/actions)
[![crates.io version](https://img.shields.io/crates/v/cc-cli.svg?style=flat-square)](https://crates.io/crates/cc-cli)

<img src="./assets/example.gif" height="500" style="border-radius: 5px" />

## About

Easy cli and git-hook to help with following the [Conventional Commits](https://conventionalcommits.org/en/v1.0.0/) specification

Heavily inspired by [gitmoji-cli](https://github.com/carloscuesta/gitmoji-cli)

## Install

using cargo

```bash
cargo install cc-cli
```

## Usage

```
cc-cli --help
```

```
Usage: cc-cli [<positionals...>] [-i] [-r]

Easy peasy Conventional Commits

Options:
  -i, --hook        hooks your commits
  -r, --unhook      removes the hook
  --help            display usage information
```

You can install the git-hook using

**NOTE:** the hook is installed in a per-repository basis

```
cc-cli -i
```

Or use it via cli with no parameters

```
cc-cli
```

To remote the hook, simply type

```
cc-cli -r
```
