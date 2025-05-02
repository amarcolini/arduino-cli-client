//! # Example
//!
//! Make sure to run the following before:
//! ```text
//! arduino-cli daemon
//! ```
//!
//! ## Code
//!
//! ```rust
//! use arduino_cli_client::commands::arduino_core_client::ArduinoCoreClient;
//! use arduino_cli_client::commands::{BoardListReq, InitReq};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!    let mut client = ArduinoCoreClient::connect("http://localhost:50051").await?;
//!
//!    // Start a new instance of the Arduino Core Service
//!    let mut init_stream = client
//!        .init(InitReq {
//!            library_manager_only: false,
//!        })
//!        .await?
//!        .into_inner();
//!
//!    let resp_instance = init_stream.message().await?.expect("Failed to init");
//!
//!    // List the boards currently connected to the computer.
//!    let resp_boards = client
//!        .board_list(BoardListReq {
//!            instance: resp_instance.instance,
//!        })
//!        .await?
//!        .into_inner();
//!
//!    print!("Boards: {:?}", resp_boards.ports);
//!
//!    Ok(())
//! }
//! ```
//!

/// Main Arduino Platform service
pub mod cc {
    pub mod arduino {
        pub mod cli {
            pub mod commands {
                pub mod v1 {
                    tonic::include_proto!("cc.arduino.cli.commands.v1");
                }
            }
        }
    }
}

pub (crate) mod google {
    pub (crate) mod rpc {
        tonic::include_proto!("google.rpc");
    }
}

// Service that abstract a debug Session usage
// pub mod debug {
//     tonic::include_proto!("cc.arduino.cli.debug.v1");
// }

// Service that abstracts a Monitor usage
// pub mod monitor {
//     tonic::include_proto!("cc.arduino.cli.monitor.v1");
// }

// The Settings service provides an interface to Arduino CLI's configuration options
// pub mod settings {
//     tonic::include_proto!("cc.arduino.cli.settings.v1");
// }

pub use crate::cc::arduino::cli::commands::v1::arduino_core_service_client::ArduinoCoreServiceClient;
// pub use crate::commands::debug_client::DebugClient;
// pub use crate::monitor::monitor_client::MonitorClient;
// pub use crate::settings::settings_client::SettingsClient;
