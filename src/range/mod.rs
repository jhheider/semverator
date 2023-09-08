use crate::semver::Semver;
use std::hash::{Hash, Hasher};

mod intersect;
mod max;
mod parse;
mod satisfies;

#[derive(Debug, Clone)]
pub struct Range {
    pub raw: String,
    pub set: Vec<Constraint>,
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub enum Constraint {
    Any,
    Single(Semver),
    Contiguous(Semver, Semver),
}

impl Eq for Constraint {}

impl Constraint {
    pub fn raw(&self) -> String {
        match self {
            Constraint::Any => "*".to_string(),
            Constraint::Single(v) => format!("={}", v.raw),
            Constraint::Contiguous(v1, v2) => format!(">={}<{}", v1.raw, v2.raw),
        }
    }
}

impl Range {
    pub fn raw(&self) -> String {
        let rv = self.set.iter().map(|c| c.raw()).collect::<Vec<String>>();
        rv.join(",")
    }
}

impl Hash for Semver {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.components.hash(state);
    }
}
