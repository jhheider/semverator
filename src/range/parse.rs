use crate::semver::Semver;

use super::{Constraint, Range};
use anyhow::{bail, Context, Result};
use regex::Regex;

impl<'a> Range<'a> {
    pub fn parse(range: &str) -> Result<Self> {
        let raw = range.to_string();
        let mut set = Vec::new();

        if range.is_empty() {
            bail!("no constraints")
        }

        if range == "*" {
            set.push(Constraint::Any);
            return Ok(Self { raw, set });
        }
        let re = Regex::new(r"\s*(,|\|\|)\s*")?;
        set = re
            .split(range)
            .map(Constraint::parse)
            .collect::<Result<Vec<Constraint>>>()?;

        for c in set.iter() {
            if let Constraint::Contiguous(v1, v2) = c {
                if !v1.lt(v2) {
                    bail!("{} is greater than {}", v1.raw, v2.raw)
                }
            }
        }
        Ok(Self { raw, set })
    }
}

impl<'a> Constraint<'a> {
    fn parse(constraint: &str) -> Result<Self> {
        let re = Regex::new(r"^>=((\d+\.)*\d+)\s*(<((\d+\.)*\d+))?$")?;
        if let Some(cap) = re.captures(constraint) {
            let v1 = Semver::parse(cap.get(1).context("invalid description")?.as_str())?;
            let v2 = if cap.get(3).is_some() {
                Semver::parse(cap.get(4).context("invalid description")?.as_str())?
            } else {
                Semver::infinty()
            };
            return Ok(Constraint::Contiguous(v1, v2));
        }

        let re = Regex::new(r"^([~=<^])(.+)$")?;
        if let Some(cap) = re.captures(constraint) {
            return match cap.get(1).context("invalid character")?.as_str() {
                "^" => {
                    let v1 = Semver::parse(cap.get(2).context("invalid description")?.as_str())?;
                    if v1.major > 0 {
                        let v2 = Semver::parse(&format!("{}", v1.major + 1))?;
                        return Ok(Constraint::Contiguous(v1, v2));
                    } else if v1.minor > 0 {
                        let v2 = Semver::parse(&format!("{}.{}", v1.major, v1.minor + 1))?;
                        return Ok(Constraint::Contiguous(v1, v2));
                    } else {
                        return Ok(Constraint::Single(v1));
                    }
                }
                "~" => {
                    let v1 = Semver::parse(cap.get(2).context("invalid description")?.as_str())?;

                    let v2 = if v1.components.len() == 1 {
                        Semver::parse(&format!("{}", v1.major + 1))?
                    } else {
                        Semver::parse(&format!("{}.{}", v1.major, v1.minor + 1))?
                    };
                    Ok(Constraint::Contiguous(v1, v2))
                }
                "<" => {
                    let v1 = Semver::parse("0")?;
                    let v2 = Semver::parse(cap.get(2).context("invalid description")?.as_str())?;
                    Ok(Constraint::Contiguous(v1, v2))
                }
                "=" => Ok(Constraint::Single(Semver::parse(
                    cap.get(2).context("invalid description")?.as_str(),
                )?)),
                _ => unreachable!("invalid range description"),
            };
        }
        bail!("invalid range description")
    }
}
