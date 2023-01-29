use super::Semver;

impl<'a> Semver<'a> {
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
}

impl<'a> PartialEq for Semver<'a> {
    fn eq(&self, other: &Semver) -> bool {
        self.compare(other) == Compare::Eq
    }
}

#[derive(PartialEq)]
enum Compare {
    Eq,
    Gt,
    Lt,
}
