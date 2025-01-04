use std::fmt;

pub mod bump;
pub mod compare;
pub mod parse;

#[derive(Default, Debug, Clone, Eq)]
pub struct Semver {
    pub components: Vec<usize>,

    pub major: usize,
    pub minor: usize,
    pub patch: usize,

    pub prerelease: Vec<String>,
    pub build: Vec<String>,

    pub raw: String,
}

impl Semver {
    pub fn infinty() -> Self {
        Self {
            components: vec![usize::MAX, usize::MAX, usize::MAX],
            major: usize::MAX,
            minor: usize::MAX,
            patch: usize::MAX,
            raw: "Infinity.Infinity.Infinity".to_string(),
            ..Default::default()
        }
    }
}

impl fmt::Display for Semver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.components
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(".")
        )?;
        if !self.prerelease.is_empty() {
            write!(
                f,
                "-{}",
                self.prerelease
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(".")
            )?;
        }
        if !self.build.is_empty() {
            write!(
                f,
                "+{}",
                self.build
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(".")
            )?;
        }
        Ok(())
    }
}
