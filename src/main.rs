extern crate curl;

use std::env;
use std::ffi::OsString;
use curl::easy::{Easy, List};
use serde_json::{Result, Value};

fn main() {
    let auth_email = String::from("YOUR_EMAIL_ADDRESS");
    let auth_key = String::from("YOUR_GLOBAL_API_KEY");
    let zone = String::from("YOUR_ZONE");

    let args: Vec<OsString> = env::args_os().collect();

    if args.len() == 2 {
        let file = args[1].to_str().unwrap_or("");

        if !file.is_empty() {
            let mut headers = List::new();
            headers.append(&format!("X-Auth-Email: {}", auth_email)).unwrap();
            headers.append(&format!("X-Auth-Key: {}", auth_key)).unwrap();
            headers.append("Content-Type: application/json").unwrap();

            let data = format!("{{\"files\":[\"{}\"]}}", file);
            let mut response_body = Vec::new();

            let mut easy = Easy::new();
            easy.url(&format!("https://api.cloudflare.com/client/v4/zones/{}/purge_cache", zone)).unwrap();
            easy.post(true).unwrap();
            easy.post_fields_copy(data.as_bytes()).unwrap();
            easy.http_headers(headers).unwrap();
            {
                let mut transfer = easy.transfer();
                transfer.write_function(|data| {
                    response_body.extend_from_slice(data);
                    Ok(data.len())
                }).unwrap();
                transfer.perform().unwrap();
            }

            //println!("{}", easy.response_code().unwrap());

            let response_body = String::from_utf8(response_body).expect("Body is not valid UTF8.");

            let json: Value = serde_json::from_str(&response_body).unwrap();

            if json["success"].is_boolean() {
                let s = json["success"].as_bool().unwrap();
                if s {
                    println!("Success");
                } else {
                    println!("Oops, something went wrong.");

                    if json["errors"].is_array() {
                        let errors = json["errors"].as_array().unwrap();

                        for error in errors {
                            if error.is_object() {
                                let error = error.as_object().unwrap();
                                if error["message"].is_string() && error["code"].is_number() {
                                    println!("  -> {} ({})", error["message"].as_str().unwrap(), error["code"]);
                                }
                            }
                        }
                    }
                }
            } else {
                println!("Oops, something went wrong.");
            }
        }
    }
}
