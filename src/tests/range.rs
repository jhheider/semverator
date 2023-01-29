use anyhow::Result;

use crate::{range::Range, semver::Semver};

#[test]
fn test_parse() -> Result<()> {
    let a = Range::parse(">=11<15");
    let b = Range::parse("^16");
    let c = Range::parse("~16");
    let d = Range::parse("=16");
    let e = Range::parse("<16");
    let f = Range::parse(">=11<15||^16.5||~16||=16||<16");
    let g = Range::parse("*");
    let h = Range::parse(">=15<12");
    let i = Range::parse("<=11>15");
    let j = Range::parse("Your mom");
    let k = Range::parse("");
    let l = Range::parse(">=12");

    assert!(a.is_ok());
    assert!(b.is_ok());
    assert!(c.is_ok());
    assert!(d.is_ok());
    assert!(e.is_ok());
    assert!(f.is_ok());
    assert!(g.is_ok());
    assert!(h.is_err());
    assert!(i.is_err());
    assert!(j.is_err());
    assert!(k.is_err());
    assert!(l.is_ok());

    assert_eq!(f?.set.len(), 5);

    Ok(())
}

#[test]
fn test_raw() -> Result<()> {
    let ra = Range::parse("*")?;

    assert_eq!(ra.raw(), "*");

    Ok(())
}

#[test]
fn test_satisfies() -> Result<()> {
    let ra = Range::parse(">=11<14")?;

    let sa = Semver::parse("10.5")?;
    let sb = Semver::parse("11.5")?;
    let sc = Semver::parse("13.5")?;
    let sd = Semver::parse("15.5")?;

    assert!(!ra.satisfies(&sa));
    assert!(ra.satisfies(&sb));
    assert!(ra.satisfies(&sc));
    assert!(!ra.satisfies(&sd));

    let rb = Range::parse("^11")?;

    assert!(!rb.satisfies(&sa));
    assert!(rb.satisfies(&sb));
    assert!(!rb.satisfies(&sc));
    assert!(!rb.satisfies(&sd));

    let rc = Range::parse("^11.6")?;

    assert!(!rc.satisfies(&sb));

    let rd = Range::parse("=11.5")?;

    assert!(!rd.satisfies(&sa));
    assert!(rd.satisfies(&sb));

    Ok(())
}

#[test]
fn test_max() -> Result<()> {
    let ra = Range::parse("*")?;
    let rb = Range::parse(">=11<15")?;
    let rc = Range::parse("^16.5")?;
    let rd = Range::parse("^3")?;

    let sa = vec![
        Semver::parse("10.5")?,
        Semver::parse("11.5")?,
        Semver::parse("12.5")?,
        Semver::parse("13.5")?,
        Semver::parse("14.5")?,
        Semver::parse("15.5")?,
        Semver::parse("16.3")?,
        Semver::parse("16.5")?,
        Semver::parse("16.8")?,
        Semver::parse("17.8")?,
        Semver::parse("4.8")?,
    ];

    assert_eq!(ra.max(&sa).unwrap().raw, "17.8");
    assert_eq!(rb.max(&sa).unwrap().raw, "14.5");
    assert_eq!(rc.max(&sa).unwrap().raw, "16.8");
    assert!(rd.max(&sa).is_none());
    Ok(())
}

#[test]
fn test_intersect() -> Result<()> {
    let ra = Range::parse("^3.7")?;
    let rb = Range::parse("=3.11")?;

    let ia = ra.intersect(&rb);
    assert!(ia.is_ok());
    assert_eq!(ia?.raw, rb.raw);

    let rc = Range::parse("^3.9")?;

    let ib = ra.intersect(&rc);
    assert!(ib.is_ok());
    assert_eq!(ib?.raw, ">=3.9<4");

    let rd = Range::parse("*")?;

    let ic = ra.intersect(&rd);
    assert!(ic.is_ok());
    assert_eq!(ic?.raw, "^3.7");

    let re = Range::parse("~3.7")?;
    let rf = Range::parse("~3.8")?;

    let id = re.intersect(&rf);
    assert!(id.is_err());

    let rg = Range::parse("=3.8")?;

    let ie = ra.intersect(&rg);
    assert!(ie.is_ok());
    assert_eq!(ie?.raw, rg.raw);

    let rh = Range::parse("^11,^12")?;
    let ri = Range::parse("^11.3")?;

    let r#if = rh.intersect(&ri);
    assert!(r#if.is_ok());
    assert_eq!(r#if?.raw, ">=11.3<12");

    let rj = Range::parse("^11.3,^12.2")?;

    let ig = rh.intersect(&rj);
    assert!(ig.is_ok());
    assert_eq!(ig?.raw, ">=11.3<12,>=12.2<13");

    let rk = Range::parse("*")?;

    let ih = rk.intersect(&ra);
    assert!(ih.is_ok());
    assert_eq!(ih?.raw, ra.raw);

    let ii = ra.intersect(&rk);
    assert!(ii.is_ok());
    assert_eq!(ii?.raw, ra.raw);

    let rl = Range::parse("=3.8")?;
    let rm = Range::parse("=3.9")?;

    let ij = rg.intersect(&rl);
    let ik = rg.intersect(&rm);

    assert!(ij.is_ok());
    assert!(ik.is_err());
    assert_eq!(ij?.raw, rl.raw);

    let il = rg.intersect(&ra);
    assert!(il.is_ok());
    assert_eq!(il?.raw, rg.raw);

    Ok(())
}
