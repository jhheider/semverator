use crate::semver::{bump::SemverComponent, Semver};
use anyhow::Result;

#[test]
fn test_parse() -> Result<()> {
    let a = Semver::parse("1.2.3");
    let b = Semver::parse("1.2.4");
    let c = Semver::parse("1.2.four");
    let d = Semver::parse("1.1.1q");

    assert!(a.is_ok());
    assert!(b.is_ok());
    assert!(c.is_err());
    assert!(d.is_ok());

    let d = d?;

    assert_eq!(a?.raw, "1.2.3");
    assert_eq!(b?.raw, "1.2.4");
    assert_eq!(d.raw, "1.1.1q");
    assert_eq!(d.components, [1, 1, 1, 17]);

    Ok(())
}

#[test]
fn test_compare() -> Result<()> {
    let a = Semver::parse("1.2.3")?;
    let b = Semver::parse("1.2.4")?;

    assert!(a.eq(&a));
    assert!(!a.eq(&b));

    assert!(!a.neq(&a));
    assert!(a.neq(&b));

    assert!(!a.gt(&b));
    assert!(b.gt(&a));

    assert!(a.lt(&b));
    assert!(!b.lt(&a));

    let c = Semver::parse("1.1.1q")?;
    let d = Semver::parse("1.1.1s")?;

    assert!(c.eq(&c));
    assert!(!c.eq(&d));

    assert!(!c.neq(&c));
    assert!(c.neq(&d));

    assert!(!c.gt(&d));
    assert!(d.gt(&c));

    assert!(c.lt(&d));
    assert!(!d.lt(&c));

    Ok(())
}

#[test]
fn test_bump() -> Result<()> {
    let a = Semver::parse("1.2.3")?;
    let b = Semver::parse("1.2.4")?;
    let c = Semver::parse("1.3.0")?;
    let d = Semver::parse("2.0.0")?;

    assert_eq!(a.bump(&SemverComponent::Major)?, d);
    assert_eq!(a.bump(&SemverComponent::Minor)?, c);
    assert_eq!(a.bump(&SemverComponent::Patch)?, b);
    assert_eq!(a.bump(&SemverComponent::None)?, a);

    assert_eq!(SemverComponent::from("major"), SemverComponent::Major);
    assert_eq!(SemverComponent::from("minor"), SemverComponent::Minor);
    assert_eq!(SemverComponent::from("patch"), SemverComponent::Patch);
    assert_eq!(SemverComponent::from("gibberish"), SemverComponent::None);

    Ok(())
}

#[test]
fn test_infinty() {
    let inf = Semver::infinty();

    assert_eq!(inf.components, [usize::MAX, usize::MAX, usize::MAX]);
    assert_eq!(inf.raw, "Infinity.Infinity.Infinity");
}
