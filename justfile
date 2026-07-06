default:
  just --list

[linux]
dev: 
   WEBKIT_DISABLE_DMABUF_RENDERER=1 bun run tauri dev

[windows]
dev:
  bun run tauri dev

[linux]
build: 
  bun tauri build
  bun tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc
