use super::main;
use std::error;
use std::fmt;
use std::path;

impl PartialEq for main::IgnoreError {
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}

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

            main::Error::Ignore(main::IgnoreError(error)) => write!(formatter, "{}", error),

            main::Error::MissingFieldTemplate(field) => {
                let field = String::from(field.clone());
                write!(
                    formatter,
                    "No template for field {:?}. Add one to your configuration as follows:
```
[field_templates]
{} = …
```",
                    field, field,
                )
            }

            main::Error::NameCollision(main::NameCollisionError {
                collider,
                identifier,
            }) => write!(
                formatter,
                "File {:?} collides on generated identifier {:?} \
                with another file; rename the file or configure {:?}.",
                collider.0, identifier, "module_tree = false",
            ),

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
            main::Error::Ignore(main::IgnoreError(error)) => Some(error),
            main::Error::MissingFieldTemplate(_) => None,
            main::Error::NameCollision(_) => None,
            main::Error::PathStripPrefix(error) => Some(error),
        }
    }
}

impl From<ignore::Error> for main::Error {
    fn from(error: ignore::Error) -> Self {
        main::Error::Ignore(main::IgnoreError(error))
    }
}

impl From<path::StripPrefixError> for main::Error {
    fn from(error: path::StripPrefixError) -> Self {
        main::Error::PathStripPrefix(error)
    }
}
