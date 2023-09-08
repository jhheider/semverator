use crate::semver::Semver;

use super::{Constraint, Range};

impl Range {
    pub fn max(&self, semvers: &[Semver]) -> Option<Semver> {
        let rv = semvers.iter().filter(|s| self.satisfies(s)).fold(
            Semver::parse("0").unwrap(),
            |max, s| {
                if max.gt(s) {
                    max
                } else {
                    s.clone()
                }
            },
        );
        let zero = Semver::parse("0").unwrap();
        if rv.eq(&zero) {
            return None;
        }
        Some(Semver::parse(&rv.raw).unwrap())
    }
}

impl Constraint {
    pub fn satisfies(&self, semver: &Semver) -> bool {
        match self {
            Constraint::Any => true,
            Constraint::Single(v) => v.eq(semver),
            Constraint::Contiguous(v1, v2) => v1.eq(semver) || (v1.lt(semver) && v2.gt(semver)),
        }
    }
}
