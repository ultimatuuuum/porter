use tokio::io::copy_bidirectional;
use tokio::net::{TcpListener, TcpStream};

use futures::FutureExt;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let (listen_addr, server_addr) = match args.len() {
        2 => ("127.0.0.1:8132".to_string(), args[1].clone()),
        3 => (args[1].clone(), args[2].clone()),
        _ => {
            eprintln!("Usage: {} [LISTEN_ADDR] REMOTE_ADDR", args[0]);
            eprintln!("  LISTEN_ADDR (optional) - local address to listen on (default: 127.0.0.1:8132)");
            eprintln!("  REMOTE_ADDR (required) - remote server address (e.g., 78.43.2.5:88923)");
            std::process::exit(1);
        }
    };

    println!("Listening on: {listen_addr}");
    println!("Proxying to: {server_addr}");

    let listener = TcpListener::bind(listen_addr).await?;

    while let Ok((mut inbound, _)) = listener.accept().await {
        let mut outbound = TcpStream::connect(server_addr.clone()).await?;

        tokio::spawn(async move {
            copy_bidirectional(&mut inbound, &mut outbound)
                .map(|r| {
                    if let Err(e) = r {
                        println!("Failed to transfer; error={e}");
                    }
                })
                .await
        });
    }

    Ok(())
}
