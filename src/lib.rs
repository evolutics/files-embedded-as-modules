//! Include file data from many files in your Rust code for self-contained binaries.
//!
//! # Motivation
//!
//! Self-contained binaries are easy to ship, as they come with any required file
//! data such as game assets, web templates, etc.
//!
//! The standard library's `std::include_str!` includes the contents of a given
//! file. Iftree generalizes this in two ways:
//!
//! 1. Not just one, but many files can be included at once with **path patterns**
//!    in a `.gitignore`-like format.
//! 1. Instead of including the file contents only, files can be associated with
//!    **any data** such as their media type, a compiled template, etc.
//!
//! Conceptually:
//!
//! ```text
//! std:       include_str!("my_file")
//! Iftree:    any_macro!("my_files/**")
//! ```
//!
//! # Introduction
//!
//! The basic functionality is simple. Say you have the following files:
//!
//! ```text
//! my_assets/
//! - file_a
//! - file_b
//! - subfolder/
//!   - file_c
//! ```
//!
//! To include these files in your code, the macro `iftree::include_file_tree` is
//! attached to a custom type like so:
//!
//! ```ignore
//! #[iftree::include_file_tree("paths = '/my_assets/**'")]
//! pub struct MyAsset {
//!     contents_str: &'static str,
//! }
//! ```
//!
//! Here we configure a path pattern to filter the files in `my_assets/` and its
//! subfolders. For each selected file, an instance of `MyAsset` is initialized. The
//! standard field `contents_str` is automatically populated with a call to
//! `include_str!`, but you can plug in your own initializer.
//!
//! Based on this, Iftree generates an array `ASSETS` with the desired file data.
//! You can use it like this:
//!
//! ```ignore
//! assert_eq!(ASSETS.len(), 3);
//! assert_eq!(ASSETS[0].contents_str, "… contents `file_a`\n");
//! assert_eq!(ASSETS[1].contents_str, "… contents `file_b`\n");
//! assert_eq!(ASSETS[2].contents_str, "… and `file_c`\n");
//! ```
//!
//! Furthermore, variables `base::x::y::MY_FILE` are generated (named by file path):
//!
//! ```ignore
//! assert_eq!(base::my_assets::FILE_A.contents_str, "… contents `file_a`\n");
//! assert_eq!(base::my_assets::FILE_B.contents_str, "… contents `file_b`\n");
//! assert_eq!(base::my_assets::subfolder::FILE_C.contents_str, "… and `file_c`\n");
//! ```
//!
//! # Usage
//!
//! Now that you have a general idea of the library, learn how to integrate it with
//! your project.
//!
//! By the way, there is an
//! [**`examples` folder**](https://github.com/evolutics/iftree/tree/main/examples)
//! if you like to explore by example.
//!
//! ## Getting started
//!
//! 1. Add the dependency `iftree = "0.1"` to your manifest (`Cargo.toml`).
//! 1. Define your **asset type.** This is a `struct` with the fields you need per
//!    file (`MyAsset` in the [introduction](#introduction)). Alternatively, it can
//!    be a type alias, which may be convenient if you have a exactly one field.
//! 1. Next, **filter files** to be included by annotating your asset type with
//!    `#[iftree::include_file_tree("paths = '…'")]`. Path patterns in a
//!    `.gitignore`-like format are supported, with one pattern per line. The paths
//!    are relative to the folder with your manifest by default. Patterns are
//!    flexible: you can skip hidden files, filter by filename extension, add
//!    multiple folders, select a fixed list of files, etc. See the
//!    [`paths` configuration](#paths) for more.
//! 1. The generated code then uses an **initializer** to instantiate the asset type
//!    once per file. By default, a field `contents_str` (if any) is populated with
//!    `include_str!`, a field `contents_bytes` is populated with `include_bytes!`,
//!    and a couple of other [standard fields](#standard-fields) are recognized.
//!    However, you can plug in your own macro to fully customize the initialization
//!    by [configuring an `initializer`](#templateinitializer). For even more
//!    control over code generation, there is the concept of
//!    [visitors](#template-visitors).
//!
//! The
//! [showcase example](https://github.com/evolutics/iftree/blob/main/examples/showcase.rs)
//! combines these things.
//!
//! ## Standard fields
//!
//! When you use fields from the following list only, an initializer for your asset
//! type is generated without further configuration. You can still override these
//! field names with a [custom `initializer`](#templateinitializer).
//!
//! - **`contents_bytes:`** `&'static [u8]`
//!
//!   File contents as a byte array, using
//!   [`std::include_bytes`](https://doc.rust-lang.org/std/macro.include_bytes.html).
//!
//! - **`contents_str:`** `&'static str`
//!
//!   File contents interpreted as a UTF-8 string, using
//!   [`std::include_str`](https://doc.rust-lang.org/std/macro.include_str.html).
//!
//! - **`get_bytes:`** `fn() -> std::borrow::Cow<'static, [u8]>`
//!
//!   In debug builds (that is, when
//!   [`debug_assertions`](https://doc.rust-lang.org/reference/conditional-compilation.html#debug_assertions)
//!   is enabled), this function reads the file afresh on each call at runtime. It
//!   panics if there is any error such as if the file does not exist. This helps
//!   with faster development, as it avoids rebuilding if asset file contents are
//!   changed only (note that you still need to rebuild if assets are added,
//!   renamed, or removed).
//!
//!   In release builds, it returns the file contents included at compile time,
//!   using
//!   [`std::include_bytes`](https://doc.rust-lang.org/std/macro.include_bytes.html).
//!
//! - **`get_str:`** `fn() -> std::borrow::Cow<'static, str>`
//!
//!   Same as `get_bytes` but for the file contents interpreted as a UTF-8 string,
//!   using
//!   [`std::include_str`](https://doc.rust-lang.org/std/macro.include_str.html).
//!
//! - **`relative_path:`** `&'static str`
//!
//!   File path relative to the base folder, which is the folder with your manifest
//!   (`Cargo.toml`) by default.
//!
//! See
//! [example](https://github.com/evolutics/iftree/blob/main/examples/basics_standard_fields.rs).
//!
//! ## Name sanitization
//!
//! When generating identifiers based on paths, names are sanitized as follows to
//! ensure they are
//! [valid identifiers](https://doc.rust-lang.org/reference/identifiers.html):
//!
//! - Characters other than ASCII alphanumericals are replaced by `"_"`.
//! - If the first character is numeric, then `"_"` is prepended.
//! - If the name is `"_"`, `"crate"`, `"self"`, `"Self"`, or `"super"`, then `"_"`
//!   is appended.
//!
//! Names are further adjusted to respect naming conventions in the default case:
//!
//! - Lowercase for folders (because they map to module names).
//! - Uppercase for filenames (because they map to static variables).
//!
//! ## Troubleshooting
//!
//! To inspect the generated code, there is a [`debug` configuration](#debug).
//!
//! # Configuration
//!
//! The `iftree::include_file_tree` macro is configured via a
//! [TOML](https://toml.io) string with the following fields.
//!
//! ## `paths`
//!
//! A string with a path pattern per line to filter files.
//!
//! It works like a `.gitignore` file with inverted meaning:
//!
//! - If the last matching pattern is negated (with `!`), the file is excluded.
//! - If the last matching pattern is not negated, the file is included.
//! - If no pattern matches, the file is excluded.
//!
//! The pattern language is as documented in the
//! [`.gitignore` reference](https://git-scm.com/docs/gitignore), with this
//! difference: you must use `a/b/*` instead of `a/b/` to include files in a folder
//! `a/b/`; to also include subfolders (recursively), use `a/b/**`.
//!
//! By default, path patterns are relative to the environment variable
//! `CARGO_MANIFEST_DIR`, which is the folder with your manifest (`Cargo.toml`). See
//! the [`base_folder` configuration](#base_folder) to customize this.
//!
//! To avoid path issues when developing on different platforms, follow these
//! recommendations:
//!
//! - Always use a slash `/` as a folder separator (even on Windows).
//! - Never use backslashes `\` in filenames (even on Linux).
//!
//! This is a **required** option without default.
//!
//! See
//! [example](https://github.com/evolutics/iftree/blob/main/examples/configuration_paths.rs).
//!
//! ## `base_folder`
//!
//! Path patterns are interpreted as relative to this folder.
//!
//! If this path itself is relative, then it is joined to the folder given by the
//! environment variable `CARGO_MANIFEST_DIR`. That is, a relative path `a/b/c` has
//! a full path `[CARGO_MANIFEST_DIR]/[base_folder]/a/b/c`.
//!
//! **Default:** `""`
//!
//! See
//! [example](https://github.com/evolutics/iftree/blob/main/examples/configuration_base_folder.rs).
//!
//! ## `root_folder_variable`
//!
//! The name of the environment variable to use as the root folder for the
//! [`base_folder` configuration](#base_folder).
//!
//! This should be an absolute path.
//!
//! **Default:** `"CARGO_MANIFEST_DIR"`
//!
//! ## `template.initializer`
//!
//! A macro name used to instantiate the asset type per file.
//!
//! As an input, the macro is passed the following arguments, separated by comma:
//!
//! 1. Relative file path as a string literal.
//! 1. Absolute file path as a string literal.
//!
//! As an output, the macro must return a
//! [constant expression](https://doc.rust-lang.org/reference/const_eval.html#constant-expressions).
//!
//! **Default:** A default initializer is constructed by recognizing
//! [standard fields](#standard-fields).
//!
//! See
//! [example](https://github.com/evolutics/iftree/blob/main/examples/configuration_template_initializer.rs).
//!
//! ## `template.identifiers`
//!
//! Whether to generate an identifier per file.
//!
//! Given a file `a/b/my_file`, a `static` variable `base::a::b::MY_FILE` is
//! generated, nested in modules for folders. Their root module is `base`, which
//! represents the base folder.
//!
//! Each variable is a reference to the corresponding element in the `ASSETS` array.
//!
//! Generated identifiers are subject to [name sanitization](#name-sanitization).
//! Because of this, there may be collisions in the generated code, causing an error
//! about a name being defined multiple times. The code generation does not try to
//! resolve such collisions automatically, as this would likely cause confusion
//! about which identifier refers to which file. Instead, you need to manually
//! rename any affected paths.
//!
//! **Default:** `true`
//!
//! See
//! [example](https://github.com/evolutics/iftree/blob/main/examples/configuration_template_identifiers.rs).
//!
//! ## `template` visitors
//!
//! See
//! [example](https://github.com/evolutics/iftree/blob/main/examples/configuration_template_visitors.rs).
//!
//! ## `debug`
//!
//! Whether to generate an identifier `DEBUG` with debug information such as the
//! generated code.
//!
//! **Default:** `false`
//!
//! See
//! [example](https://github.com/evolutics/iftree/blob/main/examples/configuration_debug.rs).
//!
//! # Recipes
//!
//! Here are example solutions for given problems.
//!
//! ## Kinds of asset types
//!
//! - [Type alias](https://github.com/evolutics/iftree/blob/main/examples/basics_type_alias.rs)
//!   (`type X = …`)
//! - [Struct](https://github.com/evolutics/iftree/blob/main/examples/basics_type_named_fields.rs)
//!   (`struct` with named fields)
//! - [Tuple struct](https://github.com/evolutics/iftree/blob/main/examples/basics_type_tuple_fields.rs)
//!   (`struct` with unnamed fields)
//! - [Unit-like struct](https://github.com/evolutics/iftree/blob/main/examples/basics_type_unit.rs)
//!   (`struct` without field list)
//!
//! ## Integration with other libraries
//!
//! - [Actix Web](https://github.com/evolutics/iftree/blob/main/examples/library_actix_web.rs)
//! - [`mime_guess`](https://github.com/evolutics/iftree/blob/main/examples/library_mime_guess.rs)
//! - [`once_cell`](https://github.com/evolutics/iftree/blob/main/examples/library_once_cell.rs)
//!
//! ## Including file metadata
//!
//! - [Filename](https://github.com/evolutics/iftree/blob/main/examples/scenario_filename.rs)
//! - [Filename extension](https://github.com/evolutics/iftree/blob/main/examples/scenario_filename_extension.rs)
//! - [Media type](https://github.com/evolutics/iftree/blob/main/examples/scenario_media_type.rs)
//!   (formerly MIME type)
//!
//! ## Custom constructions
//!
//! - [Hash map](https://github.com/evolutics/iftree/blob/main/examples/scenario_hash_map.rs)
//! - [Nested hash map](https://github.com/evolutics/iftree/blob/main/examples/scenario_nested_hash_map.rs)

mod data;
mod generate_view;
mod go;
mod list_files;
mod model;
mod parse;
mod print;

/// See the [module level documentation](self).
#[proc_macro_attribute]
pub fn include_file_tree(
    parameters: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let configuration = syn::parse_macro_input!(parameters);
    let item2 = proc_macro2::TokenStream::from(item.clone());
    let type_ = syn::parse_macro_input!(item);

    match go::main(configuration, item2, type_) {
        Err(error) => panic!("{}", error),
        Ok(code) => code.into(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn readme_includes_manifest_description() {
        let manifest = get_manifest();
        let description = manifest["package"]["description"].as_str().unwrap();
        let embedded_description = format!("\n\n{}\n\n", description);

        let actual = get_readme().contains(&embedded_description);

        assert!(actual);
    }

    fn get_manifest() -> toml::Value {
        include_str!("../Cargo.toml").parse().unwrap()
    }

    fn get_readme() -> &'static str {
        include_str!("../README.md")
    }

    #[test]
    fn module_documentation_corresponds_to_readme() {
        let mut actual = String::from("# Iftree: Include File Tree 🌳\n\n");
        let mut is_empty = true;
        for line in include_str!("lib.rs").lines() {
            match line.strip_prefix("//!") {
                None => break,
                Some(line) => {
                    let line = line.strip_prefix(' ').unwrap_or(line);
                    let line = if is_empty && line.starts_with('#') {
                        format!("#{}", line)
                    } else if line == "```ignore" {
                        String::from("```rust")
                    } else {
                        String::from(line)
                    };
                    actual.push_str(&format!("{}\n", line));
                    is_empty = line.is_empty();
                }
            }
        }
        let actual = actual;

        let expected = get_readme();
        assert_eq!(actual, expected);
    }

    #[test]
    fn readme_refers_to_current_manifest_version() {
        let manifest = get_manifest();
        let version = manifest["package"]["version"].as_str().unwrap();
        let major_minor_version = version.rsplitn(2, '.').nth(1).unwrap();
        let dependency = format!("`iftree = \"{}\"`", major_minor_version);

        let actual = get_readme().contains(&dependency);

        assert!(actual);
    }
}
