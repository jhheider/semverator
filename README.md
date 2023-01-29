![GitHub release (latest by date)](https://img.shields.io/github/v/release/jhheider/semverator)
[![Test with Code Coverage](https://github.com/jhheider/semverator/actions/workflows/test.yml/badge.svg)](https://github.com/jhheider/semverator/actions/workflows/test.yml)
[![Check and Lint](https://github.com/jhheider/semverator/actions/workflows/check-and-lint.yaml/badge.svg)](https://github.com/jhheider/semverator/actions/workflows/check-and-lint.yaml)
[![Coverage Status](https://coveralls.io/repos/github/jhheider/semverator/badge.svg?branch=main)](https://coveralls.io/github/jhheider/semverator?branch=main)

# semverator

Pure rust implementation of [teaxyz/cli:semver.ts](https://github.com/teaxyz/cli/blob/main/src/utils/semver.ts) for command-line usage.

## Install

`cargo install semverator` or, for [tea](https://tea.xyz) users, `tea +crates.io/semverator true`.

## Usage

Well, thanks to [clap](https://github.com/clap-rs/clap), the help system flows nicely from the implementation:

```sh
A command line tool for working with semantic versioning (tea.xyz implementation)

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

## TODO

- [x] validate semver
- [x] include letter-extend versions (openssl-1.1.1s)
- [x] semver comparisons
- [x] constraint ranges
- [x] range intersections
- [x] 90+% test coverage
