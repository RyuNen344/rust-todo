#[macro_use]
extern crate actix_web;

use std::{env, io};

use actix_session::{CookieSession, Session};
use actix_web::{App, guard, HttpRequest, HttpResponse, HttpServer, middleware, Result, web};
use actix_web::http::StatusCode;
use env_logger;
use log::{debug, info};

#[get("/welcome")]
fn welcome(session: Session, req: HttpRequest) -> Result<HttpResponse> {
    info!("start");
    debug!("{:?}", req);

    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        debug!("session value: {}", count);
        counter = count + 1;
    }

    session.set("counter", counter)?;

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("this is test"))
}

fn p404() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::NOT_FOUND)
        .content_type("text/html; charset=utf-8")
        .body("404"))
}

fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    info!("application start");

    let sys = actix_rt::System::new("rust-todo");

    HttpServer::new(|| {
        App::new()
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .wrap(middleware::Logger::default())
            .service(welcome)
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get().to(p404))
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )
    })
        .bind("127.0.0.1:8080")?
        .start();
    sys.run()
}
