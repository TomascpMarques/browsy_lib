use browsy_helpers::text_utills::{self, TextPadding};
use colored::Colorize;
use scraper::Html;

use crate::{selector, versioning::SemanticVersion};

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
        let crate_url_from_html = Self::construct_crate_url(html_fragment);
        let crate_name_from_html = Self::construct_crate_name(html_fragment);
        let last_change_from_html = Self::construct_crate_last_change(html_fragment);
        let crate_description_from_html = Self::construct_crate_description(html_fragment);
        let crate_version_from_html = Self::construct_crate_version(html_fragment);

        match crate_url_from_html.is_none()
            || last_change_from_html.is_none()
            || crate_description_from_html.is_none()
            || crate_version_from_html.is_none()
            || crate_name_from_html.is_none()
        {
            false => Some(Self {
                crate_url: crate_url_from_html.unwrap(),
                crate_name: crate_name_from_html.unwrap(),
                last_changed: last_change_from_html.unwrap(),
                crate_description: crate_description_from_html.unwrap(),
                crate_version: crate_version_from_html.unwrap(),
            }),
            _ => None,
        }
    }

    pub fn print_crate_info(&self) {
        println!("{}", Self::crate_info_fmt(self))
    }

    pub fn crate_info_fmt(&self) -> String {
        format!(
            " {} {} - {}\n {}\n\n {}\n{}  {}=\"{}\"\n{}       cargo add {}\n\n{} {}\n",
            self.crate_name.p().bright_yellow().on_black(),
            self.crate_version.to_string().bold().green(),
            self.last_changed.italic(),
            text_utills::text_wrapp(self.crate_description.as_str(), 50),
            "Install".p().on_yellow().white().bold(),
            "Cargo.toml".p().white().on_red().italic().bold(),
            self.crate_name,
            self.crate_version,
            "Cargo".p().white().on_blue().italic().bold(),
            self.crate_name.bold(),
            "LINK".p().on_blue().white().bold().italic(),
            format!("https://docs.rs{}", self.crate_url)
                .bright_blue()
                .underline()
        )
    }

    pub fn crate_info_line_separated(&self) -> Vec<String> {
        Self::crate_info_fmt(&self)
            .split('\n')
            .map(|c| c.trim().into())
            .collect::<Vec<String>>()
    }

    pub fn crate_widget_fmt(&self) -> String {
        format!(
            "{} {} - {}",
            self.crate_name.p().bright_yellow().on_black(),
            self.crate_version.to_string().bold().green(),
            self.last_changed.italic(),
        )
    }

    pub fn crate_discriptor_fmt(&self) -> String {
        format!(
            "{} {}\n-> {}",
            self.crate_name
                .pad_left("· ", 1)
                .p()
                .bright_yellow()
                .on_black(),
            self.crate_version.to_string().bold().green(),
            self.crate_description,
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
