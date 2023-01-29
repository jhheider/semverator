use std::collections::HashSet;

use crate::semver::Semver;

mod intersect;
mod max;
mod parse;
mod satisfies;

#[derive(Debug, Clone)]
pub struct Range<'a> {
    pub raw: String,
    pub set: HashSet<Constraint<'a>>,
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub enum Constraint<'a> {
    Any,
    Single(Semver<'a>),
    Contiguous(Semver<'a>, Semver<'a>),
}

impl<'a> Eq for Constraint<'a> {}

impl<'a> Constraint<'a> {
    pub fn raw(&self) -> String {
        match self {
            Constraint::Any => "*".to_string(),
            Constraint::Single(v) => format!("={}", v.raw),
            Constraint::Contiguous(v1, v2) => format!(">={}<{}", v1.raw, v2.raw),
        }
    }
}

impl<'a> Range<'a> {
    pub fn raw(&self) -> String {
        let rv = self.set.iter().map(|c| c.raw()).collect::<Vec<String>>();
        rv.join(",")
    }
}
