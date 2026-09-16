// JH Portal 앱 (Windows 데스크톱 + Android/iOS 공용 로직)
// portal.jhsol.kr을 창(또는 화면 전체)으로 불러오고, 팝업(재직증명서/PDF 출력 등)을
// 새 네이티브 창으로 띄워주는 Tauri 래퍼입니다.
//
// 데스크톱 로그인 정책: 앱을 새로 실행할 때마다 로그인 세션 저장소를 초기화하여
// 매번 로그인 화면부터 시작하지만, 앱이 켜져 있는 동안에는 로그인 정보가
// 그대로 유지되어 업무 중 자동 로그아웃되지 않습니다. (모바일은 OS 표준 동작을
// 따르도록 이 초기화 로직을 적용하지 않습니다.)
use tauri::{WebviewUrl, WebviewWindowBuilder};

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

#[cfg(desktop)]
fn write_log(base_dir: &std::path::Path, msg: &str) {
    use std::io::Write;
    let _ = std::fs::create_dir_all(base_dir);
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(base_dir.join("app.log"))
    {
        let _ = writeln!(f, "{msg}");
    }
}

#[cfg(desktop)]
fn setup_desktop(app: &tauri::App) -> tauri::Result<()> {
    use std::fs;
    use tauri::Manager;

    // 앱 실행 시마다 로그인 세션 저장 폴더를 초기화합니다.
    // (앱이 실행 중인 동안에는 이 폴더가 계속 유지되므로 로그인이 풀리지 않습니다.)
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
}

#[cfg(mobile)]
fn setup_mobile(app: &tauri::App) -> tauri::Result<()> {
    // 모바일은 앱마다 저장소가 OS 차원에서 이미 격리되어 있으므로
    // 데스크톱처럼 세션 폴더를 직접 관리할 필요가 없습니다.
    WebviewWindowBuilder::new(
        app,
        "main",
        WebviewUrl::External("https://portal.jhsol.kr".parse().unwrap()),
    )
    .title("JH Portal")
    .initialization_script(POPUP_OVERRIDE_SCRIPT)
    .build()?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_popup])
        .setup(|app| {
            #[cfg(desktop)]
            setup_desktop(app)?;
            #[cfg(mobile)]
            setup_mobile(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("JH Portal 실행 중 오류가 발생했습니다");
}
