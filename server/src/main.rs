extern crate bmon;
use generator;

mod handler;

use fastly::http::{Method, StatusCode};
use fastly::{Error, Request, Response};

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    if req.get_method() != Method::GET {
        return Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
            .with_body("This method is not allowed"));
    }

    let path = req.get_url().path();
    if path == "/" {
        return Ok(Response::redirect("/hello"));
    }

    let routes = generator::yaml_routes!("site.yaml");

    let resp = routes
        .into_iter()
        .find(|page| page.path == path)
        .map(|page| handler::render(&req, page))
        .unwrap_or_else(|| Response::from_status(StatusCode::NOT_FOUND).with_body("No such page"));

    Ok(resp)
}
