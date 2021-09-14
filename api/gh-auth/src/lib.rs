use worker::*;

const GITHUB_URL: &str = "https://github.com";
const AUTH_PATH: &str = "/login/oauth/authorize";

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    log_request(&req);

    let client_id = match env.secret("GITHUB_CLIENT_ID") {
        Ok(id) => id,
        Err(_) => return Err(Error::Internal("GITHUB_CLIENT_ID not set".into())),
    };

    let url = format!(
        "{}{}?client_id={}",
        GITHUB_URL,
        AUTH_PATH,
        client_id.to_string()
    );

    let mut headers = Headers::new();
    headers.set("Location", url.as_str())?;
    let response = Response::empty()?.with_status(302).with_headers(headers);

    Ok(response)
}
