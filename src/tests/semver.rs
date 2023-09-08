use crate::semver::{bump::SemverComponent, Semver};
use anyhow::Result;

#[test]
fn test_parse() -> Result<()> {
    assert_eq!(Semver::parse("1.2.3.4.5.6")?.raw, "1.2.3.4.5.6");
    assert_eq!(Semver::parse("1.2.3.4.5")?.raw, "1.2.3.4.5");
    assert_eq!(Semver::parse("1.2.3.4")?.raw, "1.2.3.4");
    assert_eq!(Semver::parse("1.2.3")?.raw, "1.2.3");
    assert_eq!(Semver::parse("v1.2.3")?.raw, "1.2.3");
    assert_eq!(Semver::parse("1.2")?.raw, "1.2.0");
    assert_eq!(Semver::parse("v1.2")?.raw, "1.2.0");
    assert_eq!(Semver::parse("1")?.raw, "1.0.0");
    assert_eq!(Semver::parse("v1")?.raw, "1.0.0");

    assert_eq!(Semver::parse("9e")?.raw, "9e");
    assert_eq!(Semver::parse("9e")?.components, [9, 5]);
    assert_eq!(Semver::parse("3.3a")?.raw, "3.3a");
    assert_eq!(Semver::parse("3.3a")?.components, [3, 3, 1]);
    assert_eq!(Semver::parse("1.1.1q")?.raw, "1.1.1q");
    assert_eq!(Semver::parse("1.1.1q")?.components, [1, 1, 1, 17]);

    assert_eq!(Semver::parse("1.2.3-alpha")?.raw, "1.2.3-alpha");
    assert_eq!(Semver::parse("1.2-alpha")?.raw, "1.2.0-alpha");
    assert_eq!(Semver::parse("1-alpha")?.raw, "1.0.0-alpha");
    assert_eq!(Semver::parse("1.2.3-alpha.1")?.raw, "1.2.3-alpha.1");

    assert_eq!(Semver::parse("1.2.3+build")?.raw, "1.2.3+build");
    assert_eq!(Semver::parse("1.2+build")?.raw, "1.2.0+build");
    assert_eq!(Semver::parse("1+build")?.raw, "1.0.0+build");
    assert_eq!(Semver::parse("1.2.3+build.1")?.raw, "1.2.3+build.1");

    assert_eq!(Semver::parse("1.2.3-alpha+build")?.raw, "1.2.3-alpha+build");
    assert_eq!(Semver::parse("1.2-alpha+build")?.raw, "1.2.0-alpha+build");
    assert_eq!(Semver::parse("1-alpha+build")?.raw, "1.0.0-alpha+build");
    assert_eq!(
        Semver::parse("1.2.3-alpha.1+build.1")?.raw,
        "1.2.3-alpha.1+build.1"
    );

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
