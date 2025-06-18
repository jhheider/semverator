use crate::{range::Range, semver::Semver};
use anyhow::Result;
#[cfg(feature = "serde")]
use serde_test::{assert_tokens, Token};

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
    let m = Range::parse("@1");
    let n = Range::parse("@1.1");
    let o = Range::parse("@1.1.1");
    let p = Range::parse("@1.1.1.1");

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
    assert!(m.is_ok());
    assert!(n.is_ok());
    assert!(o.is_ok());
    assert!(p.is_ok());

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

    let re = Range::parse("^0.1.0")?;

    let se = Semver::parse("0.1.1")?;
    let sf = Semver::parse("0.2.1")?;

    assert!(re.satisfies(&se));
    assert!(!re.satisfies(&sf));

    let rg = Range::parse("^0.0.5")?;

    let sg = Semver::parse("0.0.5")?;
    let sh = Semver::parse("0.0.6")?;

    assert!(rg.satisfies(&sg));
    assert!(!rg.satisfies(&sh));

    let ri = Range::parse("^0")?;

    let si = Semver::parse("0.0.5")?;
    let sj = Semver::parse("0.21.1")?;
    let sk = Semver::parse("1.0.0")?;

    assert!(ri.satisfies(&si));
    assert!(ri.satisfies(&sj));
    assert!(!ri.satisfies(&sk));

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

    assert_eq!(ra.max(&sa).unwrap().raw, "17.8.0");
    assert_eq!(rb.max(&sa).unwrap().raw, "14.5.0");
    assert_eq!(rc.max(&sa).unwrap().raw, "16.8.0");
    assert!(rd.max(&sa).is_none());
    Ok(())
}

#[test]
fn test_intersect() -> Result<()> {
    let ra = Range::parse("^3.7")?;
    let rb = Range::parse("=3.11")?;

    let ia = ra.intersect(&rb);
    assert!(ia.is_ok());
    assert_eq!(ia?.raw, "=3.11.0");

    let rc = Range::parse("^3.9")?;

    let ib = ra.intersect(&rc);
    assert!(ib.is_ok());
    assert_eq!(ib?.raw, ">=3.9.0<4.0.0");

    let rd = Range::parse("*")?;

    let ic = ra.intersect(&rd);
    assert!(ic.is_ok());
    assert_eq!(ic?.raw, "^3.7");

    let re = Range::parse("~3.7")?;
    let rf = Range::parse("~3.8")?;

    let id = re.intersect(&rf);
    assert!(id.is_err());

    let rg = Range::parse("=3.8.0")?;

    let ie = ra.intersect(&rg);
    assert!(ie.is_ok());
    assert_eq!(ie?.raw, rg.raw);

    let rh = Range::parse("^11,^12")?;
    let ri = Range::parse("^11.3")?;

    let r#if = rh.intersect(&ri);
    assert!(r#if.is_ok());
    assert_eq!(r#if?.raw, ">=11.3.0<12.0.0");

    let rj = Range::parse("^11.3,^12.2")?;

    let ig = rh.intersect(&rj);
    assert!(ig.is_ok());
    assert_eq!(ig?.raw, ">=11.3.0<12.0.0,>=12.2.0<13.0.0");

    let rk = Range::parse("*")?;

    let ih = rk.intersect(&ra);
    assert!(ih.is_ok());
    assert_eq!(ih?.raw, ra.raw);

    let ii = ra.intersect(&rk);
    assert!(ii.is_ok());
    assert_eq!(ii?.raw, ra.raw);

    let rl = Range::parse("=3.8.0")?;
    let rm = Range::parse("=3.9.0")?;

    let ij = rg.intersect(&rl);
    let ik = rg.intersect(&rm);

    assert!(ij.is_ok());
    assert!(ik.is_err());
    assert_eq!(ij?.raw, rl.raw);

    let il = rg.intersect(&ra);
    assert!(il.is_ok());
    assert_eq!(il?.raw, rg.raw);

    // This is the test for https://github.com/pkgxdev/pkgx/issues/1190
    let rn = Range::any();
    let ro = Range::parse(">=5.0.0<Infinity.Infinity.Infinity")?;

    let im = rn.intersect(&ro);
    assert!(im.is_ok());
    assert_eq!(im?.raw, ro.raw);

    let r#in = ro.intersect(&rn);
    assert!(r#in.is_ok());
    assert_eq!(r#in?.raw, ro.raw);

    Ok(())
}

#[test]
fn test_at() -> Result<()> {
    let ra = Range::parse(">=1.0<1.1")?;
    let rb = Range::parse("=1.1")?;

    assert_eq!(format!("{ra}"), "~1");
    assert_eq!(format!("{rb}"), "=1.1");

    let rc = Range::parse(">=1.1.0<1.1.1")?;
    let rd = Range::parse("=1.1.1")?;

    assert_eq!(format!("{rc}"), "@1.1.0");
    assert_eq!(format!("{rd}"), "=1.1.1");

    let re = Range::parse(">=1.1.1.0<1.1.1.1")?;
    let rf = Range::parse("=1.1.1.0")?;

    assert_eq!(format!("{re}"), "@1.1.1.0");
    assert_eq!(format!("{rf}"), "=1.1.1.0");

    let rg = Range::parse(">=1.1<1.1.1.1.1")?;
    let rh = Range::parse(">=1.1.1<1.1.3")?;
    let ri = Range::parse(">=1.1.1<1.2.2")?;

    assert_eq!(format!("{rg}"), ">=1.1<1.1.1.1.1");
    assert_eq!(format!("{rh}"), ">=1.1.1<1.1.3");
    assert_eq!(format!("{ri}"), ">=1.1.1<1.2.2");

    let rj = Range::parse("@1")?;
    let rk = Range::parse("@1.1")?;
    let rl = Range::parse("@1.1.1")?;
    let rm = Range::parse("@1.1.1.1")?;

    assert_eq!(format!("{rj}"), "^1");
    assert_eq!(format!("{rk}"), "~1.1");
    assert_eq!(format!("{rl}"), "@1.1.1");
    assert_eq!(format!("{rm}"), "@1.1.1.1");

    Ok(())
}

#[test]
fn test_display() -> Result<()> {
    let ra = Range::parse("^3.7")?;
    let rb = Range::parse("=3.11")?;
    let rc = Range::parse("^3.9")?;
    let rd = Range::parse("*")?;
    let re = Range::parse(">=0<1")?;
    let rf = Range::parse(">=0.1<1")?;
    let rg = Range::parse(">=0.1<0.2")?;
    let rh = Range::parse(">=0.1.1<0.2")?;
    let ri = Range::parse(">=0.1.1")?;
    let rj = Range::parse(">=0.1.1<3")?;

    assert_eq!(ra.to_string(), "^3.7");
    assert_eq!(rb.to_string(), "=3.11");
    assert_eq!(rc.to_string(), "^3.9");
    assert_eq!(rd.to_string(), "*");
    assert_eq!(re.to_string(), "^0");
    assert_eq!(rf.to_string(), ">=0.1<1");
    assert_eq!(rg.to_string(), "~0.1");
    assert_eq!(rh.to_string(), "~0.1.1");
    assert_eq!(ri.to_string(), ">=0.1.1");
    assert_eq!(rj.to_string(), ">=0.1.1<3");

    // This is the test for https://github.com/pkgxdev/pkgx/issues/1190
    let rk = Range::parse(">=5.0.0<Infinity.Infinity.Infinity")?;
    assert_eq!(rk.to_string(), ">=5");

    Ok(())
}

#[test]
fn test_constructors() -> Result<()> {
    let ra = Range::parse("*")?;
    let rb = Range::any();

    assert_eq!(ra.raw, rb.raw);

    let rc = Range::parse("=1.2.3")?;
    let rd = Range::single("1.2.3")?;

    assert_eq!(rc.raw, rd.raw);

    let re = Range::parse(">=1.2.3<2.0.0")?;
    let rf = Range::contiguous("1.2.3", "2.0.0")?;

    assert_eq!(re.raw, rf.raw);

    let rg = Range::parse("^1.2.3")?;
    let rh = Range::caret("1.2.3")?;

    assert_eq!(rg.raw, rh.raw);

    let ri = Range::parse("~1.2.3")?;
    let rj = Range::tilde("1.2.3")?;

    assert_eq!(ri.raw, rj.raw);

    let sa = Semver::parse("1.2.3")?;
    let rk = Range::from_semver(&sa)?;

    assert_eq!(rd.raw, rk.raw);

    Ok(())
}

#[cfg(feature = "serde")]
#[test]
fn test_serde() -> Result<()> {
    let ra = Range::parse("^3.7")?;
    let rb = Range::parse("=3.11")?;
    let rc = Range::parse("^3.9")?;

    assert_tokens(&ra, &[Token::Str("^3.7")]);
    assert_tokens(&rb, &[Token::Str("=3.11")]);
    assert_tokens(&rc, &[Token::Str("^3.9")]);

    let rd = serde_json::from_str::<Range>("\"your mom\"");
    assert!(rd.is_err());

    Ok(())
}
