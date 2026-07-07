# semverator

[![CI](https://github.com/jhheider/semverator/actions/workflows/ci.yml/badge.svg)](https://github.com/jhheider/semverator/actions/workflows/ci.yml)
[![Coverage Status](https://coveralls.io/repos/github/jhheider/semverator/badge.svg?branch=main)](https://coveralls.io/github/jhheider/semverator?branch=main)
[![crates.io](https://img.shields.io/crates/v/semverator.svg)](https://crates.io/crates/semverator)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE.md)

Pure rust implementation of [libpkgx:utils/semver.ts](https://github.com/pkgxdev/libpkgx/blob/main/src/utils/semver.ts)
for command-line usage.

## Install

`semverator` is available four ways:

- **Homebrew:** `brew install jhheider/tap/semverator`
- **Cargo:** `cargo install semverator`
- **pkgx:** `pkgx +crates.io/semverator`
- **Source:** `cargo install --path cli`

## Usage

Well, thanks to [clap](https://github.com/clap-rs/clap), the help system flows
nicely from the implementation:

```sh
A command line tool for working with semantic versioning (libpkgx implementation)

Usage: semverator <COMMAND>

Commands:
  validate        validates a version
  eq              checks if two versions are equal
  neq             checks if two versions are not equal
  gt              checks if left > right
  lt              checks if left < right
  validate-range  validates a range
  satisfies       validates a range satisfies a semver
  max             maximum version that satisifies a range
  intersect       intersection between two ranges
  help            Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
