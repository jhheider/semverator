extern crate clap;

mod range;
mod semver;
#[cfg(test)]
mod tests;

use std::process::exit;

use anyhow::{Context, Result};
use clap::{arg, command, ArgMatches, Command};
use semver::Semver;

fn main() -> Result<()> {
    let matches = command!()
        .subcommand(
            Command::new("validate")
                .about("validates a version")
                .arg(arg!([semver] "the version to validate")),
        )
        .subcommand(
            Command::new("eq")
                .about("checks if two versions are equal")
                .arg(arg!([left] "the first version to compare"))
                .arg(arg!([right] "the second version to compare")),
        )
        .subcommand(
            Command::new("neq")
                .about("checks if two versions are not equal")
                .arg(arg!([left] "the first version to compare"))
                .arg(arg!([right] "the second version to compare")),
        )
        .subcommand(
            Command::new("gt")
                .about("checks if left > right")
                .arg(arg!([left] "the first version to compare"))
                .arg(arg!([right] "the second version to compare")),
        )
        .subcommand(
            Command::new("lt")
                .about("checks if left < right")
                .arg(arg!([left] "the first version to compare"))
                .arg(arg!([right] "the second version to compare")),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("validate", args)) => {
            let semver = get_arg::<Semver>(args, "semver")?;
            println!("{} is valid", semver.raw);
            Ok(())
        }
        Some(("eq", args)) => {
            let left = get_arg::<Semver>(args, "left")?;
            let right = get_arg::<Semver>(args, "right")?;

            if left.eq(&right) {
                println!("versions are equal");
                Ok(())
            } else {
                println!("versions are not equal");
                exit(1);
            }
        }
        Some(("neq", args)) => {
            let left = get_arg::<Semver>(args, "left")?;
            let right = get_arg::<Semver>(args, "right")?;

            if left.neq(&right) {
                println!("versions are not equal");
                Ok(())
            } else {
                println!("versions are equal");
                exit(1);
            }
        }
        Some(("gt", args)) => {
            let left = get_arg::<Semver>(args, "left")?;
            let right = get_arg::<Semver>(args, "right")?;

            if left.gt(&right) {
                println!("{} is greater than {}", left.raw, right.raw);
                Ok(())
            } else {
                println!("{} is not greater than {}", left.raw, right.raw);
                exit(1);
            }
        }
        Some(("lt", args)) => {
            let left = get_arg::<Semver>(args, "left")?;
            let right = get_arg::<Semver>(args, "right")?;

            if left.lt(&right) {
                println!("{} is less than {}", left.raw, right.raw);
                Ok(())
            } else {
                println!("{} is not less than {}", left.raw, right.raw);
                exit(1);
            }
        }

        Some((&_, _)) => todo!("what is this?"),
        None => {
            println!("no command supplied");
            exit(1)
        }
    }
}

pub trait Parseable<'a>: Sized {
    fn parse(input: &'a str) -> Result<Self>;
}

fn get_arg<'a, T>(args: &'a ArgMatches, key: &'a str) -> Result<T>
where
    T: Parseable<'a>,
{
    let s_val = args
        .get_one::<String>(key)
        .context(format!("{key} is missing"))?;
    T::parse(s_val).context(format!("?{s_val} is unparsable"))
}
