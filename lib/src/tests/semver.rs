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

    let e = Semver::parse("1.2.3-alpha")?;
    let f = Semver::parse("1.2.3-alpha.1")?;
    let g = Semver::parse("1.2.3-alpha.2")?;
    let h = Semver::parse("1.2.3-beta.1")?;
    let i = Semver::parse("1.2.3-alpha.1+8ec0834")?;
    let j = Semver::parse("1.2.3-alpha.1+7ec0834")?;
    let k = Semver::parse("1.2.3-alpha.2+7ec0834")?;

    assert!(e.lt(&f)); // 1.2.3-alpha < 1.2.3-alpha.1
    assert!(f.eq(&f)); // 1.2.3-alpha.1 == 1.2.3-alpha.1
    assert!(f.lt(&a)); // 1.2.3-alpha.1 < 1.2.3
    assert!(a.gt(&f)); // 1.2.3 > 1.2.3-alpha.1
    assert!(f.lt(&g)); // 1.2.3-alpha.1 < 1.2.3-alpha.2
    assert!(g.gt(&f)); // 1.2.3-alpha.2 > 1.2.3-alpha.1
    assert!(g.lt(&h)); // 1.2.3-alpha.2 < 1.2.3-beta.1
    assert!(f.lt(&i)); // 1.2.3-alpha.1 < 1.2.3-alpha.1+8ec0834
    assert!(i.gt(&f)); // 1.2.3-alpha.1+8ec0834 > 1.2.3-alpha.1
    assert!(i.eq(&i)); // 1.2.3-alpha.1+8ec0834 == 1.2.3-alpha.1+8ec0834
    assert!(i.gt(&j)); // 1.2.3-alpha.1+8ec0834 > 1.2.3-alpha.1+7ec0834
    assert!(j.lt(&i)); // 1.2.3-alpha.1+7ec0834 < 1.2.3-alpha.1+8ec0834
    assert!(i.lt(&k)); // 1.2.3-alpha.1+8ec0834 < 1.2.3-alpha.2+7ec0834
    assert!(j.lt(&k)); // 1.2.3-alpha.1+7ec0834 < 1.2.3-alpha.2+7ec0834

    Ok(())
}

#[test]
fn test_sort() -> Result<()> {
    let a = Semver::parse("1.2.3")?;
    let b = Semver::parse("10.3.4")?;
    let c = Semver::parse("1.2.4")?;
    let d = Semver::parse("1.2.3.1")?;
    let e = Semver::parse("2.3.4")?;
    let mut s1 = vec![a.clone(), b.clone(), c.clone(), d.clone(), e.clone()];
    s1.sort();

    let s2 = vec![a, d, c, e, b];

    assert_eq!(s1, s2);

    Ok(())
}

#[test]
fn test_calver_sort() -> Result<()> {
    let a = Semver::parse("1.2.3")?;
    let b = Semver::parse("2.3.4")?;
    let c = Semver::parse("2023.03.04")?;
    let d = Semver::parse("1.2.3.1")?;
    let e = Semver::parse("3.4.5")?;
    let mut s1 = vec![a.clone(), b.clone(), c.clone(), d.clone(), e.clone()];
    s1.sort();

    let s2 = vec![c, a, d, b, e];

    assert_eq!(s1, s2);

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

#[test]
fn test_display() -> Result<()> {
    let a = Semver::parse("1.2.3")?;
    let b = Semver::parse("1.2.3-alpha")?;
    let c = Semver::parse("1.2.0")?;
    let d = Semver::parse("1.0.0")?;
    let e = Semver::parse("1.2.3-alpha.1+b40")?;
    let f = Semver::parse("1.2.3-alpha.1+build.40")?;
    let g = Semver::parse("1")?;

    assert_eq!(a.to_string(), "1.2.3");
    assert_eq!(b.to_string(), "1.2.3-alpha");
    assert_eq!(c.to_string(), "1.2.0");
    assert_eq!(d.to_string(), "1.0.0");
    assert_eq!(e.to_string(), "1.2.3-alpha.1+b40");
    assert_eq!(f.to_string(), "1.2.3-alpha.1+build.40");
    assert_eq!(g.to_string(), "1");

    Ok(())
}
