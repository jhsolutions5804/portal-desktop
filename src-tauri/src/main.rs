// Windows 데스크톱 진입점 — 실제 로직은 lib.rs(jh_portal_lib)에 있습니다.
// (모바일은 lib.rs의 mobile_entry_point가 직접 진입점 역할을 합니다.)
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    jh_portal_lib::run();
}
