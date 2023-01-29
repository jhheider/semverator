use crate::{range::Range, semver::Semver};
use anyhow::{Context, Result};
use clap::{arg, command, ArgAction, ArgMatches, Command};

pub fn setup() -> ArgMatches {
    command!()
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
        .get_matches()
}

pub fn get_arg<'a, T>(args: &'a ArgMatches, key: &'a str) -> Result<T>
where
    T: Clone + Send + Sync + 'static,
{
    args.get_one::<T>(key)
        .context(format!("{key} is missing"))
        .cloned()
}

pub fn get_arg_vec<'a, T>(args: &'a ArgMatches, key: &'a str) -> Result<Vec<T>>
where
    T: Clone + Send + Sync + 'static,
{
    Ok(args
        .get_many::<T>(key)
        .context(format!("no {key}s were passed"))?
        .cloned()
        .collect::<Vec<T>>())
}
