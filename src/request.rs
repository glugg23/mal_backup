use reqwest::blocking::{Client, Response};
use reqwest::Error;
use std::fmt::Write;

pub struct Request {
    client: Client,
    authorisation_token: String,
    domain: String,
}

impl Request {
    pub fn new(authorisation_token: String) -> Self {
        Self {
            client: Client::new(),
            authorisation_token,
            domain: "https://api.myanimelist.net".into(),
        }
    }

    pub fn get(self, path: &str) -> Result<Response, Error> {
        let mut url = String::new();
        write!(url, "{}{}", self.domain, path).unwrap();

        self.client
            .get(&url)
            .bearer_auth(self.authorisation_token)
            .send()
    }
}
