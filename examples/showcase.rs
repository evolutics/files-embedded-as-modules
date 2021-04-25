// A simple web server that serves files.

use actix_web::web;
use once_cell::sync;
use std::io;

// This macro is used as an initializer below.
// Due to macro evaluation, it needs to be defined first.
macro_rules! initialize {
    ($relative_path:literal, $absolute_path:literal) => {
        Asset {
            path: $relative_path,

            // As this must be a constant expression,
            // we use `once_cell` to compute non-constant data (lazily).
            // Here we guess the media (MIME) type based on the file path.
            media_type: once_cell::sync::Lazy::new(|| {
                let media_type = mime_guess::from_path($relative_path).first_or_octet_stream();
                String::from(media_type.essence_str())
            }),

            contents: include_str!($absolute_path),
        }
    };
}

#[iftree::include_file_tree(
// The following string literal is the configuration in TOML format.
    "
# Select files relative to this folder.
base_folder = 'examples/assets'

# Filter the files as follows (one pattern per line):
# - Include any files in the base folder and its subfolders.
# - But exclude hidden files or folders.
paths = '''
**
!.*
'''

# For each selected file, call the macro above.
initializer = 'initialize'
"
)]
pub struct Asset {
    path: &'static str,
    media_type: sync::Lazy<String>,
    contents: &'static str,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let socket_address = "127.0.0.1:8080";

    print_index(socket_address);

    actix_web::HttpServer::new(|| actix_web::App::new().route("/{_:.*}", web::get().to(get_asset)))
        .bind(socket_address)?
        .run()
        .await
}

fn print_index(socket_address: &str) {
    // `ASSETS` is the generated array.
    for asset in &ASSETS {
        eprintln!("See: http://{}/{}", socket_address, asset.path);
    }
}

async fn get_asset(path: web::Path<String>) -> impl actix_web::Responder {
    let path = path.into_inner();
    // Use the URL path directly as file path.
    // `ASSETS` is generated in order of relative paths, hence a binary search.
    match ASSETS.binary_search_by(|asset| asset.path.cmp(&path)) {
        Err(_) => actix_web::HttpResponse::NotFound().finish(),
        Ok(index) => {
            let asset = &ASSETS[index];
            actix_web::HttpResponse::Ok()
                .content_type(&*asset.media_type)
                .body(asset.contents)
        }
    }
}
