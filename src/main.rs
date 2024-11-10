pub mod markdown;

use lsp_server::{Connection, Message};
use lsp_types::{InitializeParams, InitializeResult, ServerCapabilities};

#[tokio::main]
async fn main() {
    let (connection, io_threads) = Connection::stdio();

    let (initialize_id, initialize_params) = connection.initialize_start().unwrap();

    let _params: InitializeParams = serde_json::from_value(initialize_params).unwrap();

    let capabilities = ServerCapabilities::default();
    let initialize_result = InitializeResult {
        capabilities,
        server_info: None,
    };

    connection
        .initialize_finish(
            initialize_id,
            serde_json::to_value(initialize_result).unwrap(),
        )
        .unwrap();

    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req).unwrap() {
                    break;
                }

                eprintln!("Received request: {:?}", req);
            }
            Message::Response(resp) => {
                eprintln!("Received response: {:?}", resp);
            }
            Message::Notification(notif) => {
                eprintln!("Received notification: {:?}", notif);
            }
        }
    }

    io_threads.join().unwrap();
}
