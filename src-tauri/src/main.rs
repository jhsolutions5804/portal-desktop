// JH Portal 데스크톱 앱
// portal.jhsol.kr을 독립 창으로 불러오고, 팝업(재직증명서/PDF 출력 등)을
// 새 네이티브 창으로 띄워주는 Tauri 래퍼입니다.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

#[tauri::command]
fn open_popup(app: tauri::AppHandle, url: String) {
    let label = format!(
        "popup-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );
    let _ = WebviewWindowBuilder::new(&app, label, WebviewUrl::External(url.parse().unwrap()))
        .title("JH Portal")
        .inner_size(1000.0, 800.0)
        .build();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_popup])
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                // 포털 페이지가 window.open(url)로 팝업을 띄우려 할 때,
                // 브라우저 팝업 대신 실제 새 창을 열어주도록 가로챕니다.
                let _ = window.eval(
                    "window.open = function(url) { \
                        if (window.__TAURI__ && url) { \
                            window.__TAURI__.core.invoke('open_popup', { url: url }); \
                        } \
                        return null; \
                    };",
                );
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("JH Portal 실행 중 오류가 발생했습니다");
}
