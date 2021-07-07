use regex::Regex;
use std::cmp::Ordering;

#[cfg(not(test))]
use crate::cli::Cli;

#[cfg(test)]
use tests::Cli;

#[derive(Debug)]
pub struct Semver {
    pub major: String,
    pub minor: String,
    pub patch: String,
    pub pre_release: Option<String>,
    pub build_metadata: Option<String>,
}

impl Semver {
    pub fn from(version: String) -> Option<Self> {
        let re = Regex::new(
            r#"^[~>=<^]*(\d+)\.(\d+)\.(\d+)(?:-([.\-0-9a-zA-Z]+))?(?:\+([.\-0-9a-zA-Z]+))?$"#,
        )
        .unwrap();
        match re.captures(&version) {
            Some(captures) => {
                let get_string = |num| {
                    captures
                        .get(num)
                        .map_or(String::new(), |v| v.as_str().to_string())
                };
                let get_option = |num| {
                    captures
                        .get(num)
                        .map_or(None, |v| Some(v.as_str().to_string()))
                };

                Some(Semver {
                    major: get_string(1),
                    minor: get_string(2),
                    patch: get_string(3),
                    pre_release: get_option(4),
                    build_metadata: get_option(5),
                })
            }
            None => None,
        }
    }

    pub fn to_string(&self) -> String {
        let mut version = format!("{}.{}.{}", self.major, self.minor, self.patch);
        if let Some(pre_release) = &self.pre_release {
            version = format!("{}-{}", version, pre_release);
        }
        if let Some(build_metadata) = &self.build_metadata {
            let cli = Cli::get();
            if cli.meta {
                version = format!("{}+{}", version, build_metadata);
            }
        }
        version
    }
}

fn compare_parts(parts: &Vec<&str>, other_parts: &Vec<&str>) -> Ordering {
    for (i, part) in parts.iter().enumerate() {
        let other_part = other_parts[i];
        let ordering: Ordering;

        if let Ok(int) = part.parse::<u16>() {
            if let Ok(other_int) = other_part.parse::<u16>() {
                ordering = int.cmp(&other_int);
            } else {
                ordering = part.cmp(&other_part);
            }
        } else {
            ordering = part.cmp(&other_part);
        }
        if let Ordering::Equal = ordering {
            continue;
        } else {
            return ordering;
        }
    }
    Ordering::Equal
}

impl Ord for Semver {
    fn cmp(&self, other: &Self) -> Ordering {
        let compared = compare_parts(
            &vec![&self.major, &self.minor, &self.patch],
            &vec![&other.major, &other.minor, &other.patch],
        );

        if let Ordering::Equal = compared {
            return match &self.pre_release {
                Some(tags) => match &other.pre_release {
                    None => Ordering::Less,
                    Some(other_tags) => {
                        compare_parts(&tags.split('.').collect(), &other_tags.split('.').collect())
                    }
                },
                None => match other.pre_release {
                    None => Ordering::Equal,
                    Some(_) => Ordering::Greater,
                },
            };
        }
        return compared;
    }
}

impl PartialOrd for Semver {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Semver {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Eq for Semver {}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct Cli {
        pub meta: bool,
    }
    impl Cli {
        pub fn get() -> Self {
            Cli { meta: true }
        }
    }

    fn make(version: &str) -> Semver {
        Semver::from(String::from(version)).unwrap()
    }

    #[test]
    fn creates_semver() {
        let semver = make("8.1.2");
        assert_eq!(semver.major, "8");
        assert_eq!(semver.minor, "1");
        assert_eq!(semver.patch, "2");
        assert!(semver.pre_release.is_none());
        assert!(semver.build_metadata.is_none());
    }

    #[test]
    fn handles_non_semver_string() {
        let non_semver = Semver::from(String::from("yalc:some-file"));
        assert_eq!(non_semver, None);
    }

    #[test]
    fn creates_complex_semver() {
        let semver = make("8.1.2-alpha.0.1+1.2.3");
        assert_eq!(semver.major, "8");
        assert_eq!(semver.minor, "1");
        assert_eq!(semver.patch, "2");

        match semver.pre_release {
            Some(version) => assert_eq!(version, "alpha.0.1"),
            None => assert!(false),
        };

        match semver.build_metadata {
            Some(version) => assert_eq!(version, "1.2.3"),
            None => assert!(false),
        }
    }

    #[test]
    fn compares_two_semvers() {
        let main = make("8.4.12");
        assert_eq!(main.cmp(&make("7.4.12")), Ordering::Greater);
        assert_eq!(main.cmp(&make("8.3.12")), Ordering::Greater);
        assert_eq!(main.cmp(&make("8.4.11")), Ordering::Greater);
        assert_eq!(main.cmp(&make("8.4.13")), Ordering::Less);
        assert_eq!(main.cmp(&make("8.5.0")), Ordering::Less);
        assert_eq!(main.cmp(&make("9.0.0")), Ordering::Less);
        assert_eq!(main.cmp(&make("8.4.12")), Ordering::Equal);
    }

    #[test]
    fn use_operators() {
        let main = make("8.4.12");
        assert!(main > make("7.4.12"));
        assert!(main > make("8.3.12"));
        assert!(main > make("8.4.11"));
        assert!(main < make("8.4.13"));
        assert!(main < make("8.5.0"));
        assert!(main < make("9.0.0"));
        assert!(main == make("8.4.12"));
    }

    #[test]
    fn compares_prerelease_tags() {
        let main = make("8.4.12");
        assert!(main > make("8.4.12-alpha.0.1"));
        assert!(main < make("8.4.13-alpha.0.1"));
        assert!(main < make("8.5.0-alpha.0.1"));
        assert!(main < make("9.0.0-alpha.0.1"));

        let main = make("8.4.12-alpha.1.1");
        assert!(main < make("8.4.12-beta.0.1"));
        assert!(main > make("8.4.12-alpha.0.1"));
        assert!(main < make("8.4.12-alpha.2.0"));
        assert!(main < make("8.4.12-alpha.1.2"));
        assert!(main > make("8.4.12-alpha.1.0"));
        assert!(main == make("8.4.12-alpha.1.1"));
    }

    #[test]
    fn returns_a_string() {
        assert_eq!(make("8.4.12").to_string(), "8.4.12");
        assert_eq!(make("8.4.12-alpha.1.1").to_string(), "8.4.12-alpha.1.1");
        assert_eq!(
            make("8.4.12-alpha.1.1+13.xxx.3").to_string(),
            "8.4.12-alpha.1.1+13.xxx.3"
        );
    }
}
