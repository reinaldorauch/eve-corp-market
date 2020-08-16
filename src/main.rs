#![feature(proc_macro_hygiene, decl_macro)]

#[cfg(test)] mod tests;

use rocket::routes;
use rocket::catchers;
use rocket::fairing::AdHoc;
use rocket_contrib::templates::{Template};
use rocket_contrib::serve::StaticFiles;

mod eve_api;
mod helpers;
mod router;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![router::index, router::oauth_login, router::market])
        .mount("/assets", StaticFiles::from("assets"))
        .register(catchers![router::not_found])
        .attach(AdHoc::on_attach("Eve Config", |rocket| {
            let cfg = rocket.config();
            let cid = cfg.get_str("eve_client_id").unwrap().to_string();
            let sk = cfg.get_str("eve_secret_key").unwrap().to_string();
            let creds = eve_api::EveOauthCreds {
                client_id: cid,
                secret_key: sk
            };
            Ok(rocket.manage(creds))
        }))
        .attach(Template::custom(|engines| {
            engines.handlebars.register_helper("url_encode", Box::new(helpers::url_encode));
            engines.handlebars.register_helper("json", Box::new(helpers::json));
        }))
}

fn main() {
    rocket().launch();
}
