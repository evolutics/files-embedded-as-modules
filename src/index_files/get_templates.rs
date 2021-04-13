use crate::data;
use crate::model;

pub fn main<'a>(
    configuration: &'a model::Configuration,
    resource_structure: &model::ResourceStructure<()>,
) -> model::Result<model::ResourceStructure<&'a model::Template>> {
    Ok(match resource_structure {
        model::ResourceStructure::Unit => model::ResourceStructure::Unit,

        model::ResourceStructure::TypeAlias(_) => model::ResourceStructure::TypeAlias(
            get_template(configuration, model::FieldIdentifier::Anonymous)?,
        ),

        model::ResourceStructure::NamedFields(names) => model::ResourceStructure::NamedFields(
            names
                .iter()
                .map(|(name, _)| {
                    Ok((
                        name.clone(),
                        get_template(
                            configuration,
                            model::FieldIdentifier::Named(String::from(name)),
                        )?,
                    ))
                })
                .collect::<model::Result<_>>()?,
        ),

        model::ResourceStructure::TupleFields(structure) => model::ResourceStructure::TupleFields(
            structure
                .iter()
                .enumerate()
                .map(|(index, _)| {
                    get_template(configuration, model::FieldIdentifier::Indexed(index))
                })
                .collect::<model::Result<_>>()?,
        ),
    })
}

fn get_template(
    configuration: &model::Configuration,
    identifier: model::FieldIdentifier,
) -> model::Result<&model::Template> {
    match configuration.field_templates.get(&identifier) {
        None => {
            let name = String::from(identifier.clone());
            match data::PREDEFINED_TEMPLATES_ORDERED.binary_search_by(|entry| entry.0.cmp(&name)) {
                Err(_) => Err(model::Error::MissingFieldTemplate(identifier)),
                Ok(index) => Ok(&data::PREDEFINED_TEMPLATES_ORDERED[index].1),
            }
        }

        Some(template) => Ok(template),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_no_field_template_at_all_it_errs() {
        let configuration = model::Configuration {
            field_templates: model::FieldTemplates::new(),
            ..model::stubs::configuration()
        };

        let actual = main(&configuration, &model::ResourceStructure::TypeAlias(()));

        let actual = actual.unwrap_err();
        let expected = model::Error::MissingFieldTemplate(model::FieldIdentifier::Anonymous);
        assert_eq!(actual, expected);
    }

    #[test]
    fn given_no_configured_field_template_it_defaults_to_predefined() {
        let configuration = model::Configuration {
            field_templates: model::FieldTemplates::new(),
            ..model::stubs::configuration()
        };

        let actual = main(
            &configuration,
            &model::ResourceStructure::NamedFields(vec![(String::from("content"), ())]),
        );

        let actual = actual.unwrap();
        let expected = model::ResourceStructure::NamedFields(vec![(
            String::from("content"),
            &model::Template::Content,
        )]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn given_configured_field_template_it_gets_it() {
        let configuration = model::Configuration {
            field_templates: vec![(
                model::FieldIdentifier::Named(String::from("content")),
                model::Template::RawContent,
            )]
            .into_iter()
            .collect(),
            ..model::stubs::configuration()
        };

        let actual = main(
            &configuration,
            &model::ResourceStructure::NamedFields(vec![(String::from("content"), ())]),
        );

        let actual = actual.unwrap();
        let expected = model::ResourceStructure::NamedFields(vec![(
            String::from("content"),
            &model::Template::RawContent,
        )]);
        assert_eq!(actual, expected);
    }

    #[cfg(test)]
    mod resource_cases {
        use super::*;

        #[test]
        fn gets_unit() {
            let configuration = model::stubs::configuration();

            let actual = main(&configuration, &model::ResourceStructure::Unit);

            let actual = actual.unwrap();
            let expected = model::ResourceStructure::Unit;
            assert_eq!(actual, expected);
        }

        #[test]
        fn gets_type_alias() {
            let configuration = model::Configuration {
                field_templates: vec![(
                    model::FieldIdentifier::Anonymous,
                    model::Template::Content,
                )]
                .into_iter()
                .collect(),
                ..model::stubs::configuration()
            };

            let actual = main(&configuration, &model::ResourceStructure::TypeAlias(()));

            let actual = actual.unwrap();
            let expected = model::ResourceStructure::TypeAlias(&model::Template::Content);
            assert_eq!(actual, expected);
        }

        #[test]
        fn gets_named_fields() {
            let configuration = model::Configuration {
                field_templates: vec![(
                    model::FieldIdentifier::Named(String::from("my_content")),
                    model::Template::RawContent,
                )]
                .into_iter()
                .collect(),
                ..model::stubs::configuration()
            };

            let actual = main(
                &configuration,
                &model::ResourceStructure::NamedFields(vec![(String::from("my_content"), ())]),
            );

            let actual = actual.unwrap();
            let expected = model::ResourceStructure::NamedFields(vec![(
                String::from("my_content"),
                &model::Template::RawContent,
            )]);
            assert_eq!(actual, expected);
        }

        #[test]
        fn gets_tuple_fields() {
            let configuration = model::Configuration {
                field_templates: vec![(
                    model::FieldIdentifier::Indexed(0),
                    model::Template::RelativePath,
                )]
                .into_iter()
                .collect(),
                ..model::stubs::configuration()
            };

            let actual = main(
                &configuration,
                &model::ResourceStructure::TupleFields(vec![()]),
            );

            let actual = actual.unwrap();
            let expected =
                model::ResourceStructure::TupleFields(vec![&model::Template::RelativePath]);
            assert_eq!(actual, expected);
        }
    }
}
