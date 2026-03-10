// 飞书项目管理客户端 - Rust 后端
// 提供窗口管理和系统集成功能

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // 获取主窗口引用
            if let Some(window) = app.get_webview_window("main") {
                // 窗口准备就绪
                println!("飞书项目管理窗口已启动");
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
