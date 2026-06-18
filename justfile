default:
  just --list

[linux]
run: 
  WEBKIT_DISABLE_DMABUF_RENDERER=1 bun run tauri dev

[windows]
run:
  bun run tauri dev