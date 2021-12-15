use std::convert::From;

use itertools::Itertools;
use unicode_segmentation::UnicodeSegmentation;

pub fn upper_first(s: &str) -> String {
    let mut graphemes = s.graphemes(true);
    match graphemes.next() {
        Some(c) => c.to_uppercase() + graphemes.as_str(),
        None => String::new(),
    }
}

pub fn add_indentation(indentation: &str, s: &str) -> String {
    s.split("\n")
        .into_iter()
        .map(|s| {
            if s.trim().len() > 0 {
                String::from(indentation) + s
            } else {
                String::from(s)
            }
        })
        .join("\n")
}

#[cfg(test)]
mod tests {
    #[test]
    fn upper_first() {
        assert_eq!(super::upper_first("test"), String::from("Test"));
    }

    #[test]
    fn add_indentation() {
        assert_eq!(
            super::add_indentation("  ", "t\ne\ns\nt"),
            String::from("  t\n  e\n  s\n  t")
        );
    }
}
