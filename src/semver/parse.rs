use anyhow::{Context, Result};

use super::Semver;

impl<'a> Semver<'a> {
    pub fn parse(semver: &str) -> Result<Self> {
        // let re = Regex::new(r"\d+[a-z]?")?;
        let raw = semver.to_string();
        let mut components = Vec::new();
        let mut parts = raw.split('.');
        let major = parts
            .next()
            .context("string is too short")?
            .trim_start_matches('v')
            .parse()
            .context("major is not a digit")?;
        components.push(major);
        let minor = if let Some(p) = parts.next() {
            let v = p.parse().context("minor is not a digit")?;
            components.push(v);
            v
        } else {
            0
        };
        let patch = if let Some(p) = parts.next() {
            let v = p.parse().context("patch is not a digit")?;
            components.push(v);
            v
        } else {
            0
        };

        Ok(Self {
            components,
            major,
            minor,
            patch,
            raw,
            ..Default::default()
        })
    }
}
