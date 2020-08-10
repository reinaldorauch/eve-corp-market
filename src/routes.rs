use rocket_contrib::templates::{Template, };
use rocket::{
    Request,
    response::Redirect,
    request::{
        FromRequest,
        Outcome
    },
    http::Status,
};

use eve_api;

#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
    redirect_uri: &'static str,
    client_id: &'static str,
    // This key tells handlebars which template is the parent.
    parent: &'static str,
}

#[derive(Serialize)]
struct SellOrder {}

#[derive(Serialize)]
struct BuyOrder {}

#[derive(Serialize)]
struct User {
    user_id: u32,
    token: eve_api::TokenData,
}

#[derive(Debug)]
enum AuthError {
    NotLogged,
    Expired
}

impl <'a, 'r> FromRequest<'a, 'r> for User {
    type Error = AuthError;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();

        // Auth logic

        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, AuthError::NotLogged)),
            1 if token_valid(keys[0]) => {
                Outcome::Success(get_user_from_token(keys[0]))
            },
            1 => Outcome::Failure((Status::BadRequest, AuthError::Expired))
        }
    }
}

fn token_valid(t: &str) -> bool {
    true
}

fn get_user_from_token(t: &str) -> User {
    let at: String = String::from("cool");
    User {
        user_id: 1,
        token: {
            access_token: at,
            token_type: String::from("Beaerer"),
            expire_in: 100,
            refresh_token: String::from("HOW")
        }
    }
}

#[derive(Serialize)]
struct MarketContext {
    buying_orders: Vec<BuyOrder>,
    selling_orders: Vec<SellOrder>,
}

#[get("/")]
pub fn index() -> Template {
    Template::render("index", &TemplateContext {
        title: "index",
        redirect_uri: "http://localhost:8000/oauth-login",
        client_id: eve_api::EVE_CLIENT_ID,
        parent: "layout"
    })
}

#[get("/oauth-login?<code>")]
pub fn oauth_login(code: String) -> Redirect {
    let res = eve_api::do_login(code);
    println!("{:?}", res);
    Redirect::to("/market")
}

#[get("/market")]
pub fn market(user: User) -> Template {
    Template::render("market", &MarketContext {
        selling_orders: vec![],
        buying_orders: vec![]
    })
}

#[catch(404)]
pub fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}