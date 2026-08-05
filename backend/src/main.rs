use tokio::net::{TcpListener, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;
use futures_util::{SinkExt, StreamExt};
use serde_json;
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, error, warn};

mod models;
mod event_loop;

use models::{WsMessage, EventLoopState, ConsoleMessage, ConsoleLevel};
use event_loop::EventLoopEngine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();
    
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    info!("WebSocket server listening on: {}", addr);
    
    // Shared event loop engine
    let engine = Arc::new(Mutex::new(EventLoopEngine::new()));
    
    while let Ok((stream, addr)) = listener.accept().await {
        info!("New connection from: {}", addr);
        
        let engine_clone = Arc::clone(&engine);
        
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, engine_clone).await {
                error!("Error handling connection: {}", e);
            }
        });
    }
    
    Ok(())
}

async fn handle_connection(
    stream: TcpListener,
    engine: Arc<Mutex<EventLoopEngine>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Accept WebSocket handshake
    let ws_stream = tokio_tungstenite::accept_async(stream.into()).await?;
    let (mut write, mut read) = ws_stream.split();
    
    info!("WebSocket connection established");
    
    // Send initial state
    {
        let engine_guard = engine.lock().await;
        let state = engine_guard.state.clone();
        let msg = WsMessage::StateUpdate(state);
        let json = serde_json::to_string(&msg)?;
        write.send(Message::Text(json)).await?;
    }
    
    // Handle incoming messages
    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Err(e) = handle_message(&text, &engine, &mut write).await {
                    error!("Error handling message: {}", e);
                    
                    // Send error to client
                    let error_msg = WsMessage::Error {
                        message: e.to_string(),
                        code: None,
                    };
                    let json = serde_json::to_string(&error_msg)?;
                    write.send(Message::Text(json)).await?;
                }
            }
            Ok(Message::Ping(data)) => {
                write.send(Message::Pong(data)).await?;
            }
            Ok(Message::Close(_)) => {
                info!("Client disconnected");
                break;
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }
    
    Ok(())
}

async fn handle_message(
    text: &str,
    engine: &Arc<Mutex<EventLoopEngine>>,
    write: &mut futures_util::stream::SplitSink<WebSocketStream<tokio::net::TcpStream>, Message>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Parse incoming message
    let msg: WsMessage = serde_json::from_str(text)?;
    
    let mut engine_guard = engine.lock().await;
    
    match msg {
        WsMessage::ExecuteCode { code } => {
            info!("Executing code: {}", code.chars().take(50).collect::<String>());
            engine_guard.execute_code(&code)?;
            
            // Send updated state
            let state = engine_guard.state.clone();
            let response = WsMessage::StateUpdate(state);
            let json = serde_json::to_string(&response)?;
            write.send(Message::Text(json)).await?;
        }
        
        WsMessage::StepOver => {
            engine_guard.step_over()?;
            let state = engine_guard.state.clone();
            let response = WsMessage::StateUpdate(state);
            let json = serde_json::to_string(&response)?;
            write.send(Message::Text(json)).await?;
        }
        
        WsMessage::StepInto => {
            engine_guard.step_into()?;
            let state = engine_guard.state.clone();
            let response = WsMessage::StateUpdate(state);
            let json = serde_json::to_string(&response)?;
            write.send(Message::Text(json)).await?;
        }
        
        WsMessage::StepOut => {
            engine_guard.step_out()?;
            let state = engine_guard.state.clone();
            let response = WsMessage::StateUpdate(state);
            let json = serde_json::to_string(&response)?;
            write.send(Message::Text(json)).await?;
        }
        
        WsMessage::Play => {
            engine_guard.play()?;
            let state = engine_guard.state.clone();
            let response = WsMessage::StateUpdate(state);
            let json = serde_json::to_string(&response)?;
            write.send(Message::Text(json)).await?;
        }
        
        WsMessage::Pause => {
            engine_guard.pause()?;
            let state = engine_guard.state.clone();
            let response = WsMessage::StateUpdate(state);
            let json = serde_json::to_string(&response)?;
            write.send(Message::Text(json)).await?;
        }
        
        WsMessage::Stop => {
            engine_guard.stop()?;
            let state = engine_guard.state.clone();
            let response = WsMessage::StateUpdate(state);
            let json = serde_json::to_string(&response)?;
            write.send(Message::Text(json)).await?;
        }
        
        WsMessage::Rewind { steps } => {
            engine_guard.rewind(steps)?;
            let state = engine_guard.state.clone();
            let response = WsMessage::StateUpdate(state);
            let json = serde_json::to_string(&response)?;
            write.send(Message::Text(json)).await?;
        }
        
        WsMessage::SetBreakpoint { file, line, condition } => {
            engine_guard.set_breakpoint(file, line, condition);
            info!("Breakpoint set");
        }
        
        WsMessage::RemoveBreakpoint { file, line } => {
            engine_guard.remove_breakpoint(file, line);
            info!("Breakpoint removed");
        }
        
        WsMessage::SetSpeed { speed_ms } => {
            engine_guard.set_speed(speed_ms);
            info!("Speed set to {}ms", speed_ms);
        }
        
        _ => {
            warn!("Unhandled message type");
        }
    }
    
    Ok(())
}
