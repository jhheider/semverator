use crate::semver::Semver;

use super::Range;

impl<'a> Range<'a> {
    pub fn satisfies(&self, semver: &Semver) -> bool {
        self.set.iter().any(|c| c.satisfies(semver))
    }
}
