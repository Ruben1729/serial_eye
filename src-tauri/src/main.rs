// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use std::{time::Duration, sync::Mutex};
use tauri::{AppHandle, State, Wry};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::Mutex as AsyncMutex;
use bytes::{BytesMut, BufMut};
use tokio_serial::{DataBits, FlowControl, Parity, StopBits, SerialStream};
use tokio_util::codec::{Framed, Decoder, Encoder};
use futures::{io, StreamExt, SinkExt};

pub struct SerialState {
    pub tx: Mutex<Option<Sender<Vec<u8>>>>,
    pub port: AsyncMutex<Option<SerialStream>>,
}

pub struct CustomCodec;

impl Decoder for CustomCodec {
    type Item = u8;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if !src.is_empty() {
            Ok(Some(src.split_to(1)[0]))
        } else {
            Ok(None)
        }
    }
}

impl Encoder<Vec<u8>> for CustomCodec {
    type Error = io::Error;

    fn encode(&mut self, bytes: Vec<u8>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put_slice(bytes.as_slice());
        Ok(())
    }
}

#[tauri::command]
async fn connect_uart(state: State<'_, SerialState>, app_handle: AppHandle<Wry>) -> Result<(), String> {
    let settings = tokio_serial::new("COM7", 115_200)
        .data_bits(DataBits::Eight)
        .flow_control(FlowControl::None)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .timeout(Duration::from_millis(100));

    let port = SerialStream::open(&settings).map_err(|e| e.to_string())?;

    let (tx, rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel(32);
    let framed = Framed::new(port, CustomCodec);

    {
        let mut state_tx = state.tx.lock().unwrap();
        *state_tx = Some(tx.clone());
    }

    let (mut sink, mut stream) = framed.split();

    let app_handle_clone = app_handle.clone();
    tokio::spawn(async move {
        while let Some(result) = stream.next().await {
            match result {
                Ok(response) => {
                    app_handle_clone.emit_all("byte-handler", response).unwrap_or_else(|e| eprintln!("Failed to emit event: {}", e));
                }
                Err(e) => eprintln!("Error reading from serial port: {}", e),
            }
        }
    });

    tokio::spawn(async move {
        let mut rx = rx;
        while let Some(command) = rx.recv().await {
            if let Err(e) = sink.send(command).await {
                eprintln!("Failed to send command: {}", e);
            }
        }
    });

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![connect_uart, disconnect_uart])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
