mod compare;
mod parse;

#[derive(Default, Debug, Clone, Hash)]
pub struct Semver<'a> {
    pub components: Vec<usize>,

    pub major: usize,
    pub minor: usize,
    patch: usize,

    // todo!()
    _prerelease: Vec<&'a str>,
    _build: Vec<&'a str>,

    pub raw: String,
}

impl<'a> Semver<'a> {
    pub fn infinty() -> Self {
        Self {
            components: vec![usize::MAX, usize::MAX, usize::MAX],
            major: usize::MAX,
            minor: usize::MAX,
            patch: usize::MAX,
            raw: "Infinity.Infinity.Infinity".to_string(),
            ..Default::default()
        }
    }
}
