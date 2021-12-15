pub struct TypingUnion(Vec<String>);

impl TypingUnion {
    pub fn new<T: Into<Vec<String>>>(values: T) -> Self {
        Self(values.into())
    }

    pub fn to_string(self, indentation: &str) -> String {
        match self.0.len() {
            0 => String::from("unknown"),
            1 => self.0.into_iter().nth(0).unwrap(),
            _ => self.0.join(&format!("\n{}| ", indentation)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn no_values() {
        let union = TypingUnion::new([]);

        self::assert_eq!(union.to_string(""), String::from("unknown"));
    }

    #[test]
    fn one_value() {
        let union = TypingUnion::new([String::from("string")]);

        self::assert_eq!(union.to_string(""), String::from("string"));
    }

    #[test]
    fn multi_value() {
        let union = TypingUnion::new([
            String::from("string"),
            String::from("number"),
            String::from("boolean"),
        ]);
        let indentation = "    ";

        self::assert_eq!(
            ["string", "    | number", "    | boolean"].join("\n"),
            union.to_string(indentation)
        );
    }
}
