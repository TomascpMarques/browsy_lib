use std::fmt::Display;

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone)]
pub struct SemanticVersion(pub String, pub String, pub String);

impl Display for SemanticVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.0, self.1, self.2)
    }
}

impl SemanticVersion {
    /// Creates a new Semantic version from a string separated by dots
    /// Input is a Generic that implements display,
    /// This way accept both &str for ease of programming,
    /// and Strings as well.
    /// ## Example:
    /// ```
    /// # use crate::browsy_lib::versioning::SemanticVersion;
    /// # fn main() {
    ///   let have = SemanticVersion::new("1.3.42").unwrap();
    ///   let (mj, mn, pt) = (have.major(), have.minor(), have.patch());
    ///
    ///   assert_eq!(mj, "1".to_string());
    ///   assert_eq!(mn, "3".to_string());
    ///   assert_eq!(pt, "42".to_string())
    /// # }
    /// ```
    pub fn new<T>(target: T) -> Result<Self, String>
    where
        T: Display,
    {
        let formatted = target
            .to_string()
            .split('.')
            .take(3)
            .filter_map(|x| x.parse::<u32>().ok())
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        if formatted.len().ne(&3) {
            return Err("Bad value was given".to_owned());
        }

        Ok(Self(
            formatted[0].clone(),
            formatted[1].clone(),
            formatted[2].clone(),
        ))
    }

    pub fn is_semantic_v<T>(target: T) -> bool
    where
        T: Display,
    {
        Self::new(target).is_ok()
    }

    pub fn new_from_numbers(num: u32, num1: u32, num2: u32) -> Self {
        Self(num.to_string(), num1.to_string(), num2.to_string())
    }

    pub fn major(&self) -> String {
        self.0.clone()
    }

    pub fn minor(&self) -> String {
        self.1.clone()
    }

    pub fn patch(&self) -> String {
        self.2.clone()
    }
}

#[cfg(test)]
mod test_semantic_versioning_suport {
    use crate::versioning::SemanticVersion;

    #[test]
    fn test_semantic_version() {
        let have = SemanticVersion::new("1.3.42").unwrap();
        let (mj, mn, pt) = (have.major(), have.minor(), have.patch());

        assert_eq!(mj, "1".to_string());
        assert_eq!(mn, "3".to_string());
        assert_eq!(pt, "42".to_string())
    }

    #[test]
    fn test_semantic_version_fail() {
        match SemanticVersion::new("hasi3.4asfk4.sfkka") {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        };
        match SemanticVersion::new("hasi3.4as.fk4.sfke3.1.2.33.44.1.ka") {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        };
    }

    #[test]
    fn test_semantic_version_zero_padded() {
        match SemanticVersion::new("03.04.0123") {
            Ok(v) => {
                assert_eq!(v.0, "3".to_string());
                assert_eq!(v.1, "4".to_string());
                assert_eq!(v.2, "123".to_string())
            }
            Err(_) => assert!(true),
        }
    }
}
