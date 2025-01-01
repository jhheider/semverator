pub mod bump;
pub mod compare;
pub mod parse;

#[derive(Default, Debug, Clone, Eq)]
pub struct Semver {
    pub components: Vec<usize>,

    pub major: usize,
    pub minor: usize,
    pub patch: usize,

    pub prerelease: Vec<String>,
    pub build: Vec<String>,

    pub raw: String,
}

impl Semver {
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
