extern crate clap;

mod range;
mod semver;
#[cfg(test)]
mod tests;

use anyhow::{bail, Context, Result};
use clap::{arg, command, ArgAction, ArgMatches, Command};
use range::Range;
use semver::Semver;

fn main() -> Result<()> {
    let matches = command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        // Semver::validate
        .subcommand(
            Command::new("validate")
                .about("validates a version")
                .arg(arg!([semver] "the version to validate").value_parser(Semver::parse)),
        )
        // Semver::eq
        .subcommand(
            Command::new("eq")
                .about("checks if two versions are equal")
                .arg(arg!([left] "the first version to compare").value_parser(Semver::parse))
                .arg(arg!([right] "the second version to compare").value_parser(Semver::parse)),
        )
        // Semver::neq
        .subcommand(
            Command::new("neq")
                .about("checks if two versions are not equal")
                .arg(arg!([left] "the first version to compare").value_parser(Semver::parse))
                .arg(arg!([right] "the second version to compare").value_parser(Semver::parse)),
        )
        // Semver::gt
        .subcommand(
            Command::new("gt")
                .about("checks if left > right")
                .arg(arg!([left] "the first version to compare").value_parser(Semver::parse))
                .arg(arg!([right] "the second version to compare").value_parser(Semver::parse)),
        )
        // Semver::lt
        .subcommand(
            Command::new("lt")
                .about("checks if left < right")
                .arg(arg!([left] "the first version to compare").value_parser(Semver::parse))
                .arg(arg!([right] "the second version to compare").value_parser(Semver::parse)),
        )
        // Range::validate-range
        .subcommand(
            Command::new("validate-range")
                .about("validates a range")
                .arg(arg!([range] "the range to validate").value_parser(Range::parse)),
        )
        // Range::satisfies
        .subcommand(
            Command::new("satisfies")
                .about("validates a range satisfies a semver")
                .arg(arg!([range] "the range to validate").value_parser(Range::parse))
                .arg(arg!([semver] "the semver to test").value_parser(Semver::parse)),
        )
        // Range::max
        .subcommand(
            Command::new("max")
                .about("maximum version that satisifies a range")
                .arg(arg!([range] "the range to validate").value_parser(Range::parse))
                .arg(
                    arg!([semver] "the semvers to test")
                        .value_parser(Semver::parse)
                        .action(ArgAction::Append),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        // Semver::validate
        Some(("validate", args)) => {
            let semver = get_arg::<Semver>(args, "semver")?;
            println!("{} is valid", semver.raw);
            Ok(())
        }

        // Semver::eq
        Some(("eq", args)) => {
            let left = get_arg::<Semver>(args, "left")?;
            let right = get_arg::<Semver>(args, "right")?;

            if left.eq(&right) {
                println!("versions are equal");
                Ok(())
            } else {
                bail!("versions are not equal");
            }
        }

        // Semver::neq
        Some(("neq", args)) => {
            let left = get_arg::<Semver>(args, "left")?;
            let right = get_arg::<Semver>(args, "right")?;

            if left.neq(&right) {
                println!("versions are not equal");
                Ok(())
            } else {
                bail!("versions are equal");
            }
        }

        // Semver::gt
        Some(("gt", args)) => {
            let left = get_arg::<Semver>(args, "left")?;
            let right = get_arg::<Semver>(args, "right")?;

            if left.gt(&right) {
                println!("{} is greater than {}", left.raw, right.raw);
                Ok(())
            } else {
                bail!("{} is not greater than {}", left.raw, right.raw);
            }
        }

        // Semver::lt
        Some(("lt", args)) => {
            let left = get_arg::<Semver>(args, "left")?;
            let right = get_arg::<Semver>(args, "right")?;

            if left.lt(&right) {
                println!("{} is less than {}", left.raw, right.raw);
                Ok(())
            } else {
                bail!("{} is not less than {}", left.raw, right.raw);
            }
        }

        // Range::validate
        Some(("validate-range", args)) => {
            let range = get_arg::<Range>(args, "range")?;
            println!("{} is valid", range.raw);
            Ok(())
        }

        // Range::satisfies
        Some(("satisfies", args)) => {
            let range = get_arg::<Range>(args, "range")?;
            let semver = get_arg::<Semver>(args, "semver")?;
            if range.satisfies(&semver) {
                println!("{} satisifes {}", semver.raw, range.raw);
                Ok(())
            } else {
                bail!("{} doesn't satisify {}", semver.raw, range.raw);
            }
        }

        // Range::max
        Some(("max", args)) => {
            let range = get_arg::<Range>(args, "range")?;
            let semvers = args
                .get_many::<Semver>("semver")
                .context("no [semvers] were passed")?
                .cloned()
                .collect::<Vec<Semver>>();
            match range.max(&semvers) {
                Some(semver) => {
                    println!("{}", semver.raw);
                    Ok(())
                }
                None => bail!("no viable candidates"),
            }
        }

        Some((&_, _)) => todo!("what is this?"),
        None => bail!("no command supplied"),
    }
}

pub trait Parseable<'a>: Sized {
    fn parse(input: &'a str) -> Result<Self>;
}

fn get_arg<'a, T>(args: &'a ArgMatches, key: &'a str) -> Result<T>
where
    T: Clone + Send + Sync + 'static,
{
    args.get_one::<T>(key)
        .context(format!("{key} is missing"))
        .cloned()
}
