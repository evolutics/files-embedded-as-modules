pub fn main(original: &str, convention: Convention) -> String {
    let identifier = sanitize_by_convention(original, convention);
    let identifier = sanitize_special_characters(&identifier);
    let identifier = sanitize_first_character(identifier);
    let identifier = sanitize_special_cases(identifier);
    return format!("r#{}", identifier);
}

pub enum Convention {
    ScreamingSnakeCase,
    SnakeCase,
}

fn sanitize_by_convention(identifier: &str, convention: Convention) -> String {
    match convention {
        Convention::ScreamingSnakeCase => identifier.to_uppercase(),
        Convention::SnakeCase => identifier.to_lowercase(),
    }
}

fn sanitize_special_characters(identifier: &str) -> String {
    identifier
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character
            } else {
                '_'
            }
        })
        .collect()
}

fn sanitize_first_character(identifier: String) -> String {
    match identifier.chars().next() {
        Some(first_character) if first_character.is_numeric() => format!("_{}", identifier),
        _ => identifier,
    }
}

fn sanitize_special_cases(identifier: String) -> String {
    match identifier.as_ref() {
        "" => String::from("__"),
        "_" | "crate" | "self" | "Self" | "super" => format!("{}_", identifier),
        _ => identifier,
    }
}

#[cfg(test)]
mod stubs {
    use super::*;

    pub fn convention() -> Convention {
        Convention::ScreamingSnakeCase
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitizes_by_convention_of_screaming_snake_case() {
        let actual = main("README.md", Convention::ScreamingSnakeCase);

        let expected = "r#README_MD";
        assert_eq!(actual, expected);
    }

    #[test]
    fn sanitizes_by_convention_of_snake_case() {
        let actual = main("README.md", Convention::SnakeCase);

        let expected = "r#readme_md";
        assert_eq!(actual, expected);
    }

    #[test]
    fn sanitizes_special_characters() {
        let actual = main("A B##C_D±EÅF𝟙G.H", Convention::ScreamingSnakeCase);

        let expected = "r#A_B__C_D_E_F_G_H";
        assert_eq!(actual, expected);
    }

    #[test]
    fn sanitizes_first_character() {
        let actual = main("2a", Convention::SnakeCase);

        let expected = "r#_2a";
        assert_eq!(actual, expected);
    }

    #[test]
    fn sanitizes_empty_string() {
        let actual = main("", stubs::convention());

        let expected = "r#__";
        assert_eq!(actual, expected);
    }

    #[test]
    fn sanitizes_wildcard_pattern() {
        let actual = main("_", stubs::convention());

        let expected = "r#__";
        assert_eq!(actual, expected);
    }

    #[test]
    fn sanitizes_special_keywords() {
        let actual = main("self", Convention::SnakeCase);

        let expected = "r#self_";
        assert_eq!(actual, expected);
    }

    #[test]
    fn sanitizes_other_keywords() {
        let actual = main("match", Convention::SnakeCase);

        let expected = "r#match";
        assert_eq!(actual, expected);
    }
}
