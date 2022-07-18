use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct SemanticVersion(pub u32, pub u32, pub u32);

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
    ///   assert_eq!(mj, 1);
    ///   assert_eq!(mn, 3);
    ///   assert_eq!(pt, 42)
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
            .filter_map(|x| x.parse().ok())
            .collect::<Vec<u32>>();

        if formatted.len().ne(&3) {
            return Err("Bad value was given".to_owned());
        }

        Ok(Self(formatted[0], formatted[1], formatted[2]))
    }

    pub fn is_semantic_v<T>(target: T) -> bool
    where
        T: Display,
    {
        Self::new(target).is_ok()
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
mod test_semantic_versioning_suport {
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
                assert_eq!(v.0, 3);
                assert_eq!(v.1, 4);
                assert_eq!(v.2, 123)
            }
            Err(_) => assert!(true),
        }
    }
}
