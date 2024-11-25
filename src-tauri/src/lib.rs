use anyhow::Result;
use majsoul_max_rs::*;
use std::sync::Arc;
use tauri::{async_runtime::JoinHandle, path::BaseDirectory, Emitter, Listener, Manager};
use tokio::sync::{Mutex, Notify};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn start_proxy(handle: tauri::AppHandle) {
    static JH: Mutex<Option<JoinHandle<Result<()>>>> = Mutex::const_new(None);
    let mut jh = JH.lock().await;
    if let Some(jh) = jh.as_ref() {
        if !jh.inner().is_finished() {
            return;
        }
    }
    let new_jh = tauri::async_runtime::spawn(run_proxy(handle.clone()));
    *jh = Some(new_jh);
}

#[tauri::command]
fn stop_proxy(handle: tauri::AppHandle) -> tauri::Result<()> {
    handle.emit("stop_proxy", ())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .with_colors(tauri_plugin_log::fern::colors::ColoredLevelConfig::default())
                .level(log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, stop_proxy, start_proxy])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub async fn run_proxy(handle: tauri::AppHandle) -> Result<()> {
    let config_path = handle
        .path()
        .resolve("liqi_config/", BaseDirectory::Resource)?;
    let settings = Box::new(Settings::new(&config_path)?);
    let settings: &'static Settings = Box::leak(settings);
    let mod_settings = RwLock::new(ModSettings::new(settings)?);

    // show mod and helper switch status, green for on, red for off
    println!(
        "\n\x1b[{}mmod: {}\x1b[0m\n\x1b[{}mhelper: {}\x1b[0m\n",
        if settings.mod_on() { 32 } else { 31 },
        if settings.mod_on() { "on" } else { "off" },
        if settings.helper_on() { 32 } else { 31 },
        if settings.helper_on() { "on" } else { "off" }
    );

    if settings.auto_update() {
        info!("自动更新liqi已开启");
        let mut new_settings = settings.clone();
        match new_settings.update().await {
            Err(e) => warn!("更新liqi失败: {e}"),
            Ok(true) => {
                info!("liqi更新成功, 请重启程序");
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                return Ok(());
            }
            _ => (),
        }
    }

    let modder = if settings.mod_on() {
        // start mod worker
        info!("Mod worker started");
        if mod_settings.read().await.auto_update() {
            info!("自动更新mod已开启");
            let mut new_mod_settings = mod_settings.read().await.clone();
            match new_mod_settings.get_lqc().await {
                Err(e) => warn!("更新mod失败: {e}"),
                Ok(true) => {
                    info!("mod更新成功, 请重启程序");
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    return Ok(());
                }
                Ok(false) => (),
            }
        }
        Some(Modder::new(mod_settings).await?)
    } else {
        None
    };

    let stop_notify = Arc::new(Notify::new());
    let stop_notify_clone = stop_notify.clone();
    handle.once("stop_proxy", move |_| {
        stop_notify_clone.notify_one();
    });
    build_and_start_proxy(settings, modder, async move {
        stop_notify.notified().await;
        warn!("Proxy stopped");
    })
    .await
}
