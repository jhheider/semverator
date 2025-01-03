use super::Semver;

impl Semver {
    pub fn chomp(&self) -> String {
        let result = self.raw.trim_end_matches(".0");
        if result.is_empty() {
            "0".to_string()
        } else {
            result.to_string()
        }
    }
}
