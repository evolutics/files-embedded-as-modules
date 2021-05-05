use crate::data;
use crate::model;

pub fn main(
    structure: model::TypeStructure<()>,
) -> model::Result<model::TypeStructure<model::Populator>> {
    match structure {
        model::TypeStructure::Unit => Ok(model::TypeStructure::Unit),

        model::TypeStructure::TypeAlias(_) => Err(model::Error::NoInitializer),

        model::TypeStructure::NamedFields(fields) => Ok(model::TypeStructure::NamedFields(
            fields
                .into_iter()
                .map(|(field, _)| {
                    let populator = get_populator(&field)?;
                    Ok((field, populator))
                })
                .collect::<model::Result<_>>()?,
        )),

        model::TypeStructure::TupleFields(unary_length) => {
            if unary_length.is_empty() {
                Ok(model::TypeStructure::TupleFields(vec![]))
            } else {
                Err(model::Error::NoInitializer)
            }
        }
    }
}

fn get_populator(field: &str) -> model::Result<model::Populator> {
    match data::STANDARD_FIELD_POPULATORS_ORDERED.binary_search_by_key(&field, |entry| entry.0) {
        Err(_) => Err(model::Error::NonstandardField {
            field: String::from(field),
        }),
        Ok(index) => Ok(data::STANDARD_FIELD_POPULATORS_ORDERED[index].1.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_unit() {
        let actual = main(model::TypeStructure::Unit);

        let actual = actual.unwrap();
        let expected = model::TypeStructure::Unit;
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_type_alias() {
        let actual = main(model::TypeStructure::TypeAlias(()));

        let actual = actual.unwrap_err();
        let expected = model::Error::NoInitializer;
        assert_eq!(actual, expected);
    }

    #[cfg(test)]
    mod handles_named_fields {
        use super::*;

        #[test]
        fn given_standard_fields_only_it_handles() {
            let actual = main(model::TypeStructure::NamedFields(vec![
                (String::from("relative_path"), ()),
                (String::from("contents_str"), ()),
            ]));

            let actual = actual.unwrap();
            let expected = model::TypeStructure::NamedFields(vec![
                (
                    String::from("relative_path"),
                    model::Populator::RelativePath,
                ),
                (String::from("contents_str"), model::Populator::ContentsStr),
            ]);
            assert_eq!(actual, expected);
        }

        #[test]
        fn given_nonstandard_field_it_errs() {
            let actual = main(model::TypeStructure::NamedFields(vec![
                (String::from("relative_path"), ()),
                (String::from("abc"), ()),
            ]));

            let actual = actual.unwrap_err();
            let expected = model::Error::NonstandardField {
                field: String::from("abc"),
            };
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_each_standard_field() {
            let actual = main(model::TypeStructure::NamedFields(
                data::STANDARD_FIELD_POPULATORS_ORDERED
                    .iter()
                    .map(|(field, _)| (String::from(*field), ()))
                    .collect(),
            ));

            let actual = actual.unwrap();
            let expected = model::TypeStructure::NamedFields(
                data::STANDARD_FIELD_POPULATORS_ORDERED
                    .iter()
                    .map(|(field, populator)| (String::from(*field), populator.clone()))
                    .collect(),
            );
            assert_eq!(actual, expected);
        }
    }

    #[cfg(test)]
    mod handles_tuple_fields {
        use super::*;

        #[test]
        fn given_no_fields_it_handles() {
            let actual = main(model::TypeStructure::TupleFields(vec![]));

            let actual = actual.unwrap();
            let expected = model::TypeStructure::TupleFields(vec![]);
            assert_eq!(actual, expected);
        }

        #[test]
        fn given_fields_it_errs() {
            let actual = main(model::TypeStructure::TupleFields(vec![()]));

            let actual = actual.unwrap_err();
            let expected = model::Error::NoInitializer;
            assert_eq!(actual, expected);
        }
    }
}
