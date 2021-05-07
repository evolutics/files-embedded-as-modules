macro_rules! visit_base {
    ($length:literal, $($contents:expr)*) => {
        pub static ASSETS: once_cell::sync::Lazy<std::collections::HashMap<&str, Asset>> =
            once_cell::sync::Lazy::new(|| vec![$($contents,)*].into_iter().collect());
    };
}

macro_rules! visit_file {
    ($name:literal, $id:ident, $index:literal, $relative_path:literal, $absolute_path:literal) => {
        (
            $relative_path,
            Asset {
                contents: include_str!($absolute_path),
            },
        )
    };
}

#[iftree::include_file_tree(
    "
paths = '/examples/assets/**'

[[template]]
visit_base = 'visit_base'
visit_file = 'visit_file'
"
)]
pub struct Asset {
    contents: &'static str,
}

pub fn main() {
    assert_eq!(ASSETS.len(), 6);

    let mut keys = ASSETS.keys().collect::<Vec<_>>();
    keys.sort_unstable();
    assert_eq!(
        keys,
        vec![
            &"examples/assets/.env",
            &"examples/assets/configuration/menu.json",
            &"examples/assets/configuration/translations.csv",
            &"examples/assets/credits.md",
            &"examples/assets/world/levels/tutorial.json",
            &"examples/assets/world/physical_constants.json",
        ],
    );

    assert_eq!(
        ASSETS.get("examples/assets/credits.md").unwrap().contents,
        "Boo Far\n",
    );

    assert!(ASSETS.get("examples/assets/seed.json").is_none());
}
