// JH Portal 데스크톱 앱
// portal.jhsol.kr을 독립 창으로 불러오고, 팝업(재직증명서/PDF 출력 등)을
// 새 네이티브 창으로 띄워주는 Tauri 래퍼입니다.
//
// 로그인 정책: 앱을 새로 실행할 때마다 로그인 세션 저장소를 초기화하여
// 매번 로그인 화면부터 시작하지만, 앱이 켜져 있는 동안에는 로그인 정보가
// 그대로 유지되어 업무 중 자동 로그아웃되지 않습니다.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::io::Write;
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

const POPUP_OVERRIDE_SCRIPT: &str = r#"
(function () {
    window.open = function (url) {
        try {
            if (window.__TAURI__ && url) {
                window.__TAURI__.core.invoke('open_popup', { url: url });
            }
        } catch (e) {}
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

fn write_log(base_dir: &std::path::Path, msg: &str) {
    let _ = fs::create_dir_all(base_dir);
    if let Ok(mut f) = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(base_dir.join("app.log"))
    {
        let _ = writeln!(f, "{msg}");
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_popup])
        .setup(|app| {
            let base_dir = app
                .path()
                .app_local_data_dir()
                .unwrap_or_else(|_| std::env::temp_dir().join("jh-portal"));
            let _ = fs::create_dir_all(&base_dir);

            let session_dir = base_dir.join("session");
            if session_dir.exists() {
                if let Err(e) = fs::remove_dir_all(&session_dir) {
                    write_log(&base_dir, &format!("세션 폴더 삭제 실패: {e}"));
                }
            }
            if let Err(e) = fs::create_dir_all(&session_dir) {
                write_log(&base_dir, &format!("세션 폴더 생성 실패: {e}"));
            }

            let build_result = WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External("https://portal.jhsol.kr".parse().unwrap()),
            )
            .title("JH Portal")
            .inner_size(1400.0, 900.0)
            .min_inner_size(1024.0, 700.0)
            .initialization_script(POPUP_OVERRIDE_SCRIPT)
            .data_directory(session_dir)
            .build();

            if let Err(e) = build_result {
                write_log(
                    &base_dir,
                    &format!("데이터 폴더 지정 창 생성 실패: {e} → 기본 방식으로 재시도"),
                );
                // 세션 초기화 기능이 실패하더라도 앱 자체는 반드시 뜨도록 기본 방식으로 재시도합니다.
                WebviewWindowBuilder::new(
                    app,
                    "main",
                    WebviewUrl::External("https://portal.jhsol.kr".parse().unwrap()),
                )
                .title("JH Portal")
                .inner_size(1400.0, 900.0)
                .min_inner_size(1024.0, 700.0)
                .initialization_script(POPUP_OVERRIDE_SCRIPT)
                .build()?;
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("JH Portal 실행 중 오류가 발생했습니다");
}
