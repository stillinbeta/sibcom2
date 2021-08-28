use std::str::FromStr;

use bmon::html;
use bmon::Value;
use fastly::http::header::{ACCEPT, COOKIE};
use fastly::http::StatusCode;
use fastly::{mime, Request, Response};
use headers::{Cookie, Header};

// fn get_latest(&self, client: &updater::Client) -> Value {
//     let mastodon = match client.get_mastodon() {
//         Ok(status) => Value::Link(status.url, status.message),
//         // TODO: slog
//         Err(err) => {
//             eprintln!("Mastodon error: {:?}", err);
//             Value::String("unknown".into())
//         }
//     };
//     let github = match client.get_commit() {
//         Ok(commit) => Value::Object(vec![
//             (
//                 Value::String("commit".into()),
//                 Value::Link(commit.commit.url, commit.commit.message),
//             ),
//             (
//                 Value::String("repository".into()),
//                 Value::Link(commit.repository.url, commit.repository.name),
//             ),
//         ]),
//         Err(err) => {
//             eprintln!("Github error: {:?}", err);
//             Value::String("unknown".into())
//         }
//     };

//     let location = match client.get_location() {
//         Ok(location) => Value::String(location.position),
//         Err(err) => {
//             eprintln!("Github error: {:?}", err);
//             Value::String("unknown".into())
//         }
//     };

//     Value::Object(vec![
//         (Value::String("toot".into()), mastodon),
//         (Value::String("location".into()), location),
//         (Value::String("push".into()), github),
//     ])

fn send_value(req: &Request, value: &Value, title: &str) -> Response {
    let accept = req
        .get_header(ACCEPT)
        .and_then(|a| a.to_str().ok())
        .and_then(|a| mime::Mime::from_str(a).ok())
        .map(|v| v == mime::APPLICATION_JSON)
        .unwrap_or(false);

    if accept {
        return Response::new()
            .with_status(StatusCode::OK)
            .with_body_json(value)
            .unwrap_or_else(|err| {
                eprintln!("Failed to serialise body: {}", err);
                Response::from_status(StatusCode::INTERNAL_SERVER_ERROR)
                    .with_body("Internal JSON error")
            });
    }

    // get_cookie(req.headers())
    //     .and_then(|c| )
    // req.headers().get(COOKIE)
    // Cookie::parse(req.)
    // .cookies()
    // .get("theme")
    // .and_then(|t| FromStr::from_str(t.value()).ok())
    // .unwrap_or(html::Theme::SolarizedDark);

    let theme = Cookie::decode(&mut req.get_header(COOKIE).into_iter())
        .ok()
        .and_then(|c| c.get("theme").and_then(|v| html::Theme::from_str(v).ok()))
        .unwrap_or(html::Theme::SolarizedDark);

    Response::new()
        .with_status(StatusCode::OK)
        .with_content_type(mime::TEXT_HTML)
        .with_body(html::render_page(title, theme, value))
}

fn make_page_with_nav(body: bmon::Value, nav: bmon::Value, title: &str) -> bmon::Value {
    Value::Object(vec![
        (Value::String("nav".into()), nav),
        (Value::String(title.into()), body),
    ])
}

fn render_homepage(req: &Request, page: bmon::Page) -> Response {
    match page.root {
        Value::Object(map) => {
            let body = map.clone();
            // body.push((Value::String("latest".into()), self.get_latest(client)));

            send_value(
                &req,
                &make_page_with_nav(bmon::Value::Object(body), page.nav, &page.title),
                &page.title,
            )
        }
        _ => {
            eprintln!("Homepage not a string!");
            send_value(req, &page.root, &page.title)
        }
    }
}

fn render_page(req: &Request, page: bmon::Page) -> Response {
    send_value(
        &req,
        &make_page_with_nav(page.root, page.nav, &page.title),
        &page.title,
    )
}

pub fn render(req: &Request, page: bmon::Page) -> Response {
    match page.root {
        bmon::Value::Link(_, s) => Response::redirect(s),
        _ if page.title == "hello" => render_homepage(req, page),
        _ => render_page(req, page),
    }
}
