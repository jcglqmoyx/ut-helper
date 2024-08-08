use std::{process, time};
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use actix_web::{App, HttpServer, Responder, web};
use anyhow::{Context, Result};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::signal::ctrl_c;

use crate::util::quit_game;

mod util;

struct AppState {
    mode: Mutex<i32>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        AppState {
            mode: Mutex::new(*self.mode.lock().unwrap()),
        }
    }
}

async fn update(data: web::Data<Arc<AppState>>, mode: web::Path<i32>) -> impl Responder {
    let mut mode_lock = data.mode.lock().unwrap();
    *mode_lock = *mode;
    format!("Mode set to: {}", *mode_lock)
}

async fn get(data: web::Data<Arc<AppState>>) -> impl Responder {
    let mode = data.mode.lock().unwrap();
    format!("Current mode: {}", *mode)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let data = Arc::new(AppState {
        mode: Mutex::new(1),
    });

    let data_clone = data.clone();
    let server_data = web::Data::new(data.clone());

    let server = HttpServer::new(move || {
        App::new()
            .app_data(server_data.clone())
            .route("/update/{mode}", web::get().to(update))
            .route("/get", web::get().to(get))
    })
        .bind("0.0.0.0:8080")?
        .run();

    let server_handle = tokio::spawn(server);

    tokio::spawn(async move {
        let mut last_time_gamble: u64 = 0;
        let mut kit = 0;
        loop {
            if *data_clone.mode.lock().unwrap() == 0 {
                continue;
            }
            if *data_clone.mode.lock().unwrap() == 1 {
                util::key_press(&mut kit);
            } else if *data_clone.mode.lock().unwrap() >= 50000 {
                let now = util::now();
                if now - last_time_gamble < 125000 {
                    continue;
                }
                util::gamble(*data_clone.mode.lock().unwrap());
                last_time_gamble = now;
            }
            tokio::time::sleep(time::Duration::from_millis(50)).await;
        }
    });
    tokio::spawn(async move {
        fn other_users_connected(s: &str) -> bool {
            if s.contains("connections") && s.contains("connected from") {
                if !(s.contains("UrT") || s.contains("Juliet") || s.contains("Fried") || s.contains("Camel")) {
                    return true;
                }
            }
            false
        }
        async fn run_application() -> Result<()> {
            let command = "stdbuf";
            let args = [
                "-oL", "/home/hqc/Downloads/games/UrbanTerror43/Quake3-UrT.x86_64",
                "+connect", "94.130.173.8:27961",
            ];

            let mut process = Command::new(command)
                .args(&args)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .context("Failed to spawn process.")?;

            if let Some(stdout) = process.stdout.take() {
                tokio::spawn(async move {
                    let mut reader = BufReader::new(stdout).lines();
                    while let Some(line) = reader.next_line().await.unwrap_or(None) {
                        if other_users_connected(&line) {
                            println!("{}\nExiting..", line);
                            quit_game();
                            tokio::time::sleep(Duration::from_secs(1)).await;
                            process::exit(0);
                        }
                    }
                });
            }

            if let Some(stderr) = process.stderr.take() {
                tokio::spawn(async move {
                    let mut reader = BufReader::new(stderr).lines();
                    while let Some(line) = reader.next_line().await.unwrap_or(None) {
                        if other_users_connected(&line) {
                            println!("{}\nExiting..", line);
                            quit_game();
                            tokio::time::sleep(Duration::from_secs(1)).await;
                            process::exit(0);
                        }
                    }
                });
            }

            process.wait().await.context("Failed to await process completion.")?;

            Ok(())
        }
        run_application().await.expect("Encountered an error. Exited.");
    });
    ctrl_c().await.expect("Failed to listen for ctrl+c.");
    server_handle.abort();
    Ok(())
}
