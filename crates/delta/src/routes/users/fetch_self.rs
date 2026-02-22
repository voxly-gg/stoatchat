use voxly_database::User;
use voxly_models::v0;
use voxly_result::Result;
use rocket::serde::json::Json;

/// # Fetch Self
///
/// Retrieve your user information.
#[openapi(tag = "User Information")]
#[get("/@me")]
pub async fn fetch(user: User) -> Result<Json<v0::User>> {
    Ok(Json(user.into_self(false).await))
}
