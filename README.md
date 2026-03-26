Discord Web is a lightweight desktop wrapper for Discord Web, built with Tauri.

It opens discord.com/app inside a desktop window and does not run the official Discord desktop app.

Requirements:

Rust
Node.js
WebView2 Runtime (Windows)
Install: npm install

Run in development: npm run dev:normal npm run dev:lite npm run dev:no-gpu

Build: npm run build:normal npm run build:lite npm run build:no-gpu

Build outputs are created per profile:

dist/normal
dist/lite
dist/no-gpu
Each profile folder contains:

discord-web.exe (portable)
Discord Web_0.1.0_x64-setup.exe (installer)
Profiles:

normal: balanced default
lite: lower background resource usage
no-gpu: lite mode with GPU disabled (may increase CPU on some systems)
