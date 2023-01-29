use crate::semver;
use anyhow::Result;

#[test]
fn test_parse() -> Result<()> {
    let a = semver::Semver::parse("1.2.3");
    let b = semver::Semver::parse("1.2.4");
    let c = semver::Semver::parse("1.2.four");

    assert!(a.is_ok());
    assert!(b.is_ok());
    assert!(c.is_err());

    assert_eq!(a?.raw, "1.2.3");
    assert_eq!(b?.raw, "1.2.4");

    Ok(())
}

#[test]
fn test_compare() -> Result<()> {
    let a = semver::Semver::parse("1.2.3")?;
    let b = semver::Semver::parse("1.2.4")?;

    assert!(a.eq(&a));
    assert!(!a.eq(&b));

    assert!(!a.neq(&a));
    assert!(a.neq(&b));

    assert!(!a.gt(&b));
    assert!(b.gt(&a));

    assert!(a.lt(&b));
    assert!(!b.lt(&a));

    Ok(())
}
