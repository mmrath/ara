use oxide_auth::endpoint::PreGrant;
use reqwest::header;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::response::content::Html;
use rocket::response::status::Custom;
use rocket::response::Redirect;
use rocket::{Request, Rocket, State};
use std::collections::HashMap;
use std::io::Read;
use std::sync::{Mutex, MutexGuard};

pub struct ClientFairing;

impl Fairing for ClientFairing {
    fn info(&self) -> Info {
        Info {
            name: "Simple oauth client implementation",
            kind: Kind::Attach,
        }
    }

    fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
        Ok(rocket
            .manage(ClientState {
                token: Mutex::new(None),
            })
            .mount(
                "/clientside",
                routes![oauth_endpoint, client_view, client_debug],
            ))
    }
}

struct ClientState {
    token: Mutex<Option<String>>,
}

struct Token<'r> {
    inner: MutexGuard<'r, Option<String>>,
}

#[get("/endpoint?<code>&<error>")]
fn oauth_endpoint(
    code: Option<String>,
    error: Option<String>,
    mut guard: Token,
) -> Result<Redirect, String> {
    if let Some(error) = error {
        return Err(format!("Error during owner authorization: {:?}", error));
    }
    if let Some(code) = code {
        let token = retrieve_token(code)?;
        *guard.inner = Some(token);
        return Ok(Redirect::found("/clientside"));
    }

    Err(format!("Endpoint hit without an authorization code"))
}

#[get("/")]
fn client_view<'r>(guard: Token<'r>) -> Result<Html<String>, Custom<&'static str>> {
    let token = match *guard.inner {
        Some(ref token) => token,
        None => return Err(Custom(Status::PreconditionFailed, "No token retrieved yet")),
    };

    let protected_page =
        retrieve_protected_page(token).unwrap_or_else(|err| format!("Error: {}", err));
    let token = token.replace(",", ",</br>");
    let display_page = format!(
        "<html><style>
            aside{{overflow: auto; word-break: keep-all; white-space: nowrap}}
            main{{text-align: center}}
            main>aside,main>article{{margin: auto; text-align: left; border: 1px solid black; width: 50%}}
        </style>
        <main>
        Used token <aside style>{}</aside> to access
        <a href=\"http://localhost:8020/\">http://localhost:8020/</a>.
        Its contents are:
        <article>{:?}</article>
        </main></html>", token, protected_page);

    Ok(Html(display_page))
}

#[get("/debug")]
fn client_debug(guard: Token) -> String {
    match *guard.inner {
        Some(ref token) => token.to_owned(),
        None => "".to_owned(),
    }
}

fn retrieve_token(code: String) -> Result<String, String> {
    // Construct a request against http://localhost:8020/token, the access token endpoint
    let client = reqwest::Client::new();

    let mut params = HashMap::new();
    params.insert("grant_type", "authorization_code");
    params.insert("client_id", "LocalClient");
    params.insert("code", &code);
    params.insert("redirect_uri", "http://localhost:8000/clientside/endpoint");
    let access_token_request = client
        .post("http://localhost:8000/token")
        .form(&params)
        .build()
        .unwrap();
    let mut token_response = match client.execute(access_token_request) {
        Ok(response) => response,
        Err(_) => return Err("Could not fetch bearer token".into()),
    };

    let mut token = String::new();
    token_response.read_to_string(&mut token).unwrap();
    let token_map: HashMap<String, String> = match serde_json::from_str(&token) {
        Ok(token_map) => token_map,
        Err(err) => {
            return Err(format!(
                "Error unwrapping json response, got {:?} instead",
                err
            ))
        }
    };

    if let Some(err) = token_map.get("error") {
        return Err(format!("Error fetching bearer token: {:?}", err));
    }

    if let Some(token) = token_map.get("access_token") {
        return Ok(token.to_owned());
    }

    Err("Token response neither error nor token".into())
}

fn retrieve_protected_page(token: &str) -> Result<String, String> {
    let client = reqwest::Client::new();

    // Request the page with the oauth token
    let page_request = client
        .get("http://localhost:8000/")
        .header(header::AUTHORIZATION, "Bearer ".to_string() + token)
        .build()
        .unwrap();

    let mut page_response = match client.execute(page_request) {
        Ok(response) => response,
        Err(_) => return Err("Could not access protected resource".into()),
    };

    let mut protected_page = String::new();
    page_response.read_to_string(&mut protected_page).unwrap();
    Ok(protected_page)
}

impl<'a, 'r> FromRequest<'a, 'r> for Token<'r> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        request.guard::<State<'r, ClientState>>().map(|ok| Token {
            inner: ok.inner().token.lock().unwrap(),
        })
    }
}

/// Try to open the server url `http://localhost:8020` in the browser, or print a guiding statement
/// to the console if this is not possible.
pub fn open_in_browser() {
    use std::io::{Error, ErrorKind};
    use std::process::Command;

    let target_addres = "http://localhost:8020/";

    // As suggested by <https://stackoverflow.com/questions/3739327/launching-a-website-via-windows-commandline>
    let open_with = if cfg!(target_os = "linux") {
        // `xdg-open` chosen over `x-www-browser` due to problems with the latter (#25)
        Ok("xdg-open")
    } else if cfg!(target_os = "windows") {
        Ok("explorer")
    } else if cfg!(target_os = "macos") {
        Ok("open")
    } else {
        Err(Error::new(ErrorKind::Other, "Open not supported"))
    };

    open_with
        .and_then(|cmd| Command::new(cmd).arg(target_addres).status())
        .and_then(|status| {
            if status.success() {
                Ok(())
            } else {
                Err(Error::new(ErrorKind::Other, "Non zero status"))
            }
        })
        .unwrap_or_else(|_| println!("Please navigate to {}", target_addres));
}

pub fn consent_page_html(route: &str, grant: &PreGrant) -> String {
    macro_rules! template {
        () => {
"<html>'{0:}' (at {1:}) is requesting permission for '{2:}'
<form method=\"post\">
    <input type=\"submit\" value=\"Accept\" formaction=\"{4:}?response_type=code&client_id={3:}&allow=true\">
    <input type=\"submit\" value=\"Deny\" formaction=\"{4:}?response_type=code&client_id={3:}\">
</form>
</html>"
        };
    }

    format!(
        template!(),
        grant.client_id, grant.redirect_uri, grant.scope, grant.client_id, &route
    )
}
