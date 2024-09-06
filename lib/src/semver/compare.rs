use super::Semver;
use std::cmp::Ordering;

impl Semver {
    pub fn neq(&self, other: &Semver) -> bool {
        self.compare(other) != Ordering::Equal
    }

    pub fn gt(&self, other: &Semver) -> bool {
        self.compare(other) == Ordering::Greater
    }

    pub fn lt(&self, other: &Semver) -> bool {
        self.compare(other) == Ordering::Less
    }

    // Treat majors >1996 as calver, and less than 0.0.0.
    fn handle_calver(&self) -> Vec<usize> {
        if self.major < 1996 || self.major == usize::MAX {
            self.components.clone()
        } else {
            let mut cmps = vec![0, 0, 0];
            cmps.extend(self.components.iter().cloned());
            cmps
        }
    }

    fn compare(&self, other: &Semver) -> Ordering {
        let acmps = self.handle_calver();
        let bcmps = other.handle_calver();

        let len = acmps.len().max(bcmps.len());
        for x in 0..len {
            let a = acmps.get(x);
            let b = bcmps.get(x);
            match (a, b) {
                (None, _) => return Ordering::Less,
                (_, None) => return Ordering::Greater,
                (Some(a), Some(b)) if a > b => return Ordering::Greater,
                (Some(a), Some(b)) if a < b => return Ordering::Less,
                _ => continue,
            }
        }

        // Special case: all prerelease versions are less than no prerelease
        if self.prerelease.is_empty() && !other.prerelease.is_empty() {
            return Ordering::Greater;
        } else if !self.prerelease.is_empty() && other.prerelease.is_empty() {
            return Ordering::Less;
        }

        let len = self.prerelease.len().max(other.prerelease.len());
        for x in 0..len {
            let a = self.prerelease.get(x);
            let b = other.prerelease.get(x);
            match (a, b) {
                (None, _) => return Ordering::Less,
                (_, None) => return Ordering::Greater,
                (Some(a), Some(b)) if a > b => return Ordering::Greater,
                (Some(a), Some(b)) if a < b => return Ordering::Less,
                _ => continue,
            }
        }

        let len = self.build.len().max(other.build.len());
        for x in 0..len {
            let a = self.build.get(x);
            let b = other.build.get(x);
            match (a, b) {
                (None, _) => return Ordering::Less,
                (_, None) => return Ordering::Greater,
                (Some(a), Some(b)) if a > b => return Ordering::Greater,
                (Some(a), Some(b)) if a < b => return Ordering::Less,
                _ => continue,
            }
        }

        Ordering::Equal
    }
}

impl PartialEq for Semver {
    fn eq(&self, other: &Semver) -> bool {
        self.compare(other) == Ordering::Equal
    }
}

impl Ord for Semver {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare(other)
    }
}

impl PartialOrd for Semver {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
