use std::collections::HashSet;

use crate::versioning::SemanticVersion;

pub struct Docs {}
pub struct DocsQuery {
    pub topic: String,
    pub crate_results: HashSet<String, DocsCrate>,
}
pub struct DocsCrate {
    pub crate_name: String,
    pub crate_version: SemanticVersion,
}
