use fancy_regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref NUMBER: Regex = number();
}

fn number() -> Regex {
    let binary = ('b', r"[0-1]");
    let octal = ('o', r"[1-7]");
    let hexa = ('x', r"[\da-f]");
    let decimal = r"(?:\d+(\.\d+)?)";

    let number = [binary, octal, hexa]
        .map(|(c, r)| format!(r"(?:0{c}((?!0{c}){r})+)", c = c, r = r))
        .join("|")
        + "|"
        + decimal;

    let regex = format!(r"(?i){}(?-i)n?", number);

    Regex::new(&regex).unwrap()
}
