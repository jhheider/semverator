mod semver {
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
}

mod range {
    use anyhow::Result;

    use crate::range::Range;

    #[test]
    fn test_parse() -> Result<()> {
        let a = Range::parse(">=11<15");
        let b = Range::parse("^16");
        let c = Range::parse("~16");
        let d = Range::parse("=16");
        let e = Range::parse("<16");
        let f = Range::parse(">=11<15||^16||~16||=16||<16");
        let g = Range::parse("*");
        let h = Range::parse("<=11>15");
        let i = Range::parse("Your mom");

        assert!(a.is_ok());
        assert!(b.is_ok());
        assert!(c.is_ok());
        assert!(d.is_ok());
        assert!(e.is_ok());
        assert!(f.is_ok());
        assert!(g.is_ok());
        assert!(h.is_err());
        assert!(i.is_err());

        assert_eq!(f?.set.len(), 5);

        Ok(())
    }
}
