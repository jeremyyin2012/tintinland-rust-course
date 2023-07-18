#![feature(error_generic_member_access)]
#![feature(provide_any)]
#![feature(tuple_trait)]
#![feature(proc_macro_hygiene, decl_macro)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate core;
extern crate dotenv;
#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use rocket_okapi::openapi_get_routes;
use rocket_okapi::rapidoc::GeneralConfig;
use rocket_okapi::rapidoc::make_rapidoc;
use rocket_okapi::rapidoc::RapiDocConfig;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::make_swagger_ui;
use rocket_okapi::swagger_ui::SwaggerUIConfig;

use crate::business::Business;
use crate::store::Store;

mod business;
mod conf;
mod model;
mod resp;
mod route;
mod services;
mod store;
mod utils;

pub fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "/openapi.json".to_string(),
        ..Default::default()
    }
}

pub fn get_rapidoc() -> RapiDocConfig {
    RapiDocConfig {
        general: GeneralConfig {
            spec_urls: vec![UrlObject::new("General", "/openapi.json")],
            ..Default::default()
        },
        ..Default::default()
    }
}


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();
    let routes = openapi_get_routes![
        route::index,
        route::favicon,
        route::hello,
        route::ping,
        route::api_ping,
        route::api_ping_pgsql,
        route::api_ping_redis,
        route::api::get_class_list,
        route::api::get_class_info,
        route::api::get_student_list,
        route::api::get_student_info,
    ];
    let store = Store::new().await;
    let sentry_dsn = store.config.sentry_dsn.clone();
    let app_env = store.config.app_env.clone();
    let biz = Business::new(store);
    let res = biz
        .ping
        .do_ping()
        .await
        .expect("database or cache ping error");
    println!("ping: {}", res);
    let _guard = sentry::init((
        sentry_dsn,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            environment: Some(app_env.into()),
            send_default_pii: true,
            ..Default::default()
        },
    ));
    let _rocket = rocket::build()
        .manage(biz)
        .mount("/", routes)
        .mount("/api/swagger", make_swagger_ui(&get_docs()))
        .mount("/api/rapidoc", make_rapidoc(&get_rapidoc()))
        .launch()
        .await?;

    Ok(())
}
