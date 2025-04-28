use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

use super::Semver;

lazy_static! {
    static ref FULL_REGEX: Regex =
        Regex::new(r"^(\d+(?:\.\d+)*)([a-z])?(?:-([0-9A-Za-z-\.]+))?(?:\+([0-9A-Za-z-\.]+))?$")
            .unwrap();
    static ref SHORT_REGEX: Regex = Regex::new(r"^\d+(\.\d+)?([-\+].*)?$").unwrap();
}

impl Semver {
    pub fn parse(semver: &str) -> Result<Self> {
        let raw = semver.trim_start_matches('v').to_string();

        let captures = FULL_REGEX.captures(&raw).context("invalid semver")?;

        let mut components: Vec<usize> = captures
            .get(1)
            .context("regex failure")?
            .as_str()
            .split('.')
            .map(|s| s.parse::<usize>().context("invalid digit"))
            .collect::<Result<Vec<usize>>>()?;

        if let Some(letter) = captures.get(2) {
            let letter = letter.as_str().chars().next().context("not a character")? as usize
                - 'a' as usize
                + 1;
            components.push(letter);
        }

        let major = *components.first().context("string is too short")?;
        let minor = *components.get(1).unwrap_or(&0);
        let patch = *components.get(2).unwrap_or(&0);

        let prerelease = if let Some(pr) = captures.get(3) {
            pr.as_str().split('.').map(|s| s.to_string()).collect()
        } else {
            vec![]
        };

        let build = if let Some(b) = captures.get(4) {
            b.as_str().split('.').map(|s| s.to_string()).collect()
        } else {
            vec![]
        };

        let raw = if SHORT_REGEX.is_match(&raw) {
            let mut r = format!("{major}.{minor}.{patch}");
            if !prerelease.is_empty() {
                r.push_str(&format!("-{}", prerelease.join(".")));
            }
            if !build.is_empty() {
                r.push_str(&format!("+{}", build.join(".")));
            }
            r
        } else {
            raw
        };

        Ok(Self {
            components,
            major,
            minor,
            patch,
            prerelease,
            build,
            raw,
        })
    }

    pub fn from(input: (usize, usize, usize)) -> Result<Self> {
        Self::parse(&format!("{}.{}.{}", input.0, input.1, input.2))
    }
}
