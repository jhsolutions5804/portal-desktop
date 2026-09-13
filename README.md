# JH Portal 데스크톱 앱

`portal.jhsol.kr`을 불러오는 Windows용 Tauri 래퍼입니다.

## 구조
- `src-tauri/tauri.conf.json` — 앱 이름, 창 크기, 불러올 주소(`https://portal.jhsol.kr`) 설정
- `src-tauri/src/main.rs` — 앱 진입점
- `src-tauri/icons/` — 임시 아이콘 (추후 회사 로고로 교체 권장)
- `.github/workflows/build.yml` — GitHub Actions로 Windows exe/msi 자동 빌드

## exe 빌드 방법 (택 1)

### 방법 A — GitHub Actions로 자동 빌드 (권장, 별도 PC 불필요)
1. 이 폴더를 GitHub 저장소(`jhsolutions5804/portal-desktop` 등)에 push
2. GitHub Actions 탭 → `Build JH Portal Desktop App` 워크플로우 실행
3. 완료되면 `Artifacts`에서 `jh-portal-windows.zip` 다운로드 → 안에 `.exe`(NSIS 설치파일)와 `.msi` 포함
4. 이후 코드 수정 없이 push할 때마다 자동으로 새 설치파일 생성됨

### 방법 B — Windows PC에서 직접 빌드
1. [Rust](https://rustup.rs) 설치
2. [Node.js](https://nodejs.org) 설치 (18 이상)
3. 터미널에서:
   ```
   npm install -g @tauri-apps/cli
   cd jh-portal-desktop
   tauri build
   ```
4. `src-tauri/target/release/bundle/nsis/` 폴더에 설치 파일 생성됨

## 업데이트 방식
앱은 항상 `portal.jhsol.kr`을 불러오므로, 포털 기능을 웹에 배포하면 앱 재빌드 없이 바로 반영됩니다.
exe를 다시 빌드해야 하는 경우는 아이콘/앱 이름/창 크기 등 "껍데기"를 바꿀 때뿐입니다.

## 아이콘 교체
`src-tauri/icons/` 안의 파일들을 회사 로고 기반 아이콘으로 교체 후 재빌드하면 됩니다.
