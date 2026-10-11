use clap::Parser;
use qr_pake_protocols::{AvailableVariants, KemChoice, PakeProtocol};

pub const DEFAULT_IP: &str = "0.0.0.0";
pub const DEFAULT_PORT: &str = "8080";
const DEFAULT_LOGIN_THRESHOLD: &str = "3";
const DEFAULT_LOGIN_WINDOW: &str = "60";
const DEFAULT_TIMEOUT: &str = "5";

type IpAddress = String;
type Port = String;
type LoginThreshold = usize;
type LoginWindow = u64;
type Timeout = u64;

pub type ParsedServerArgs = (
    PakeProtocol,
    IpAddress,
    Port,
    LoginThreshold,
    LoginWindow,
    Timeout,
);

type ClientId = String;
type Password = String;

pub type ParsedClientArgs = (
    ClientId,
    Password,
    PakeProtocol,
    AvailableVariants,
    KemChoice,
    IpAddress,
    Port,
);

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct ServerArgs {
    /// PAKE Protocol to use
    pake_protocol_name: PakeProtocol,

    /// IP Address
    #[arg(long, default_value = DEFAULT_IP)]
    ip: String,

    /// Port
    #[arg(long, default_value = DEFAULT_PORT)]
    port: String,

    /// Login threshold determines the maximum number of incorrect logins allowed before a client is blocked
    #[arg(long, default_value = DEFAULT_LOGIN_THRESHOLD)]
    login_threshold: String,

    /// Login window (in seconds) determines the time window within which we track the number of incorrect logins
    #[arg(long, default_value = DEFAULT_LOGIN_WINDOW)]
    login_window: String,

    /// Timeout (in seconds) determines the amount of time the server will wait before receiving a response from the client
    #[arg(long, default_value = DEFAULT_TIMEOUT)]
    timeout: String,
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct ClientArgs {
    /// Client ID (Use quotes "", if there are spaces in the ID)
    client_id: String,

    /// Password (must not contain spaces)
    #[arg(value_parser = validate_password)]
    password: String,

    /// PAKE Protocol to use
    pake_protocol_name: PakeProtocol,

    /// Variant of Pake Protocol to use
    #[arg(long, default_value_t = AvailableVariants::Recommended)]
    protcol_variant: AvailableVariants,

    /// KEM to use
    #[arg(long, default_value_t = KemChoice::Kyber)]
    kem_choice: KemChoice,

    /// IP Address
    #[arg(long, default_value = DEFAULT_IP)]
    ip: String,

    /// Port
    #[arg(long, default_value = DEFAULT_PORT)]
    port: String,
}

fn validate_password(password: &str) -> Result<String, String> {
    if password.contains(' ') || password.contains('\t') {
        Err("Password must not contain spaces or tabs".to_string())
    } else {
        Ok(password.to_string())
    }
}

pub fn parse_server_cli_args() -> ParsedServerArgs {
    let args: ServerArgs = ServerArgs::parse();
    let pake_protocol_name: PakeProtocol = args.pake_protocol_name;
    let ip_address: IpAddress = args.ip;
    let port: Port = args.port;
    let login_threshold: LoginThreshold = args.login_threshold.parse::<usize>().unwrap();
    let login_window: LoginWindow = args.login_window.parse::<u64>().unwrap();
    let timeout: Timeout = args.timeout.parse::<u64>().unwrap();

    (
        pake_protocol_name,
        ip_address,
        port,
        login_threshold,
        login_window,
        timeout,
    )
}

pub fn parse_client_cli_args() -> ParsedClientArgs {
    let args: ClientArgs = ClientArgs::parse();
    let client_id: ClientId = args.client_id;
    let password: Password = args.password;
    let pake_protocol_name: PakeProtocol = args.pake_protocol_name;
    let protcol_variant: AvailableVariants = args.protcol_variant;
    let kem_choice: KemChoice = args.kem_choice;
    let ip_address: IpAddress = args.ip;
    let port: Port = args.port;

    (
        client_id,
        password,
        pake_protocol_name,
        protcol_variant,
        kem_choice,
        ip_address,
        port,
    )
}
