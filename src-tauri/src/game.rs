use anyhow::Result;
use tauri::{
    async_runtime::spawn, AppHandle, Emitter, Listener, LogicalPosition, LogicalSize, Manager,
    State, Webview, WebviewUrl, Window, WindowEvent,
};
use tauri_plugin_shell::{process::CommandEvent, ShellExt};

#[tauri::command]
pub async fn start_game(app_handle: tauri::AppHandle) -> Result<(), String> {
    let width = 800.;
    let height = 600.;
    let window = tauri::window::WindowBuilder::new(&app_handle, "game")
        .inner_size(width, height)
        .title("MajSoulMax-tauri: Game")
        .center()
        .build()
        .map_err(|e| e.to_string())?;

    let _game = window.add_child(
            tauri::webview::WebviewBuilder::new(
                "game",
                WebviewUrl::External("https://game.maj-soul.com/1/".parse().unwrap()),
            )
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36")
            .auto_resize(),
            LogicalPosition::new(0., 0.),
            LogicalSize::new(width, height),
        ).map_err(|e|e.to_string())?;

    let helper = window
        .add_child(
            tauri::webview::WebviewBuilder::new("helper", WebviewUrl::App("helper".into()))
                .auto_resize()
                .transparent(true),
            LogicalPosition::new(width / 2., 0.),
            LogicalSize::new(width / 2.0, height / 2.0),
        )
        .map_err(|e| e.to_string())?;

    spawn(start_helper(helper));
    Ok(())
}

async fn start_helper(webview: Webview) {
    let cmd = webview
        .shell()
        .sidecar("mahjong-helper")
        .unwrap()
        .set_raw_out(true);
    let (mut rx, mut child) = cmd.spawn().unwrap();

    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stdout(v) => {
                let string = String::from_utf8_lossy(&v);
                println!("Recv: {string}");
                if let Err(e) = webview.emit("helper", &string) {
                    println!("Error emit msg: {e}");
                };
                if string.contains("请输入") {
                    if let Err(e) = child.write(b"1\n") {
                        println!("Fail to write to pty: {e}")
                    }
                };
            }
            _ => {}
        }
    }
}
