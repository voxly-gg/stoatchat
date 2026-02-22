use voxly_config::configure;
use voxly_database::DatabaseInfo;
use voxly_result::Result;
use tasks::{file_deletion, prune_dangling_files, prune_members};
use tokio::try_join;

pub mod tasks;

#[tokio::main]
async fn main() -> Result<()> {
    configure!(crond);

    let db = DatabaseInfo::Auto.connect().await.expect("database");
    try_join!(
        file_deletion::task(db.clone()),
        prune_dangling_files::task(db.clone()),
        prune_members::task(db.clone())
    )
    .map(|_| ())
}
