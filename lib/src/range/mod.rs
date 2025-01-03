use crate::semver::Semver;
use std::{
    fmt,
    hash::{Hash, Hasher},
};

pub mod intersect;
pub mod max;
pub mod parse;
pub mod satisfies;

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

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = self
            .set
            .iter()
            .map(|v| match v {
                Constraint::Any => "*".to_string(),
                Constraint::Single(v) => format!("={}", v),
                Constraint::Contiguous(v1, v2) => {
                    if v2.major == v1.major + 1 && v2.minor == 0 && v2.patch == 0 {
                        let v = chomp(v1);
                        if v1.major == 0 {
                            if v1.components.len() == 1 {
                                "^0".to_string()
                            } else {
                                format!(">={}<1", v)
                            }
                        } else {
                            format!("^{}", v)
                        }
                    } else if v2.major == v1.major && v2.minor == v1.minor + 1 && v2.patch == 0 {
                        let v = chomp(v1);
                        format!("~{}", v)
                    } else if v2.major == usize::MAX {
                        let v = chomp(v1);
                        format!(">={}", v)
                    } else if at(v1, v2) {
                        format!("@{}", v1)
                    } else {
                        format!(">={}<{}", chomp(v1), chomp(v2))
                    }
                }
            })
            .collect::<Vec<_>>()
            .join(",");
        write!(f, "{}", str)
    }
}

/// checks @ syntax, eg. node@22.1
fn at(v1: &Semver, v2: &Semver) -> bool {
    let mut cc1 = v1.components.clone();
    let cc2 = &v2.components;

    if cc1.len() > cc2.len() {
        return false;
    }

    // Ensure cc1 and cc2 have the same length by appending 0s to cc1
    while cc1.len() < cc2.len() {
        cc1.push(0);
    }

    if last(&cc1) != last(cc2) - 1 {
        return false;
    }

    for i in 0..cc1.len() - 1 {
        if cc1[i] != cc2[i] {
            return false;
        }
    }

    true
}

fn last(arr: &[usize]) -> usize {
    *arr.last().unwrap()
}

fn chomp(v: &Semver) -> String {
    let result = v.raw.trim_end_matches(".0");
    if result.is_empty() {
        "0".to_string()
    } else {
        result.to_string()
    }
}
