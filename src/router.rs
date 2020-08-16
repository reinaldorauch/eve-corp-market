use serde_derive::Serialize;
use rocket::{
    Request,
    State,
    request::{
        FromRequest,
        Outcome

    },
    get,
    catch,
    http::Status,
};
use rocket_contrib::templates::Template;

use super::eve_api::EveOauthCreds;

#[derive(Serialize)]
struct TemplateContext {
    title: String,
    redirect_uri: String,
    client_id: String,
    user: Option<User>,
    // This key tells handlebars which template is the parent.
    parent: String,
}

#[derive(Serialize)]
struct SellOrder {}

#[derive(Serialize)]
struct BuyOrder {}

#[derive(Serialize)]
pub struct User {
    user_id: u32,
    token: super::eve_api::TokenData,
}

#[derive(Debug)]
pub enum AuthError {
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
            _ => Outcome::Failure((Status::BadRequest, AuthError::Expired)),
        }
    }
}

fn token_valid(t: &str) -> bool {
    true
}

fn get_user_from_token(t: &str) -> User {
    User {
        user_id: 1,
        token: super::eve_api::TokenData {
            access_token: String::from("FOOOOo"),
            token_type: String::from("Beaerer"),
            expires_in: 100,
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
pub fn index<'a>(user: Option<User>, creds: State<'a, EveOauthCreds>) -> Template {
    Template::render("index", &TemplateContext {
        title: String::from("index"),
        redirect_uri: String::from("http://localhost:8000/oauth-login"),
        client_id: (*creds.client_id).to_string(),
        user: user,
        parent: String::from("layout")
    })
}

#[get("/oauth-login?<code>")]
pub fn oauth_login<'a>(creds: State<'a, EveOauthCreds>, code: String) -> Result<Template, String> {
    let res = super::eve_api::do_login(creds.inner(), code);
    super::eve_api::get_key_sets();
    match res {
        Ok(t) => {
            let d = super::eve_api::get_user_data(t);
            Ok(Template::render("user", d))
        },
        Err(e) => Err(format!("LOGIN Failed: {:?}", e))
    }
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