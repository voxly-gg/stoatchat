use std::time::Duration;

use log::warn;
use voxly_database::Database;
use voxly_result::Result;
use tokio::time::sleep;

pub async fn task(db: Database) -> Result<()> {
    loop {
        let success = db.remove_dangling_members().await;
        if let Err(s) = success {
            voxly_config::capture_error(&s);
            warn!("Failed to prune dangling members: {:?}", &s);
        }

        sleep(Duration::from_secs(90)).await;
    }
}
