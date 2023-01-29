use anyhow::Result;

use crate::{range::Range, semver::Semver};

#[test]
fn test_parse() -> Result<()> {
    let a = Range::parse(">=11<15");
    let b = Range::parse("^16");
    let c = Range::parse("~16");
    let d = Range::parse("=16");
    let e = Range::parse("<16");
    let f = Range::parse(">=11<15||^16||~16||=16||<16");
    let g = Range::parse("*");
    let h = Range::parse(">=15<12");
    let i = Range::parse("<=11>15");
    let j = Range::parse("Your mom");

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

    assert_eq!(f?.set.len(), 5);

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
    ];

    assert_eq!(ra.max(&sa).unwrap().raw, "17.8");
    assert_eq!(rb.max(&sa).unwrap().raw, "14.5");
    assert_eq!(rc.max(&sa).unwrap().raw, "16.8");

    Ok(())
}
