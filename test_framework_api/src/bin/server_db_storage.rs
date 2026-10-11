use qr_pake_protocol_executors::{Storage, parse_server_cli_args, run};
use qr_pake_protocols::{KemAeServer, OcakeServer, PakeProtocol, TkServer};
use std::sync::Arc;
use test_framework_api::other_storage::DbStorage;

#[tokio::main]
async fn main() {
    // Get parsed args
    let (pake_protocol, ip_address, port, login_threshold, login_window, timeout) =
        parse_server_cli_args();

    let path_to_db: String = "../protocol_state.db".to_string();

    match pake_protocol {
        PakeProtocol::KemAePake => {
            // Create the SQLite database storage backend
            let storage =
                Arc::new(DbStorage::new(path_to_db)) as Arc<dyn Storage<KemAeServer> + Send + Sync>;

            run::<KemAeServer>(
                storage,
                &ip_address,
                &port,
                login_threshold,
                login_window,
                timeout,
            )
            .await
        }
        PakeProtocol::OcakePake => {
            // Create the SQLite database storage backend
            let storage =
                Arc::new(DbStorage::new(path_to_db)) as Arc<dyn Storage<OcakeServer> + Send + Sync>;

            run::<OcakeServer>(
                storage,
                &ip_address,
                &port,
                login_threshold,
                login_window,
                timeout,
            )
            .await
        }
        PakeProtocol::TkPake => {
            // Create the SQLite database storage backend
            let storage =
                Arc::new(DbStorage::new(path_to_db)) as Arc<dyn Storage<TkServer> + Send + Sync>;

            run::<TkServer>(
                storage,
                &ip_address,
                &port,
                login_threshold,
                login_window,
                timeout,
            )
            .await
        }
    }
}
