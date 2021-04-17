use super::main;
use crate::data;

impl From<String> for main::FieldIdentifier {
    fn from(string: String) -> Self {
        if string == data::ANONYMOUS_FIELD_IDENTIFIER {
            main::FieldIdentifier::Anonymous
        } else {
            match string.parse() {
                Err(_) => main::FieldIdentifier::Named(string),
                Ok(index) => main::FieldIdentifier::Indexed(index),
            }
        }
    }
}

impl From<main::FieldIdentifier> for String {
    fn from(identifier: main::FieldIdentifier) -> Self {
        match identifier {
            main::FieldIdentifier::Anonymous => String::from(data::ANONYMOUS_FIELD_IDENTIFIER),
            main::FieldIdentifier::Named(name) => name,
            main::FieldIdentifier::Indexed(index) => index.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod from_string {
        use super::*;

        #[test]
        fn handles_anonymous() {
            let actual = main::FieldIdentifier::from(String::from('_'));

            let expected = main::FieldIdentifier::Anonymous;
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_named() {
            let actual = main::FieldIdentifier::from(String::from("ab"));

            let expected = main::FieldIdentifier::Named(String::from("ab"));
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_indexed() {
            let actual = main::FieldIdentifier::from(String::from("12"));

            let expected = main::FieldIdentifier::Indexed(12);
            assert_eq!(actual, expected);
        }
    }

    #[cfg(test)]
    mod from_identifier {
        use super::*;

        #[test]
        fn handles_anonymous() {
            let actual = String::from(main::FieldIdentifier::Anonymous);

            let expected = String::from('_');
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_named() {
            let actual = String::from(main::FieldIdentifier::Named(String::from("bc")));

            let expected = String::from("bc");
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_indexed() {
            let actual = String::from(main::FieldIdentifier::Indexed(23));

            let expected = String::from("23");
            assert_eq!(actual, expected);
        }
    }
}
