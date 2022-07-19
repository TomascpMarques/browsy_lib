use std::fmt::Display;

use colored::Colorize;

#[macro_export]
macro_rules! line_separator {
    () => {
        println!("{}", "⁘".repeat(30).bright_black());
    };
    ($c: tt) => {
        println!("{}", "⁘".repeat($c).bright_black());
    };
}

pub fn explain_something<K, T>(reason: T, explanation: K) -> String
where
    K: Display,
    T: Display,
{
    format!(
        "{} {}\n{}\n{}\n{}",
        "* Info *".p().white().on_blue().bold(),
        reason.to_string().white().bold(),
        "⁘".repeat(30).bright_black(),
        explanation.to_string().white(),
        "⁘".repeat(30).bright_black(),
    )
}

pub fn text_wrapp(target: &str, width: usize) -> String {
    let mut counter = 0;
    let mut buffer = target
        .split_whitespace()
        .map(|x| x.into())
        .collect::<Vec<String>>();

    buffer.iter_mut().for_each(|s| {
        counter += s.len();
        if counter > width {
            *s = "\n".to_owned() + s;
            counter = 0;
        }
    });

    buffer.join(" ")
}

#[cfg(test)]
mod test_loose_functions {
    use colored::Colorize;

    use super::{explain_something, TextPadding};

    #[test]
    fn test_explain_reason_strs() {
        let reason = "Bad parameter was given";
        let explanation = "Was expecting a simple susum, got a complex clinical depression";

        let have = explain_something(reason, explanation);

        let want = format!(
            "{} {}\n{}\n{}\n{}",
            "* Info *".p().white().on_blue().bold(),
            reason.to_string().white().bold(),
            "⁘".repeat(30).bright_black(),
            explanation.to_string().white(),
            "⁘".repeat(30).bright_black(),
        );

        assert_eq!(have, want)
    }
}

pub trait TextPadding {
    fn pad_left(&self, p: &str, c: usize) -> String
    where
        Self: Display,
    {
        format!("{:wdth$}{}", p, self, wdth = c)
    }

    fn pad_right(&self, p: &str, c: usize) -> String
    where
        Self: Display,
    {
        format!("{}{:wdth$}", self, p, wdth = c)
    }

    fn pad(&self, p: &str, c: usize) -> String
    where
        Self: Display,
    {
        format!("{:wdth$}{}{:wdth$}", p, self, p, wdth = c)
    }

    fn p(&self) -> String
    where
        Self: Display,
    {
        format!("{:wdth$}{}{:wdth$}", " ", self, " ", wdth = 1)
    }
}

impl TextPadding for &str {}

impl TextPadding for String {}

#[cfg(test)]
mod test {
    use super::TextPadding;

    #[test]
    fn padding_left_4() {
        let want = "    lorem";
        let have = "lorem".pad_left(" ", 4);
        assert_eq!(want, have)
    }

    #[test]
    fn padding_left_2() {
        let want = "  lorem";
        let have = "lorem".pad_left(" ", 2);
        assert_eq!(want, have)
    }

    #[test]
    fn padding_left_1() {
        let want = " lorem";
        let have = "lorem".pad_left(" ", 1);
        assert_eq!(want, have)
    }

    #[test]
    fn padding_right_1() {
        let want = "lorem ";
        let have = "lorem".pad_right(" ", 1);
        assert_eq!(want, have)
    }

    #[test]
    fn padding_right_2() {
        let want = "lorem  ";
        let have = "lorem".pad_right(" ", 2);
        assert_eq!(want, have)
    }

    #[test]
    fn padding_right_4() {
        let want = "lorem    ";
        let have = "lorem".pad_right(" ", 4);
        assert_eq!(want, have)
    }

    #[test]
    fn pad_both_sides_4() {
        let want = "    lorem    ";
        let have = "lorem".pad(" ", 4);
        assert_eq!(want, have)
    }

    #[test]
    fn pad_both_sides_2() {
        let want = "  lorem  ";
        let have = "lorem".pad(" ", 2);
        assert_eq!(want, have)
    }

    #[test]
    fn pad_both_sides_1() {
        let want = " lorem ";
        let have = "lorem".pad(" ", 1);
        assert_eq!(want, have)
    }
}
