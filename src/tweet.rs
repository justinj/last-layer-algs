use std::error::Error;
use std::io::Read;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

use oauth::Token;
use oauth;
use rustc_serialize::base64::{ToBase64, Config, CharacterSet, Newline};
use rustc_serialize::json;

const CRED_FNAME: &'static str = "creds";

#[derive(Debug)]
struct Creds {
    consumer_key: String,
    consumer_secret: String,
    access_key: String,
    access_secret: String,
}

impl Creds {
    fn load() -> Result<Self, Box<Error>> {
        let path = Path::new(CRED_FNAME);
        let mut file = File::open(&path)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        let mut lines = s.lines();
        let consumer_key    = lines.next();
        let consumer_secret = lines.next();
        let access_key      = lines.next();
        let access_secret   = lines.next();
        match (consumer_key, consumer_secret, access_key, access_secret) {
            (Some(consumer_key), Some(consumer_secret),
             Some(access_key),   Some(access_secret)) =>
                Ok(Creds {
                    consumer_key: String::from(consumer_key),
                    consumer_secret: String::from(consumer_secret),
                    access_key: String::from(access_key),
                    access_secret: String::from(access_secret),
                }),
            _ => Err(From::from("Bad credentials file"))
        }
    }
}

#[derive(RustcDecodable)]
struct TwitterUploadResponse {
    media_id: u64
}

pub fn update_status(consumer: &Token, access: &Token, status: &str) -> Result<(), Box<Error>> {
    let mut file = File::open("output_file.png")?;
    let mut buf: Vec<u8> = vec![];
    file.read_to_end(&mut buf)?;
    let image: String = buf.to_base64(Config {
        char_set: CharacterSet::Standard,
        newline: Newline::LF,
        pad: true,
        line_length: None
    });

    let mut param = HashMap::new();
    let _ = param.insert("media_data".into(), image.into());
    let result = oauth::post("https://upload.twitter.com/1.1/media/upload.json",
                             consumer,
                             Some(access),
                             Some(&param))?;

    let response: TwitterUploadResponse = json::decode(String::from_utf8(result)?.as_str())?;
    println!("response was {:?}", response.media_id);

    let mut param = HashMap::new();
    // let _ = param.insert("status".into(), status.into());
    let _ = param.insert("status".into(), status.into());
    let _ = param.insert("media_ids".into(), format!("{}", response.media_id).into());
    let _ = oauth::post("https://api.twitter.com/1.1/statuses/update.json",
                        consumer,
                        Some(access),
                        Some(&param))?;
    Ok(())
}

pub fn tweet(alg: &str) -> Result<(), Box<Error>> {
    let creds = Creds::load()?;

    let consumer = Token::new(creds.consumer_key, creds.consumer_secret);
    let access = Token::new(creds.access_key, creds.access_secret);

    update_status(&consumer, &access, alg)?;

    Ok(())
}
