// JH Portal 데스크톱 앱
// portal.jhsol.kr을 독립 창으로 불러오는 Tauri 래퍼입니다.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("JH Portal 실행 중 오류가 발생했습니다");
}
