# Leptos Toy

Tauri + Leptos 를 이용한 Cross-platform 앱 개발을 위한 예제 프로젝트입니다.

## Leptos 실행 

```bash
# -----------------------------
#           사전준비
# -----------------------------

# trunk 설치
cargo install trunk

#component 설치
rustup component add rust-src

# wasm 타겟 추가
rustup target add wasm32-unknown-unknown

# -----------------------------
#           실행
# -----------------------------

#프로젝트 이동
cd ui

# trunk 실행
cargo trunk serve

```
