use worker::*;

use path::get_queries;

mod path;
mod utils;

const GITHUB_URL: &str = "https://github.com";
const AUTH_PATH: &str = "/login/oauth/access_token";

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

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    let (client_id, client_secret, callback_url) = match (
        env.secret("CLIENT_ID"),
        env.secret("CLIENT_SECRET"),
        env.secret("CALLBACK_URL"),
    ) {
        (Ok(client_id), Ok(client_secret), Ok(callback_url)) => {
            (client_id, client_secret, callback_url)
        }
        _ => return Err(Error::Internal("Missing secrets".into())),
    };
    let path = req.path();
    let queries = get_queries(&path);
    let code = match queries.get("code") {
        Some(Some(c)) => *c,
        _ => return Err(Error::Internal("Missing code".into())),
    };

    let mut request = Request::new(
        format!(
            "{}{}?client_id={}&client_secret={}&code={}",
            GITHUB_URL,
            AUTH_PATH,
            client_id.to_string(),
            client_secret.to_string(),
            code
        )
        .as_str(),
        Method::Post,
    )?;
    let headers = request.headers_mut()?;
    headers.set("Accept", "application/json")?;
    headers.set("User-Agent", "GitActivity 1.0")?;
    let response = Fetch::Request(request).send().await?.text().await?;

    let mut headers = Headers::new();
    headers.set(
        "Location",
        format!("{}?token={}", callback_url.to_string(), response).as_str(),
    )?;
    let response = Response::empty()?.with_status(302).with_headers(headers);

    Ok(response)
}
