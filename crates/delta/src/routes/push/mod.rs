use rocket::Route;

mod subscribe;
mod unsubscribe;

#[derive(OpenApi)]
#[openapi(
    paths(
        subscribe::subscribe,
        unsubscribe::unsubscribe
    )
)]
pub struct ApiDoc;

pub fn routes() -> Vec<Route> {
    routes![subscribe::subscribe, unsubscribe::unsubscribe]
}
