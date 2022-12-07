use std::error::{Error};
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use cookie::{Cookie};
use hyper::{Body, body, Client, header, Method, StatusCode};

pub async fn fetch(day: i32, session: &str) -> Result<bool, Box<dyn Error>> {
    use hyper_tls::HttpsConnector;

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);
    let uri: hyper::Uri = format!("https://adventofcode.com/2022/day/{}/input", day).parse()?;

    let cookie = Cookie::new("session", session);
    let req = hyper::Request::builder().
        uri(uri)
        .method(Method::GET)
        .header(header::COOKIE, cookie.to_string())
        .body(Body::empty())?;

    let mut res = client.request(req).await?;
    if res.status() != StatusCode::OK {
        return Err(
            Box::from(std::io::Error::new(
                ErrorKind::Other,
                format!("Invalid status code {} received", res.status())
            ))
        )
    }

    let body = res.body_mut();

    // Create dir if it doesn't exist
    let day_dir = format!("./resources/day{:02}", day);
    match fs::metadata(&day_dir) {
        Ok(m) => {
            if !m.is_dir() {
                return Err(
                    Box::from(std::io::Error::new(
                        ErrorKind::Other,
                        format!("{} is not a directory", &day_dir)
                    ))
                )
            }
        }
        Err(_) => {
            // Create dir
            fs::create_dir_all(&day_dir)?;
        }
    }

    // Save file
    let input_txt = format!("{}/{}", day_dir, "input.txt");
    let text_path = Path::new(&input_txt);
    fs::write(text_path, body::to_bytes(body).await?)?;

    return Ok(true)
}