use rocket::Route;

mod invite_delete;
mod invite_fetch;
mod invite_join;

#[derive(OpenApi)]
#[openapi(
    paths(
        invite_fetch::fetch,
        invite_join::join,
        invite_delete::delete
    )
)]
pub struct ApiDoc;


pub fn routes() -> Vec<Route> {
    routes![
        invite_fetch::fetch,
        invite_join::join,
        invite_delete::delete
    ]
}
