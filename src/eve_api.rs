use std::collections::HashMap;
use reqwest::header::{HeaderMap, AUTHORIZATION};

pub const EVE_CLIENT_ID: & 'static str = "89a236612a8c4e21a25134e324c84667";
pub const EVE_SECRET_KEY: & 'static str = "BfN8hzrLlwJ2RklbHHJ9KWPvQyioHVfIBIkmTW2e";

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenData {
    access_token: String,
    token_type: String,
    expires_in: u16,
    refresh_token: String
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


pub fn do_login(code: String) -> Result <TokenData, String> {
    let mut data = HashMap::new();
    data.insert("grant_type", "authorization_code");
    data.insert("code", code.as_str());

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!(
            "Basic {}",
            base64::encode(
                format!(
                    "{}:{}",
                    EVE_CLIENT_ID,
                    EVE_SECRET_KEY
                ).as_bytes()
            )
        ).as_str().parse().unwrap()
    );

    let request_result = reqwest::blocking::Client::new()
        .post("https://login.eveonline.com/oauth/token")
        .headers(headers)
        .json(& data)
        .send();

        match request_result {
            Ok(response) => {
            if response.status().is_success() {
                match response.json::<TokenData>() {
                    Ok(t) => Ok(t),
                    Err(e) => Err(format!("Decode error: {:?}", e))
                }
            } else {
                match response.json::<TokenError>() {
                    Ok(err) => Err(format!("{}: {}", err.error, err.error_description)),
                    Err(e) => Err(e.to_string())
                }
            }
        }
            ,
        Err(error) => Err(error.to_string())
    }
}