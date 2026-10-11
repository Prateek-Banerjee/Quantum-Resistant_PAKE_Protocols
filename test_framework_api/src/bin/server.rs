use qr_pake_protocol_executors::{DefaultStorage, Storage, parse_server_cli_args, run};
use qr_pake_protocols::{KemAeServer, OcakeServer, PakeProtocol, TkServer};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Get parsed args
    let (pake_protocol, ip_address, port, login_threshold, login_window, timeout) =
        parse_server_cli_args();

    match pake_protocol {
        PakeProtocol::KemAePake => {
            // Create the in-memory storage backend
            let storage =
                Arc::new(DefaultStorage::<KemAeServer>::new()) as Arc<dyn Storage<_> + Send + Sync>;
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
            // Create the in-memory storage backend
            let storage =
                Arc::new(DefaultStorage::<OcakeServer>::new()) as Arc<dyn Storage<_> + Send + Sync>;
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
            // Create the in-memory storage backend
            let storage =
                Arc::new(DefaultStorage::<TkServer>::new()) as Arc<dyn Storage<_> + Send + Sync>;
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
