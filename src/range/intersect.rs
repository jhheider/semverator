use std::collections::HashSet;

use super::{Constraint, Range};
use anyhow::{bail, Result};

impl<'a> Range<'a> {
    pub fn intersect(&self, range: &'a Range) -> Result<Range> {
        let a = self;
        let b = range;

        let mut set = HashSet::<Constraint>::new();

        for aa in a.set.iter() {
            for bb in b.set.iter() {
                match (aa, bb) {
                    (Constraint::Any, _) => return Range::parse(&b.raw),
                    (_, Constraint::Any) => return Range::parse(&a.raw),
                    (Constraint::Single(aaa), Constraint::Single(bbb)) => {
                        if aaa.eq(bbb) {
                            set.insert(aa.clone());
                        }
                    }
                    (Constraint::Single(aaa), Constraint::Contiguous(_, _)) => {
                        if bb.satisfies(aaa) {
                            set.insert(aa.clone());
                        }
                    }
                    (Constraint::Contiguous(_, _), Constraint::Single(bbb)) => {
                        if aa.satisfies(bbb) {
                            set.insert(bb.clone());
                        }
                    }
                    (Constraint::Contiguous(a1, a2), Constraint::Contiguous(b1, b2)) => {
                        if a1.lt(b2) && b1.lt(a2) {
                            let v1 = if a1.gt(b1) { a1 } else { b1 };
                            let v2 = if a2.lt(b2) { a2 } else { b2 };
                            set.insert(Constraint::Contiguous(v1.clone(), v2.clone()));
                        }
                    }
                }
            }
        }
        if set.is_empty() {
            bail!("no intersection possible")
        }
        let mut rv = Range {
            raw: "".to_string(),
            set,
        };
        rv.raw = rv.raw();
        Ok(rv)
    }
}
