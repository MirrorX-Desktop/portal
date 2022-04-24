use std::io::Write;
use std::sync::Arc;

use actix_web::{middleware, web, App, HttpServer};
use env_logger::{Builder, Target};
use log::LevelFilter;

mod handler;
mod state;
mod utility;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logger();

    let state = Arc::new(state::State::new()?);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .app_data(web::Data::new(state.clone()))
            // .service(
            //     web::scope("/product")
            //         .route("/version", web::get().to(service::http::product::version))
            //         .route(
            //             "/changelog",
            //             web::get().to(service::http::product::changelog),
            //         )
            //         .route(
            //             "/billboard",
            //             web::get().to(service::http::product::billboard),
            //         ),
            // )
            .service(
                web::scope("/device").route("/register", web::post().to(handler::device::register)),
            )
    })
    .bind("0.0.0.0:40000")?
    .run()
    .await
    .map_err(|err| anyhow::anyhow!(err))
}

fn init_logger() {
    Builder::new()
        .filter_level(LevelFilter::Info)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] [{}({}#{})] {} {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S.%3f"),
                record.module_path().unwrap_or(""),
                record.file().unwrap_or(""),
                record.line().unwrap_or(0),
                record.level(),
                record.args(),
            )
        })
        .target(Target::Stdout)
        .init();
}
