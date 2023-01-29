use anyhow::{Context, Result};

// use regex::Regex;

#[derive(Default, Debug, Clone)]
pub struct Semver<'a> {
    // todo!()
    pub components: Vec<usize>,

    pub major: usize,
    pub minor: usize,
    patch: usize,

    // todo!()
    _prerelease: Vec<&'a str>,
    _build: Vec<&'a str>,

    pub raw: String,
}

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

    pub fn infinty() -> Self {
        Self {
            major: usize::MAX,
            minor: usize::MAX,
            patch: usize::MAX,
            raw: "Infinity.Infinity.Infinty".to_string(),
            ..Default::default()
        }
    }
}

#[derive(PartialEq)]
enum Compare {
    Eq,
    Gt,
    Lt,
}
