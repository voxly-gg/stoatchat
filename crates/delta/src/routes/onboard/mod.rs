use rocket::Route;

mod complete;
mod hello;

#[derive(OpenApi)]
#[openapi(
    paths(
        hello::hello, complete::complete
    )
)]
pub struct ApiDoc;


pub fn routes() -> Vec<Route> {
    routes![hello::hello, complete::complete]
}
