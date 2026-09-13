// JH Portal 데스크톱 앱
// portal.jhsol.kr을 독립 창으로 불러오는 Tauri 래퍼입니다.
// window.open()으로 뜨는 팝업(PDF 미리보기, 새 창 등)을 실제 창으로 열어주기 위해
// 창 생성을 tauri.conf.json 선언 대신 코드에서 직접 처리합니다.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::webview::NewWindowResponse;
use tauri::{WebviewUrl, WebviewWindowBuilder};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External("https://portal.jhsol.kr".parse().unwrap()),
            )
            .title("JH Portal")
            .inner_size(1400.0, 900.0)
            .min_inner_size(1024.0, 700.0)
            .on_new_window(|_url, _features| {
                // 포털에서 window.open()으로 요청하는 새 창(재직증명서 등 PDF 미리보기 포함)을
                // 그대로 허용해 실제 창으로 띄웁니다.
                NewWindowResponse::Allow
            })
            .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("JH Portal 실행 중 오류가 발생했습니다");
}
