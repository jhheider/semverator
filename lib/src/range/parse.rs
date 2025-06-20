use crate::semver::Semver;

use super::{Constraint, Range};
use anyhow::{bail, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RANGE_REGEX: Regex = Regex::new(r"\s*(,|\|\|)\s*").unwrap();
    static ref CONSTRAINT_REGEX_RANGE: Regex =
        Regex::new(r"^>=((\d+\.)*\d+)\s*(<((\d+\.)*\d+))?$").unwrap();
    static ref CONSTRAINT_REGEX_SIMPLE: Regex = Regex::new(r"^([~=<^@])(.+)$").unwrap();
    static ref INFINITIES_REGEX: Regex = Regex::new(r"<Infinity(\.Infinity)+").unwrap();
}

impl Range {
    pub fn parse(range: &str) -> Result<Self> {
        // Ignore `<Infinity.Infinity.Infinity`. Fixes https://github.com/pkgxdev/pkgx/issues/1190
        let range = if INFINITIES_REGEX.is_match(range) {
            &INFINITIES_REGEX.replace(range, "").to_string()
        } else {
            range
        };
        let raw = range.to_string();
        let mut set = Vec::new();

        if range.is_empty() {
            bail!("no constraints")
        }

        if range == "*" {
            set.push(Constraint::Any);
            return Ok(Self { raw, set });
        }
        set = RANGE_REGEX
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

    pub fn any() -> Self {
        Self {
            raw: "*".to_string(),
            set: vec![Constraint::Any],
        }
    }

    pub fn single(v: &str) -> Result<Self> {
        let raw = format!("={v}");
        let set = vec![Constraint::Single(
            Semver::parse(v).context("invalid version")?,
        )];
        Ok(Self { raw, set })
    }

    pub fn contiguous(v1: &str, v2: &str) -> Result<Self> {
        let raw = format!(">={v1}<{v2}");
        let set = vec![Constraint::Contiguous(
            Semver::parse(v1).context("invalid version")?,
            Semver::parse(v2).context("invalid version")?,
        )];
        Ok(Self { raw, set })
    }

    pub fn caret(v: &str) -> Result<Self> {
        let raw = format!("^{v}");
        let set = vec![Constraint::parse(&raw)?];
        Ok(Self { raw, set })
    }

    pub fn tilde(v: &str) -> Result<Self> {
        let raw = format!("~{v}");
        let set = vec![Constraint::parse(&raw)?];
        Ok(Self { raw, set })
    }

    pub fn from_semver(v: &Semver) -> Result<Self> {
        Self::single(&v.raw)
    }
}

impl Constraint {
    pub fn parse(constraint: &str) -> Result<Self> {
        if let Some(cap) = CONSTRAINT_REGEX_RANGE.captures(constraint) {
            let v1 = Semver::parse(cap.get(1).context("invalid description")?.as_str())?;
            let v2 = if cap.get(3).is_some() {
                Semver::parse(cap.get(4).context("invalid description")?.as_str())?
            } else {
                Semver::infinty()
            };
            return Ok(Constraint::Contiguous(v1, v2));
        }

        // ^0 is a special case, in that it doesn't work like
        // ^0.x or ^0.x.y, but rather like any other ^x
        if constraint == "^0" {
            return Ok(Constraint::Contiguous(
                Semver::parse("0.0.0")?,
                Semver::parse("1.0.0")?,
            ));
        }

        if let Some(cap) = CONSTRAINT_REGEX_SIMPLE.captures(constraint) {
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
                "@" => {
                    let v1 = Semver::parse(cap.get(2).context("invalid description")?.as_str())?;
                    let mut parts = v1.components.clone();
                    let last = parts.last_mut().context("version too short")?;
                    *last += 1;
                    let v2 = Semver::parse(
                        &parts
                            .iter()
                            .map(|c| c.to_string())
                            .collect::<Vec<_>>()
                            .join("."),
                    )?;
                    Ok(Constraint::Contiguous(v1, v2))
                }
                "=" => Ok(Constraint::Single(Semver::parse(
                    cap.get(2).context("invalid description")?.as_str(),
                )?)),
                _ => unreachable!("invalid range description: {}", constraint),
            };
        }
        bail!("invalid range description: {}", constraint)
    }
}
