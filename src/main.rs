extern crate clap;

mod args;
mod range;
mod semver;
#[cfg(test)]
mod tests;

use anyhow::{bail, Result};
use args::{get_arg, get_arg_vec};
use clap::ArgMatches;
use range::Range;
use semver::Semver;

fn main() -> Result<()> {
    let args = args::setup().get_matches();

    handle_command(args.subcommand())
}

// `clap` tested for correctness
// TODO: factor out as much as possible for testing
#[cfg(not(tarpaulin_include))]
fn handle_command(matches: Option<(&str, &ArgMatches)>) -> Result<()> {
    match matches {
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
            let semvers = get_arg_vec::<Semver>(args, "semver")?;
            match range.max(&semvers) {
                Some(semver) => {
                    println!("{}", semver.raw);
                    Ok(())
                }
                None => bail!("no viable candidates"),
            }
        }

        // Range::intersect
        Some(("intersect", args)) => {
            let left = get_arg::<Range>(args, "left")?;
            let right = get_arg::<Range>(args, "right")?;

            let intersection = left.intersect(&right)?;
            println!("{}", intersection.raw);
            Ok(())
        }

        Some((cmd, _)) => unimplemented!("{cmd} isn't implemented"),
        None => bail!("no command supplied"),
    }
}
