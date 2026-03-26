<img width="639" height="68" alt="Untitled" src="https://github.com/user-attachments/assets/5a08c048-a819-4fd7-bec9-1a893852e453" />

ReduceCord opens https://discord.com/app inside a desktop window and does not run the official Discord desktop app.

## Requirements:

Rust - https://rust-lang.org/tools/install/
Node.js - https://nodejs.org/en/download
WebView2 Runtime (Windows) - https://developer.microsoft.com/en-us/microsoft-edge/webview2/

## Setup
Run '''npm install''' for installing packages.

Starting ReduceCord without build : npm run dev:normal , npm run dev:lite , npm run dev:no-gpu
Building ReduceCord : npm run build:normal , npm run build:lite , npm run build:no-gpu

## Build outputs:

dist/normal , dist/lite , dist/no-gpu
Output contains : discord-web.exe (portable) , Discord Web_0.1.0_x64-setup.exe (installer)

## Profiles:

normal: Balanced Profile.
lite: Lower Background Usage.
no-gpu: Litemode with disabled gpu isage (May use more cpu with some systems).
