use rocket::Route;

mod get_settings;
mod get_unreads;
mod set_settings;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_settings::fetch,
        set_settings::set,
        get_unreads::unreads
    )
)]
pub struct ApiDoc;

pub fn routes() -> Vec<Route> {
    routes![get_settings::fetch, set_settings::set, get_unreads::unreads]
}
