use serde::*;
use worker::*;

mod utils;

const GITHUB_URL: &str = "https://github.com";
const AUTH_PATH: &str = "/login/oauth/authorize";
const TOKEN_PATH: &str = "/login/oauth/access_token";

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseType {
    access_token: String,
    scope: String,
    token_type: String,
}

/// Redirect to GitHub URL using clinet ID.
/// After getting token, GitHub will redirect to `/api/gh-auth/callback`.
///
/// https://docs.github.com/en/developers/apps/building-oauth-apps/authorizing-oauth-apps#1-request-a-users-github-identity
fn get_code(env: Env) -> Result<Response> {
    // Get environment variable.
    let client_id = match env.secret("GITHUB_CLIENT_ID") {
        Ok(id) => id,
        Err(_) => return Err(Error::Internal("GITHUB_CLIENT_ID not set".into())),
    }
    .to_string();

    let url = format!("{}{}?client_id={}", GITHUB_URL, AUTH_PATH, client_id);

    let mut headers = Headers::new();
    // Location to redirect.
    headers.set("Location", url.as_str())?;
    let response = Response::empty()?
        .with_status(302) // 302 FOUND
        .with_headers(headers);

    Ok(response)
}

/// Exchange OAuth token.
///
/// https://docs.github.com/en/developers/apps/building-oauth-apps/authorizing-oauth-apps#2-users-are-redirected-back-to-your-site-by-github
async fn exchange_token(req: Request, env: Env) -> Result<Response> {
    let (client_id, client_secret, callback_url) = match (
        env.secret("GITHUB_CLIENT_ID"),
        env.secret("GITHUB_CLIENT_SECRET"),
        env.secret("CALLBACK_URL"),
    ) {
        (Ok(id), Ok(secret), Ok(url)) => (id.to_string(), secret.to_string(), url.to_string()),
        _ => return Err(Error::Internal("Missing secrets".into())),
    };

    // Get `code` query.
    let url = req.url()?;
    let mut code = None;
    for (key, value) in url.query_pairs() {
        if key == "code" {
            code = Some(value);
            break;
        }
    }
    let code = match code {
        Some(code) => code,
        None => return Response::error("Missing code", 401),
    };

    // Request a token.
    let mut request = Request::new(
        format!(
            "{}{}?client_id={}&client_secret={}&code={}",
            GITHUB_URL, TOKEN_PATH, client_id, client_secret, code
        )
        .as_str(),
        Method::Post,
    )?;
    let headers = request.headers_mut()?;
    headers.set("Accept", "application/json")?;
    headers.set("User-Agent", "GitActivity 1.0")?;
    let mut response = Fetch::Request(request).send().await?;

    // Deserialize response.
    let ResponseType {
        access_token,
        token_type: _,
        scope: _,
    } = response.json().await?;

    // Redirect to a success page.
    let mut headers = Headers::new();
    headers.set(
        "Location",
        format!("{}?token={}", callback_url, access_token).as_str(),
    )?;
    let response = Response::empty()?
        .with_status(302) // 302 FOUND
        .with_headers(headers);

    Ok(response)
}

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    let router = Router::new(());

    router
        .get("/", |_, ctx| get_code(ctx.get_env()))
        .get_async("/redirect", |req, ctx| async move {
            exchange_token(req, ctx.get_env()).await
        })
        .run(req, env)
        .await
}
