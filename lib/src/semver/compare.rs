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

    /// checks @ syntax, eg. node@22.1
    pub fn at(&self, at: &Semver) -> bool {
        let mut cc1 = self.components.clone();
        let cc2 = &at.components;

        // helper function to get the last element of a slice
        fn last(arr: &[usize]) -> usize {
            *arr.last().unwrap()
        }

        if cc1.len() > cc2.len() {
            return false;
        }

        // Ensure cc1 and cc2 have the same length by appending 0s to cc1
        while cc1.len() < cc2.len() {
            cc1.push(0);
        }

        if last(&cc1) + 1 != last(cc2) {
            return false;
        }

        for i in 0..cc1.len() - 1 {
            if cc1[i] != cc2[i] {
                return false;
            }
        }

        true
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
