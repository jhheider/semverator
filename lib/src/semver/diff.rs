use std::cmp::Ordering;

use super::Semver;

impl Semver {
    pub fn diff(&self, other: &Semver) -> Vec<isize> {
        let (high, low) = match self.cmp(other) {
            Ordering::Less => (other, self),
            _ => (self, other),
        };

        let mut diffs = Vec::with_capacity(high.components.len());
        let mut bumped = false;
        for (i, &hi) in high.components.iter().enumerate() {
            let lo = low.components.get(i).copied().unwrap_or(0);
            let d = if i == 0 || !bumped {
                hi as isize - lo as isize
            } else {
                hi as isize
            };
            diffs.push(d);
            bumped |= d > 0;
        }
        diffs
    }
}
