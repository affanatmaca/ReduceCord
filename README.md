<img width="660" height="72" alt="{1F3B8538-C7FD-4A12-B9BA-EC828EE370DB}" src="https://github.com/user-attachments/assets/36556f3a-4b7f-4e64-83ee-189ad5b7e154" />


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
