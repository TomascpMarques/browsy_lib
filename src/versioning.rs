use std::fmt::Display;

pub struct SemanticVersion(u32, u32, u32);

impl SemanticVersion {
    pub fn new<T>(target: T) -> Result<Self, String>
    where
        T: Display,
    {
        let formatted = target
            .to_string()
            .split(".")
            .filter(|&x| x.parse::<u32>().is_ok())
            .map(|x| x.parse().unwrap())
            .take(3)
            .collect::<Vec<u32>>();

        if formatted.len().ne(&3) {
            return Err("Bad value was given".to_owned());
        }

        Ok(Self(
            formatted[0].clone(),
            formatted[1].clone(),
            formatted[2].clone(),
        ))
    }

    pub fn new_from_numbers(num: u32, num1: u32, num2: u32) -> Self {
        Self(num, num1, num2)
    }

    pub fn major(&self) -> u32 {
        self.0
    }

    pub fn minor(&self) -> u32 {
        self.1
    }

    pub fn patch(&self) -> u32 {
        self.2
    }
}

#[cfg(test)]
mod test_docs {
    use crate::versioning::SemanticVersion;

    #[test]
    fn test_semantic_version() {
        let have = SemanticVersion::new("1.3.42").unwrap();
        let (mj, mn, pt) = (have.major(), have.minor(), have.patch());

        assert_eq!(mj, 1);
        assert_eq!(mn, 3);
        assert_eq!(pt, 42)
    }

    #[test]
    fn test_semantic_version_sfail() {
        match SemanticVersion::new("hasi3.4asfk4.sfkka") {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn test_semantic_version_zero_padded() {
        match SemanticVersion::new("03.04.0123") {
            Ok(v) => {
                assert_eq!(v.0, 3);
                assert_eq!(v.1, 4);
                assert_eq!(v.2, 123)
            }
            Err(_) => assert!(true),
        }
    }
}
