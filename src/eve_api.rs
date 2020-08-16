use std::collections::HashMap;
use reqwest::{
    StatusCode,
    header::{
        HeaderMap, AUTHORIZATION, HeaderValue
    }
};
use serde_derive::{Serialize, Deserialize};
use jsonwebtoken::{decode, DecodingKey};

#[derive(Serialize, Deserialize, Debug)]
pub struct EveOauthCreds {
    pub client_id: String,
    pub secret_key: String,
}

impl  ToString for EveOauthCreds {
    fn to_string(&self) -> String {
        format!("{}:{}", self.client_id, self.secret_key)
    }
}


impl EveOauthCreds {
    fn to_auth_basic(&self) -> String {
        format!(
            "Basic {}",
            base64::encode(self.to_string().as_bytes())
        )
    }
}

#[derive(Debug)]
enum EveOauthCredsError {
    Missing,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenData {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u16,
    pub refresh_token: String
}

#[derive(Serialize)]
struct LoginPostData {
    grant_type: &'static str,
    code: &'static str
}

#[derive(Serialize)]
struct RefreshLoginPostData {
    grant_type: &'static str,
    refresh_token: &'static str
}

#[derive(Serialize, Deserialize, Debug)]
struct TokenError {
    error: String,
    error_description: String
}

pub fn do_login(creds: &EveOauthCreds, code: String) -> Result<serde_json::Value, String> {
    let mut data = HashMap::new();
    data.insert("grant_type", "authorization_code");
    data.insert("code", code.as_str());

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(creds.to_auth_basic().as_str()).unwrap()
    );

    let request_result = reqwest::blocking::Client::new()
        .post("https://login.eveonline.com/oauth/token")
        .headers(headers)
        .form(& data)
        .send();

        match request_result {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json() {
                        Ok(t) => Ok(t),
                        Err(e) => Err(format!("Decode error: {:?}", e))
                    }
                } else {
                    match response.status() {
                        StatusCode::UNAUTHORIZED => Err(String::from("Erro na autenticação do sistema")),
                        _ => match response.json::<TokenError>() {
                            Ok(err) => Err(format!("{}: {}", err.error, err.error_description)),
                            Err(e) => Err(e.to_string())
                        }
                    }
                }
            },
            Err(error) => {
                Err(error.to_string())
            }
    }
}

pub fn validate_token(t: TokenData) -> bool {
    // let key = &DecodingKey::from_rsa_pem(key: &'a [u8]);
    // let val = decode(t.access_token);
    // val.
    // match (t.access_token) {
    //     Ok(td) => println("TOKEN DATA: {:?}", td); true,
    //     Err(_) => false
    // }
    true
}

pub fn get_key_sets() {
    let jwk_set_url: &'static str = "https://login.eveonline.com/oauth/jwks";
    let keys: serde_json::Value = reqwest::blocking::get(jwk_set_url)
        .unwrap()
        .json()
        .unwrap();
    println!("KEYS: {:?}", keys);
}

pub fn get_user_data<T>(t: T) -> T {
    t
}