// JH Portal 데스크톱 앱
// portal.jhsol.kr을 독립 창으로 불러오고, 팝업(재직증명서/PDF 출력 등)을
// 새 네이티브 창으로 띄워주는 Tauri 래퍼입니다.
//
// 로그인 정책: 앱을 새로 실행할 때마다 로그인 세션 저장소를 초기화하여
// 매번 로그인 화면부터 시작하지만, 앱이 켜져 있는 동안에는 로그인 정보가
// 그대로 유지되어 업무 중 자동 로그아웃되지 않습니다.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

const POPUP_OVERRIDE_SCRIPT: &str = r#"
(function () {
    window.open = function (url) {
        try {
            if (window.__TAURI__ && url) {
                window.__TAURI__.core.invoke('open_popup', { url: url });
            }
        } catch (e) {}
        // 포털 스크립트가 "if (!win) 팝업 차단 메시지" 형태로 확인하는 경우가 많아
        // null 대신 정상 창처럼 보이는 더미 객체를 돌려줍니다.
        return { closed: false, focus: function () {}, close: function () {} };
    };
})();
"#;

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
        .initialization_script(POPUP_OVERRIDE_SCRIPT)
        .build();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_popup])
        .setup(|app| {
            // 앱 실행 시마다 로그인 세션 저장 폴더를 초기화합니다.
            // (앱이 실행 중인 동안에는 이 폴더가 계속 유지되므로 로그인이 풀리지 않습니다.)
            let session_dir = app
                .path()
                .app_local_data_dir()
                .expect("로컬 데이터 폴더를 찾을 수 없습니다")
                .join("session");
            if session_dir.exists() {
                let _ = fs::remove_dir_all(&session_dir);
            }

            WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External("https://portal.jhsol.kr".parse().unwrap()),
            )
            .title("JH Portal")
            .inner_size(1400.0, 900.0)
            .min_inner_size(1024.0, 700.0)
            .initialization_script(POPUP_OVERRIDE_SCRIPT)
            .data_directory(session_dir)
            .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("JH Portal 실행 중 오류가 발생했습니다");
}
