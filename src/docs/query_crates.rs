use std::collections::HashSet;

use scraper::Html;

use super::crate_details;

#[macro_export]
macro_rules! selector {
    ($selector: expr) => {
        scraper::Selector::parse($selector).unwrap()
    };
}

pub struct DocsQuery {
    pub topic: String,
    target_html: String,
    pub crate_results: HashSet<crate_details::DocsCrate>,
}

impl DocsQuery {
    pub fn new(topic: String, target_html: String) -> Self {
        Self {
            topic,
            target_html: target_html.clone(),
            crate_results: Self::get_crates_from_result(target_html),
        }
    }

    pub fn get_crates_from_result(html: String) -> HashSet<crate_details::DocsCrate> {
        Html::parse_fragment(html.as_str())
            .select(&selector!("a.release"))
            .filter_map(|elem| crate_details::DocsCrate::new(elem.html().as_str()))
            .collect()
    }

    pub fn set_target_html(&mut self, target_html: String) {
        self.target_html = target_html;
    }

    pub fn target_html(&self) -> &str {
        self.target_html.as_ref()
    }
}

#[cfg(test)]
macro_rules! doc_crate_test_builder {
    ($html: literal; $fn: ident; $want: expr) => {
        let html_fragment = $html;

        let have = DocsCrate::$fn(&html_fragment.to_string()).unwrap();
        let want = $want;

        assert_eq!(have, want)
    };
}

#[cfg(test)]
mod test_docs_crate {
    use crate::versioning::SemanticVersion;

    use super::crate_details::DocsCrate;

    #[test]
    fn test_new_docs_crate() {
        doc_crate_test_builder!(
            r#"
                <a href="/generics/0.4.4/generics/" class="release">
                    <div class="pure-g">
                        <div class="pure-u-1 pure-u-sm-6-24 pure-u-md-5-24 name">
                            generics-0.4.4
                        </div>
                        <div class="pure-u-1 pure-u-sm-14-24 pure-u-md-16-24 description">
                            Provides macros for parsing generics (with optional where clause) in `macro_rules!`.
                        </div>
                        <div class="pure-u-1 pure-u-sm-4-24 pure-u-md-3-24 date"
                            title="2022-05-24T19:05:31Z">
                            May 24, 2022
                        </div>
                    </div>
                </a>
            "#;
            new;
            DocsCrate {
                crate_url: "/generics/0.4.4/generics/".to_string(),
                crate_name: "generics-0.4.4".to_string(),
                last_changed: "May 24, 2022".to_string(),
                crate_description: "Provides macros for parsing generics (with optional where clause) in `macro_rules!`.".to_string(),
                crate_version: SemanticVersion::new("0.4.4").unwrap(),
            }
        );
    }

    #[test]
    fn test_crate_last_changed() {
        doc_crate_test_builder!(
          r#"
            <div class="pure-u-1 pure-u-sm-14-24 pure-u-md-16-24 description">
                Generically forward references for operations on Copy types.
            </div>

            <div class="pure-u-1 pure-u-sm-4-24 pure-u-md-3-24 date"
                title="2022-02-12T18:55:08Z">
                    Feb 12, 2022
            </div>
          "#;
          construct_crate_last_change;
          "Feb 12, 2022".to_string()
        );
    }

    #[test]
    fn test_crate_name_construction() {
        doc_crate_test_builder!(
            r#"
                <div class="pure-u-1 pure-u-sm-14-24 pure-u-md-16-24 description">
                    Generically forward references for operations on Copy types.

                </div>

                <div class="pure-u-1 pure-u-sm-6-24 pure-u-md-5-24 name">
                    forward_ref_generic-0.2.1
                </div>
            "#;
            construct_crate_name;
            "forward_ref_generic-0.2.1".to_string()
        );
    }

    #[test]
    fn test_crate_url_construction() {
        doc_crate_test_builder!(
            r#"<a href="/forward_ref_generic/0.2.1/forward_ref_generic/" class="release">"#;
            construct_crate_url;
            "/forward_ref_generic/0.2.1/forward_ref_generic/".to_string()
        );
    }

    #[test]
    fn test_construct_crate_version() {
        doc_crate_test_builder!(
            r#"<a href="/forward_ref_generic/0.2.1/forward_ref_generic/" class="release">"#;
            construct_crate_version;
            SemanticVersion::new_from_numbers(0, 2, 1)
        );
    }
}
