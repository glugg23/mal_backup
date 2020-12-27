use dotenv::dotenv;
use mal_backup::request::Request;
use std::env;

fn main() {
    dotenv().ok();

    let authorisation_token =
        env::var("AUTHORISATION_TOKEN").expect("AUTHORISATION_TOKEN must be set");

    let request = Request::new(authorisation_token);
    println!("{}", request.get("/v2/users/@me").unwrap().text().unwrap());
}
