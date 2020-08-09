#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;
extern crate percent_encoding;
extern crate serde;
extern crate base64;
extern crate reqwest;

#[cfg(test)] mod tests;

use rocket_contrib::templates::{Template};
use rocket_contrib::serve::StaticFiles;

mod helpers;
mod routes;
mod eve_api;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![routes::index, routes::oauth_login])
        .mount("/assets", StaticFiles::from("assets"))
        .register(catchers![routes::not_found])
        .attach(Template::custom(|engines| {
            engines.handlebars.register_helper("url_encode", Box::new(helpers::url_encode));
        }))
}

fn main() {
    rocket().launch();
}
