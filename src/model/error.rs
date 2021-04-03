use super::main;
use std::error;
use std::fmt;
use std::path;

impl fmt::Display for main::Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            main::Error::EnvironmentVariable(main::EnvironmentVariableError { name, source }) => {
                write!(
                    formatter,
                    "Unable to get environment variable {:?}: {}",
                    name, source,
                )
            }

            main::Error::Ignore(error) => write!(formatter, "{}", error),

            main::Error::MissingImplementation(field) => {
                let field_hint = match field {
                    main::FieldIdentifier::Anonymous => String::new(),
                    main::FieldIdentifier::Named(name) => format!("field {:?} of ", name),
                };
                write!(
                    formatter,
                    "No implementation configured for {}resource type.",
                    field_hint,
                )
            }

            main::Error::NameCollisions(collisions) => {
                let configuration = "resolve_name_collisions = true";
                write!(
                    formatter,
                    "Name collisions in generated code; \
                    rename files or configure {:?}:",
                    configuration,
                )?;
                for collision in collisions {
                    let existing_file_hint = match &collision.existing_filename {
                        None => String::new(),
                        Some(filename) => format!("with {:?} ", filename),
                    };
                    write!(
                        formatter,
                        "\n- {:?} collides {}on identifier {:?}.",
                        collision.colliding_file.relative_path,
                        existing_file_hint,
                        collision.identifier,
                    )?;
                }
                Ok(())
            }

            main::Error::PathStripPrefix(error) => write!(formatter, "{}", error),
        }
    }
}

impl error::Error for main::Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            main::Error::EnvironmentVariable(main::EnvironmentVariableError { source, .. }) => {
                Some(source)
            }
            main::Error::Ignore(error) => Some(error),
            main::Error::MissingImplementation(_) => None,
            main::Error::NameCollisions(_) => None,
            main::Error::PathStripPrefix(error) => Some(error),
        }
    }
}

impl From<ignore::Error> for main::Error {
    fn from(error: ignore::Error) -> Self {
        main::Error::Ignore(error)
    }
}

impl From<path::StripPrefixError> for main::Error {
    fn from(error: path::StripPrefixError) -> Self {
        main::Error::PathStripPrefix(error)
    }
}
