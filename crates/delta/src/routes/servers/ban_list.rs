use futures::future::join_all;
use voxly_database::util::permissions::DatabasePermissionQuery;
use voxly_database::util::reference::Reference;
use voxly_database::{Database, User};
use voxly_models::v0;

use voxly_permissions::{calculate_server_permissions, ChannelPermission};
use voxly_result::Result;
use rocket::serde::json::Json;
use rocket::State;

/// # Fetch Bans
///
/// Fetch all bans on a server.
#[openapi(tag = "Server Members")]
#[get("/<target>/bans")]
pub async fn list(
    db: &State<Database>,
    user: User,
    target: Reference<'_>,
) -> Result<Json<v0::BanListResult>> {
    let server = target.as_server(db).await?;
    let mut query = DatabasePermissionQuery::new(db, &user).server(&server);
    calculate_server_permissions(&mut query)
        .await
        .throw_if_lacking_channel_permission(ChannelPermission::BanMembers)?;

    let bans = db.fetch_bans(&server.id).await?;
    let users = join_all(
        db.fetch_users(
            &bans
                .iter()
                .map(|x| &x.id.user)
                .cloned()
                .collect::<Vec<String>>(),
        )
        .await?
        .into_iter()
        .map(|u| u.into_self(false)),
    )
    .await;

    Ok(Json(v0::BanListResult {
        users: users.into_iter().map(Into::into).collect(),
        bans: bans.into_iter().map(Into::into).collect(),
    }))
}
