use super::Semver;

impl Semver {
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
        let len = self.components.len().max(other.components.len());
        for x in 0..len {
            let a = self.components.get(x);
            let b = other.components.get(x);
            match (a, b) {
                (None, _) => return Compare::Lt,
                (_, None) => return Compare::Gt,
                (Some(a), Some(b)) if a > b => return Compare::Gt,
                (Some(a), Some(b)) if a < b => return Compare::Lt,
                _ => continue,
            }
        }
        Compare::Eq
    }
}

impl PartialEq for Semver {
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
