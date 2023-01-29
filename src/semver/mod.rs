use anyhow::{Context, Result};

use crate::Parseable;
// use regex::Regex;

#[derive(Default, Debug)]
pub struct Semver<'a> {
    // todo!()
    _components: Vec<usize>,

    major: usize,
    minor: usize,
    patch: usize,

    // todo!()
    _prerelease: Vec<&'a str>,
    _build: Vec<&'a str>,

    pub raw: &'a str,
}

impl<'a> Parseable<'a> for Semver<'a> {
    fn parse(semver: &'a str) -> Result<Self> {
        // let re = Regex::new(r"\d+[a-z]?")?;
        let raw = semver;
        let mut parts = semver.split('.');
        let major = parts
            .next()
            .context("string is too short")?
            .trim_start_matches('v')
            .parse()
            .context("major is not a digit")?;
        let minor = parts
            .next()
            .unwrap_or("0")
            .parse()
            .context("minor is not a digit")?;
        let patch = parts
            .next()
            .unwrap_or("0")
            .parse()
            .context("patch is not a digit")?;

        Ok(Self {
            major,
            minor,
            patch,
            raw,
            ..Default::default()
        })
    }
}

impl<'a> Semver<'a> {
    pub fn eq(&self, other: &Semver) -> bool {
        self.compare(other) == Compare::Eq
    }

    pub fn neq(&self, other: &Semver) -> bool {
        self.compare(other) != Compare::Eq
    }

    pub fn gt(&self, other: &Semver) -> bool {
        self.compare(other) == Compare::Gt
    }

    pub fn lt(&self, other: &Semver) -> bool {
        self.compare(other) == Compare::Lt
    }

    fn compare(&self, other: &Semver) -> Compare {
        match (
            self.major as isize - other.major as isize,
            self.minor as isize - other.minor as isize,
            self.patch as isize - other.patch as isize,
        ) {
            (0, 0, 0) => Compare::Eq,
            (a, _, _) if a > 0 => Compare::Gt,
            (a, _, _) if a < 0 => Compare::Lt,
            (_, b, _) if b > 0 => Compare::Gt,
            (_, b, _) if b < 0 => Compare::Lt,
            (_, _, c) if c > 0 => Compare::Gt,
            (_, _, c) if c < 0 => Compare::Lt,
            _ => unreachable!("invalid comparison"),
        }
    }
}

#[derive(PartialEq)]
enum Compare {
    Eq,
    Gt,
    Lt,
}
