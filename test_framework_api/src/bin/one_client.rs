// #![allow(unused_imports)]
use qr_pake_protocol_executors::{login, parse_client_cli_args, register};
use qr_pake_protocols::{KemAeClient, OcakeClient, PakeProtocol, TkClient};

#[tokio::main]
async fn main() {
    // Get parsed args
    let (client_id, password, pake_protocol, protcol_variant, kem_choice, ip_address, port) =
        parse_client_cli_args();

    let session_key = match pake_protocol {
        PakeProtocol::KemAePake => {
            // Perform registration
            let client_instance = register::<KemAeClient>(
                client_id.as_bytes(),
                password.as_bytes(),
                protcol_variant,
                None,
                &ip_address,
                &port,
            )
            .await
            .expect("Registration failed");

            // Perform login
            let session_key: [u8; 32] = login::<KemAeClient>(client_instance, &ip_address, &port)
                .await
                .expect("Login failed");

            session_key
        }
        PakeProtocol::OcakePake => {
            // Perform registration
            let client_instance = register::<OcakeClient>(
                client_id.as_bytes(),
                password.as_bytes(),
                protcol_variant,
                Some(kem_choice),
                &ip_address,
                &port,
            )
            .await
            .expect("Registration failed");

            // Perform login
            let session_key: [u8; 32] = login::<OcakeClient>(client_instance, &ip_address, &port)
                .await
                .expect("Login failed");

            session_key
        }
        PakeProtocol::TkPake => {
            // Perform registration
            let client_instance = register::<TkClient>(
                client_id.as_bytes(),
                password.as_bytes(),
                protcol_variant,
                None,
                &ip_address,
                &port,
            )
            .await
            .expect("Registration failed");

            // Perform login
            let session_key: [u8; 32] = login::<TkClient>(client_instance, &ip_address, &port)
                .await
                .expect("Login failed");

            session_key
        }
    };
    println!(
        "\x1b[92m\t Client's Session Key: {:?} \x1b[0m\n",
        session_key
    );
}
