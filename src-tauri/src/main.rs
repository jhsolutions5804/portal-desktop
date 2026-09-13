// JH Portal 데스크톱 앱
// portal.jhsol.kr을 독립 창으로 불러오고, 팝업(재직증명서/PDF 출력 등)을
// 새 네이티브 창으로 띄워주는 Tauri 래퍼입니다.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
            // 메인 창을 코드에서 직접 생성하면서, 모든 프레임(iframe 포함)에
            // 페이지가 로드되기 전에 팝업 우회 스크립트를 먼저 주입합니다.
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
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("JH Portal 실행 중 오류가 발생했습니다");
}
