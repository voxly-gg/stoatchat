use rocket::Route;

mod emoji_create;
mod emoji_delete;
mod emoji_fetch;

#[derive(OpenApi)]
#[openapi(
    paths(
        emoji_create::create_emoji,
        emoji_delete::delete_emoji,
        emoji_fetch::fetch_emoji
    )
)]
pub struct ApiDoc;

pub fn routes() -> Vec<Route> {
    routes![
        emoji_create::create_emoji,
        emoji_delete::delete_emoji,
        emoji_fetch::fetch_emoji
    ]
}
