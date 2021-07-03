use regex::Regex;

pub struct Semver {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl Semver {
    pub fn from(version: &str) -> Self {
        let re = Regex::new(r#"(\d+)\.(\d+)\.(\d+)[-]?.*"#).unwrap();
        let groups = re.captures(version).unwrap();
        if groups.len() != 4 {
            panic!("Couldn't capture all three parts of the semver {}", version);
        }

        let get_group = |number| {
               groups.get(number).map_or(0, |version| version.as_str().parse::<u16>().unwrap())
        };

        Semver {
            major: get_group(1),
            minor: get_group(2),
            patch: get_group(3),
        }  
    }

    pub fn is_newer(&self, to_compare: &Self) -> bool {
        if self.major.gt(&to_compare.major) { return true; }
        if self.minor.gt(&to_compare.minor) { return true; }
        if self.patch.gt(&to_compare.patch) { return true; }
        false
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_semver_struct() {
        let semver = Semver::from("8.1.2");
        assert_eq!(semver.major, 8);
        assert_eq!(semver.minor, 1);
        assert_eq!(semver.patch, 2);
    }

    #[test]
    fn finds_the_newest() {
        let main = Semver::from("8.4.12");
        assert!(main.is_newer(&Semver::from("7.4.12")));
        assert!(main.is_newer(&Semver::from("8.3.12")));
        assert!(main.is_newer(&Semver::from("8.4.11")));
        assert!(!main.is_newer(&Semver::from("8.4.13")));
        assert!(!main.is_newer(&Semver::from("8.5.0")));
        assert!(!main.is_newer(&Semver::from("9.0.0")));
    }
}
