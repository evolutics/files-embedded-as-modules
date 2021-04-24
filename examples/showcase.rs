use actix_web::web;
use once_cell::sync;
use std::io;

macro_rules! initialize {
    ($relative_path:literal, $absolute_path:literal) => {
        Asset {
            path: $relative_path,
            media_type: once_cell::sync::Lazy::new(|| {
                let media_type = mime_guess::from_path($relative_path).first_or_octet_stream();
                String::from(media_type.essence_str())
            }),
            content: include_str!($absolute_path),
        }
    };
}

#[iftree::include_file_tree(
    "
paths = '''
**
!.*
'''

base_folder = 'examples/assets'
initializer = 'initialize'
"
)]
pub struct Asset {
    path: &'static str,
    media_type: sync::Lazy<String>,
    content: &'static str,
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
    for asset in &ASSETS {
        eprintln!("See: http://{}/{}", socket_address, asset.path);
    }
}

async fn get_asset(path: web::Path<String>) -> impl actix_web::Responder {
    let path = path.into_inner();
    match ASSETS.binary_search_by(|asset| asset.path.cmp(&path)) {
        Err(_) => actix_web::HttpResponse::NotFound().finish(),
        Ok(index) => {
            let asset = &ASSETS[index];
            actix_web::HttpResponse::Ok()
                .content_type(&*asset.media_type)
                .body(asset.content)
        }
    }
}
