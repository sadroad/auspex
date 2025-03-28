use futures_util::{SinkExt, StreamExt};
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::process::Command;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_url = "ws://192.168.1.21:3001";

    let mut cmd = Command::new("rpicam-vid");
    cmd.args([
        "-t",
        "0", // Run indefinitely
        "--width",
        "1920",
        "--height",
        "1080",
        "--framerate",
        "60",
        "--inline", // Embed SPS/PPS headers
        "-n",       // No preview
        "--codec",
        "libav",
        "--libav-format",
        "rawvideo",
        "-o",
        "-", // Output to stdout
    ]);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = cmd.spawn()?;
    let stdout = child
        .stdout
        .take()
        .ok_or("Child process did not have a handle to stdout")?;

    let stderr = child.stderr.take();
    if let Some(stderr_handle) = stderr {
        tokio::spawn(async move {
            let mut reader = BufReader::new(stderr_handle);
            let mut line = String::new();
            while reader.read_line(&mut line).await.unwrap_or(0) > 0 {
                eprintln!("[rpicam-vid stderr]: {}", line.trim_end());
                line.clear();
            }
        });
    }

    let (ws_stream, _) = connect_async(server_url).await.expect("Failed to connect");
    println!("WebSocket connected to {}", server_url);
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    let mut reader = BufReader::new(stdout);
    let mut buffer = vec![0u8; 8192]; // Read in 8KB chunks

    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                break;
            }

            read_result = reader.read(&mut buffer) => {
                match read_result {
                    Ok(0) => {
                        println!("rpicam-vid stdout stream ended.");
                        break; // EOF
                    }
                    Ok(n) => {
                        if let Err(e) = ws_sender.send(Message::Binary(buffer[..n].to_vec().into())).await {
                            eprintln!("WebSocket send error: {}", e);
                            break;
                        }
                        println!("Sent {} bytes", n);
                    }
                    Err(e) => {
                        eprintln!("Error reading from rpicam-vid stdout: {}", e);
                        break;
                    }
                }
            }

            Some(msg_result) = ws_receiver.next() => {
                 match msg_result {
                    Ok(msg) => {
                        if msg.is_close() {
                            println!("Server requested close.");
                            break;
                        }
                        println!("Received msg: {:?}", msg);
                    },
                    Err(e) => {
                        eprintln!("WebSocket receive error: {}", e);
                        break;
                    }
                 }
            }

             _ = child.wait() => {
                println!("rpicam-vid process exited.");
                break;
            }
        }
    }

    println!("Closing WebSocket stream.");
    let _ = ws_sender.close().await;
    let _ = child.kill().await;

    Ok(())
}
