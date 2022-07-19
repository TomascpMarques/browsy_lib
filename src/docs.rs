use std::collections::HashSet;

use colored::Colorize;
use scraper::{Html, Selector};

use crate::{
    text_utills::{self, TextPadding},
    versioning::SemanticVersion,
};

macro_rules! selector {
    ($selector: expr) => {
        Selector::parse($selector).unwrap()
    };
}

pub struct DocsQuery {
    pub topic: String,
    target_html: String,
    pub crate_results: HashSet<DocsCrate>,
}

impl DocsQuery {
    pub fn new(topic: String, target_html: String) -> Self {
        Self {
            topic,
            target_html: target_html.clone(),
            crate_results: Self::get_crates_from_result(target_html),
        }
    }

    pub fn get_crates_from_result(html: String) -> HashSet<DocsCrate> {
        Html::parse_fragment(html.as_str())
            .select(&selector!("a.release"))
            .filter_map(|elem| DocsCrate::new(elem.html().as_str()))
            .collect()
    }

    pub fn set_target_html(&mut self, target_html: String) {
        self.target_html = target_html;
    }

    pub fn target_html(&self) -> &str {
        self.target_html.as_ref()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct DocsCrate {
    pub crate_url: String,
    pub crate_name: String,
    pub last_changed: String,
    pub crate_description: String,
    pub crate_version: SemanticVersion,
}

impl DocsCrate {
    pub fn new(html_fragment: &str) -> Option<Self> {
        Some(Self {
            crate_url: Self::construct_crate_url(html_fragment)?,
            crate_name: Self::construct_crate_name(html_fragment)?,
            last_changed: Self::construct_crate_last_change(html_fragment)?,
            crate_description: Self::construct_crate_description(html_fragment)?,
            crate_version: Self::construct_crate_version(html_fragment)?,
        })
    }

    pub fn print_crate_info(&self) {
        println!("{}", Self::crate_info_fmt(self))
    }

    pub fn crate_info_fmt(&self) -> String {
        format!(
            "█ {} {} - {}\n█ {}",
            self.crate_name.p().bright_yellow().on_black(),
            self.crate_version.to_string().bold().green(),
            self.last_changed.italic(),
            text_utills::text_wrapp(self.crate_description.as_str(), 40),
        )
    }

    pub fn crate_description_fmt(&self) -> String {
        format!(
            "{} {} - {}",
            self.crate_name
                .pad_left("· ", 1)
                .p()
                .bright_yellow()
                .on_black(),
            self.crate_version.to_string().bold().green(),
            self.last_changed.italic(),
        )
    }

    pub fn extract_fragments_inner_html(
        html_fragment: &str,
        slct_err_msg: &str,
        selector: &str,
    ) -> Option<String> {
        let fragment = Html::parse_fragment(html_fragment);

        let selected = match fragment.select(&selector!(selector)).next() {
            Some(elem) => elem,
            None => {
                println!(
                    "{} {}",
                    "HTML Parssing".to_string().white().on_red().bold(),
                    slct_err_msg.to_string().yellow()
                );
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
        html_fragment: &str,
        slct_err_msg: &str,
        selector: &str,
        attrbt: &str,
    ) -> Option<String> {
        let fragment = Html::parse_fragment(html_fragment);

        let selected = match fragment.select(&selector!(selector)).next() {
            Some(elem) => elem,
            None => {
                println!(
                    "{} {}",
                    "HTML Parssing".to_string().white().on_red().bold(),
                    slct_err_msg.to_string().yellow()
                );
                return None;
            }
        };

        selected.value().attr(attrbt).map(|v| v.to_string())
    }

    pub fn construct_crate_version(html_fragment: &str) -> Option<SemanticVersion> {
        match Self::extract_fragment_html_attribute(
            html_fragment,
            "Could not extract the crates version as a single piece of data",
            "a.release",
            "href",
        ) {
            Some(v) => Some(
                v.split('/')
                    .find_map(|x| SemanticVersion::new(x).ok())
                    .unwrap_or_default(),
                // TODO handle words in the semantic versioning i.e: 0.1.0-alpha.2
            ),
            None => {
                println!(
                    "{} {}",
                    "Crate info".to_string().white().on_red().bold(),
                    "Could not extract the crates version as a single piece of data"
                        .to_string()
                        .yellow()
                );
                None
            }
        }
    }
    pub fn construct_crate_description(html_fragment: &str) -> Option<String> {
        Self::extract_fragments_inner_html(
            html_fragment,
            "Could not extract the crates description",
            "div.description",
        )
    }

    pub fn construct_crate_last_change(html_fragment: &str) -> Option<String> {
        Self::extract_fragments_inner_html(
            html_fragment,
            "Could not parse the last chnage in the crate",
            "div.date",
        )
    }

    pub fn construct_crate_name(html_fragment: &str) -> Option<String> {
        match Self::extract_fragment_html_attribute(
            html_fragment,
            "Could not get the crate name",
            "a.release",
            "href",
        ) {
            Some(v) => Some(v.split('/').find(|x| !x.is_empty()).unwrap().to_string()),
            _ => None,
        }
    }

    pub fn construct_crate_url(html_fragment: &str) -> Option<String> {
        Self::extract_fragment_html_attribute(
            html_fragment,
            "Could not extract the crate url",
            "a.release",
            "href",
        )
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
