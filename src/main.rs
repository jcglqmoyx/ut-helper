use std::sync::{Arc, Mutex};
use std::time;

use actix_web::{App, HttpServer, Responder, web};
use tokio::signal::ctrl_c;

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
        mode: Mutex::new(0),
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

    ctrl_c().await.expect("Failed to listen for ctrl+c");
    server_handle.abort();
    Ok(())
}
