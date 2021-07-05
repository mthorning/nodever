use regex::Regex;
use std::cmp::Ordering;

#[cfg(not(test))]
use crate::cli::Cli;

#[cfg(test)]
use tests::Cli;

#[derive(Debug)]
pub struct Semver<'a> {
    pub major: &'a str,
    pub minor: &'a str,
    pub patch: &'a str,
    pub pre_release: Option<&'a str>,
    pub build_metadata: Option<&'a str>,
}

impl<'a> Semver<'a> {
    pub fn from(version: &'a str) -> Self {
        let re = Regex::new(r#"^(\d+)\.(\d+)\.(\d+)(?:-([.\-0-9a-zA-Z]+))?(?:\+([.\-0-9a-zA-Z]+))?$"#).unwrap();
        let groups = re.captures(version).unwrap();
        if groups.len() != 6 {
            panic!("Couldn't capture all parts of the semver {}", version);
        }
        
        let get_group = |number| groups.get(number).map_or(None, |version| Some(version.as_str()));

        Semver {
            major: get_group(1).unwrap(),
            minor: get_group(2).unwrap(),
            patch: get_group(3).unwrap(),
            pre_release: get_group(4),
            build_metadata: get_group(5),
        }  
    }

    pub fn to_string(&self) -> String {
        let mut version = format!("{}.{}.{}", self.major, self.minor, self.patch);
        if let Some(pre_release) = self.pre_release {
            version = format!("{}-{}", version, pre_release);
        }
        if let Some(build_metadata) = self.build_metadata {
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

impl<'a> Ord for Semver<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let compared = compare_parts(
            &vec![self.major, self.minor, self.patch],
            &vec![other.major, other.minor, other.patch],
        );  

        if let Ordering::Equal = compared {
            return match self.pre_release {
                Some(tags) => match other.pre_release {
                    None => Ordering::Less,
                    Some(other_tags) => compare_parts(
                        &tags.split('.').collect(),
                        &other_tags.split('.').collect(),
                    )
                },
                None => match other.pre_release {
                    None => Ordering::Equal,
                    Some(_) => Ordering::Greater,
                }
            }
        }
        return compared
    }

}

impl<'a> PartialOrd for Semver<'a>  {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for Semver<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl<'a> Eq for Semver<'a> {}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct Cli {
        pub meta: bool,
    }
    impl Cli {
        pub fn get() -> Self {
           Cli {
               meta: true
            }
        }
    }

    #[test]
    fn creates_semver() {
        let semver = Semver::from("8.1.2");
        assert_eq!(semver.major, "8");
        assert_eq!(semver.minor, "1");
        assert_eq!(semver.patch, "2");
        assert!(semver.pre_release.is_none());
        assert!(semver.build_metadata.is_none());
    }

    #[test]
    fn creates_complex_semver() {
        let semver = Semver::from("8.1.2-alpha.0.1+1.2.3");
        assert_eq!(semver.major, "8");
        assert_eq!(semver.minor, "1");
        assert_eq!(semver.patch, "2");

        match semver.pre_release {
            Some(version) => assert_eq!(version, "alpha.0.1"),
            None => assert!(false),
        };

        match semver.build_metadata {
            Some(version) => assert_eq!(version,  "1.2.3"),
            None => assert!(false),
        }
    }

    #[test]
    fn compares_two_semvers() {
        let main = Semver::from("8.4.12");
        assert_eq!(main.cmp(&Semver::from("7.4.12")), Ordering::Greater);
        assert_eq!(main.cmp(&Semver::from("8.3.12")), Ordering::Greater);
        assert_eq!(main.cmp(&Semver::from("8.4.11")), Ordering::Greater);
        assert_eq!(main.cmp(&Semver::from("8.4.13")), Ordering::Less);
        assert_eq!(main.cmp(&Semver::from("8.5.0")), Ordering::Less);
        assert_eq!(main.cmp(&Semver::from("9.0.0")), Ordering::Less);
        assert_eq!(main.cmp(&Semver::from("8.4.12")), Ordering::Equal);
    }

    #[test]
    fn use_operators() {
        let main = Semver::from("8.4.12");
        assert!(main > Semver::from("7.4.12"));
        assert!(main > Semver::from("8.3.12"));
        assert!(main > Semver::from("8.4.11"));
        assert!(main < Semver::from("8.4.13"));
        assert!(main < Semver::from("8.5.0"));
        assert!(main < Semver::from("9.0.0"));
        assert!(main  == Semver::from("8.4.12"));
    }

    #[test]
    fn compares_prerelease_tags() {
        let main = Semver::from("8.4.12");
        assert!(main > Semver::from("8.4.12-alpha.0.1"));
        assert!(main < Semver::from("8.4.13-alpha.0.1"));
        assert!(main < Semver::from("8.5.0-alpha.0.1"));
        assert!(main < Semver::from("9.0.0-alpha.0.1"));

        let main = Semver::from("8.4.12-alpha.1.1");
        assert!(main < Semver::from("8.4.12-beta.0.1"));
        assert!(main > Semver::from("8.4.12-alpha.0.1"));
        assert!(main < Semver::from("8.4.12-alpha.2.0"));
        assert!(main < Semver::from("8.4.12-alpha.1.2"));
        assert!(main > Semver::from("8.4.12-alpha.1.0"));
        assert!(main == Semver::from("8.4.12-alpha.1.1"));
    }

    #[test]
    fn returns_a_string() {
        assert_eq!(Semver::from("8.4.12").to_string(), "8.4.12");
        assert_eq!(Semver::from("8.4.12-alpha.1.1").to_string(), "8.4.12-alpha.1.1");
        assert_eq!(Semver::from("8.4.12-alpha.1.1+13.xxx.3").to_string(), "8.4.12-alpha.1.1+13.xxx.3");
    }
}
