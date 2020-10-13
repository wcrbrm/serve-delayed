use actix_web::{ Error, HttpRequest };

mod server {
    pub fn get_bind_address() -> String {
        std::env::var("BIND_ADDR")
            .ok()
            .unwrap_or("0.0.0.0:5000".to_owned())
    }

    pub fn get_web_root() -> String {
        std::env::var("WEB_ROOT")
            .ok()
            .unwrap_or("./".to_owned())
    }

    pub fn get_cors_factory() -> actix_cors::CorsFactory {
        actix_cors::Cors::new()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::ACCEPT,
            ])
            .max_age(36000)
            .finish()
    }
}

async fn index(req: HttpRequest) -> Result<actix_files::NamedFile, Error> {
    let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();
    let path_str: String = path.into_os_string().into_string().unwrap();
    let src: String = server::get_web_root()
        + "/"
        + &(if path_str == "" {
            String::from("index.html")
        } else {
            path_str
        });
    let fallback: String = server::get_web_root() + "/index.html";
    let found: bool = std::path::Path::new(&src).exists();
    let file = actix_files::NamedFile::open(if found { &src } else { &fallback })?;
    Ok(file.use_last_modified(true))
}

async fn delayed(req: HttpRequest) -> Result<actix_files::NamedFile, Error> {
    let delay_millis: u64 = req.match_info().query("delay").parse::<u64>().unwrap();
    let duration = std::time::Duration::from_millis(delay_millis);
    println!("sleeping {:?}", duration);
    std::thread::sleep(duration);

    let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();
    let path_str: String = path.into_os_string().into_string().unwrap();
    let src: String = server::get_web_root()
        + "/"
        + &(if path_str == "" {
            String::from("index.html")
        } else {
            path_str
        });
    let fallback: String = server::get_web_root() + "/index.html";
    let found: bool = std::path::Path::new(&src).exists();
    let file = actix_files::NamedFile::open(if found { &src } else { &fallback })?;
    Ok(file.use_last_modified(true))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    println!(
        "Starting server at http://{}/, static files root at {}",
        server::get_bind_address(),
        server::get_web_root()
    );
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(server::get_cors_factory())
            .route("/delay/{delay:.*}/{filename:.*}", actix_web::web::get().to(delayed))
            .route("/{filename:.*}", actix_web::web::get().to(index))
    })
    // .workers(1)
    .bind(server::get_bind_address())?
    .run()
    .await
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//     }
// }
