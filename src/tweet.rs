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
const TWITTER_API_UPLOAD_URL: &'static str = "https://upload.twitter.com/1.1/media/upload.json";
const TWITTER_API_TWEET_URL: &'static str = "https://api.twitter.com/1.1/statuses/update.json";

#[derive(Debug)]
struct Creds {
    consumer_key: String,
    consumer_secret: String,
    access_key: String,
    access_secret: String,
}

impl Creds {
    fn next_or_err(lines: &mut Iterator<Item=&str>) -> Result<String, Box<Error>> {
        lines.next().map(|l| String::from(l)).ok_or(From::from("Bad credentials file"))
    }

    fn load() -> Result<Self, Box<Error>> {
        let path = Path::new(CRED_FNAME);
        let mut file = File::open(&path)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        let mut lines = s.lines();
        let consumer_key    = Self::next_or_err(&mut lines)?;
        let consumer_secret = Self::next_or_err(&mut lines)?;
        let access_key      = Self::next_or_err(&mut lines)?;
        let access_secret   = Self::next_or_err(&mut lines)?;
        Ok(Creds {
            consumer_key: consumer_key,
            consumer_secret: consumer_secret,
            access_key: access_key,
            access_secret: access_secret,
        })
    }
}

type MediaId = u64;

#[derive(RustcDecodable)]
struct TwitterUploadResponse {
    media_id: MediaId
}

fn upload_image(consumer: &Token, access: &Token, filename: &str) -> Result<MediaId, Box<Error>> {
    let mut file = File::open(filename)?;
    let mut buf: Vec<u8> = vec![];
    file.read_to_end(&mut buf)?;

    let base64_encoded_image: String = buf.to_base64(Config {
        char_set: CharacterSet::Standard,
        newline: Newline::LF,
        pad: true,
        line_length: None
    });

    let mut param = HashMap::new();
    param.insert("media_data".into(), base64_encoded_image.into());
    let result = oauth::post(TWITTER_API_UPLOAD_URL, consumer, Some(access), Some(&param))?;
    let response: TwitterUploadResponse = json::decode(String::from_utf8(result)?.as_str())?;
    Ok(response.media_id)
}

pub fn post_tweet(consumer: &Token, access: &Token, status: &str, filename: &str) -> Result<(), Box<Error>> {
    // Posting a tweet with an image takes two api calls, one to upload the image (which gives us
    // back an identifier for the image) and one to post the tweet (which includes the identifier)
    let media_id = upload_image(consumer, access, filename)?;
    let mut parameters = HashMap::new();
    parameters.insert("status".into(), status.into());
    parameters.insert("media_ids".into(), format!("{}", media_id).into());
    oauth::post(TWITTER_API_TWEET_URL, consumer, Some(access), Some(&parameters))?;
    Ok(())
}

pub fn tweet(alg: &str, image_filename: &str) -> Result<(), Box<Error>> {
    let creds = Creds::load()?;

    let consumer = Token::new(creds.consumer_key, creds.consumer_secret);
    let access = Token::new(creds.access_key, creds.access_secret);
    post_tweet(&consumer, &access, alg, image_filename)?;

    Ok(())
}
