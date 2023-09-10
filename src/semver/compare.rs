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

    fn compare(&self, other: &Semver) -> Ordering {
        let len = self.components.len().max(other.components.len());
        for x in 0..len {
            if x == 0 {
                match (
                    self.major > 1900 && self.major < usize::MAX,
                    other.major > 1900 && other.major < usize::MAX,
                ) {
                    (true, false) => return Ordering::Less,
                    (false, true) => return Ordering::Greater,
                    _ => (),
                }
            }
            let a = self.components.get(x);
            let b = other.components.get(x);
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
