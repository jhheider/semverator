use crate::semver::Semver;

mod intersect;
mod max;
mod parse;
mod satisfies;

#[derive(Debug, Clone)]
pub struct Range<'a> {
    pub raw: String,
    pub set: Vec<Constraint<'a>>,
}

#[derive(Debug, Clone)]
pub enum Constraint<'a> {
    Any,
    Single(Semver<'a>),
    Contiguous(Semver<'a>, Semver<'a>),
}
