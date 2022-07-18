use std::collections::HashSet;

use scraper::{Html, Selector};

use crate::versioning::SemanticVersion;

pub struct DocsQuery {
    pub topic: String,
    pub crate_results: HashSet<String, DocsCrate>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct DocsCrate {
    pub crate_url: String,
    pub crate_name: String,
    pub last_changed: String,
    pub crate_description: String,
    pub crate_version: SemanticVersion,
}

macro_rules! select {
    ($selector: expr) => {
        Selector::parse($selector).unwrap()
    };
}

impl DocsCrate {
    pub fn new(html_fragment: &String) -> Option<Self> {
        Some(Self {
            crate_url: Self::construct_crate_url(&html_fragment)?,
            crate_name: Self::construct_crate_name(&html_fragment)?,
            last_changed: Self::construct_crate_last_change(&html_fragment)?,
            crate_description: Self::construct_crate_description(&html_fragment)?,
            crate_version: Self::construct_crate_version(&html_fragment)?,
        })
    }

    pub fn extract_fragments_inner_html(
        html_fragment: &String,
        slct_err_msg: &str,
        selector: &str,
    ) -> Option<String> {
        let fragment = Html::parse_fragment(html_fragment.as_str());

        let selected = match fragment.select(&select!(selector)).next() {
            Some(elem) => elem,
            None => {
                println!("{slct_err_msg}");
                return None;
            }
        };

        if selected.inner_html().is_empty() {
            None
        } else {
            Some(selected.inner_html().trim().to_string())
        }
    }

    pub fn extract_fragment_html_attribute(
        html_fragment: &String,
        slct_err_msg: &str,
        selector: &str,
        attrbt: &str,
    ) -> Option<String> {
        let fragment = Html::parse_fragment(html_fragment.as_str());

        let selected = match fragment.select(&select!(selector)).next() {
            Some(elem) => elem,
            None => {
                println!("{slct_err_msg}");
                return None;
            }
        };

        selected.value().attr(attrbt).map(|v| v.to_string())
    }

    pub fn construct_crate_version(html_fragment: &String) -> Option<SemanticVersion> {
        match Self::extract_fragment_html_attribute(
            html_fragment,
            "Could not extract the crates version as a single piece of data",
            "a.release",
            "href",
        ) {
            Some(v) => {
                println!("->{v}");
                Some(
                    v.split('/')
                        .find_map(|x| SemanticVersion::new(x).ok())
                        .unwrap(),
                )
            }
            None => {
                println!("Could not extract the crates version as a single piece of data");
                None
            }
        }
    }
    pub fn construct_crate_description(html_fragment: &String) -> Option<String> {
        Self::extract_fragments_inner_html(
            html_fragment,
            "Could not extract the crates description",
            "div.description",
        )
    }

    pub fn construct_crate_last_change(html_fragment: &String) -> Option<String> {
        Self::extract_fragments_inner_html(
            html_fragment,
            "Could not parse the last chnage in the crate",
            "div.date",
        )
    }

    pub fn construct_crate_name(html_fragment: &String) -> Option<String> {
        Self::extract_fragments_inner_html(
            html_fragment,
            "Could not extract the name from the fragment",
            "div.name",
        )
    }

    pub fn construct_crate_url(html_fragment: &String) -> Option<String> {
        Self::extract_fragment_html_attribute(html_fragment, "", "a.release", "href")
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

    use super::DocsCrate;

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
