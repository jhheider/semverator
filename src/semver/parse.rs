use anyhow::{Context, Result};
use regex::Regex;

use super::Semver;

impl<'a> Semver<'a> {
    pub fn parse(semver: &str) -> Result<Self> {
        let re = Regex::new(r"(\d+)([a-z])")?;
        let raw = semver.to_string();
        let mut components = Vec::new();

        for r in raw.split('.') {
            match re.captures(r) {
                Some(caps) => {
                    components.push(caps.get(1).context("regex failure")?.as_str().parse()?);
                    components.push(
                        caps.get(2)
                            .context("regex failure")?
                            .as_str()
                            .chars()
                            .next()
                            .context("regex failure")? as usize
                            - 'a' as usize
                            + 1,
                    );
                }
                None => components.push(r.trim_start_matches('v').parse()?),
            }
        }
        let major = *components.first().context("string is too short")?;
        let minor = *components.get(1).unwrap_or(&0);
        let patch = *components.get(2).unwrap_or(&0);

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
