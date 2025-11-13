use rocket::Route;

mod acknowledge_policy_changes;

#[derive(OpenApi)]
#[openapi(
    paths(
        acknowledge_policy_changes::acknowledge_policy_changes
    )
)]
pub struct ApiDoc;


pub fn routes() -> Vec<Route> {
    routes![
        // Policy
        acknowledge_policy_changes::acknowledge_policy_changes,
    ]
}
