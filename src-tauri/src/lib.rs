use tauri::Manager;

#[cfg(target_os = "windows")]
mod sharing_overlay_hider {
    use std::{thread, time::Duration};
    use windows_sys::Win32::Foundation::{HWND, LPARAM};
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        EnumWindows, GetWindowTextLengthW, GetWindowTextW, IsWindowVisible, ShowWindow, SW_HIDE,
    };
    use windows_sys::core::BOOL;

    pub fn start() {
        thread::spawn(|| loop {
            hide_sharing_overlays();
            thread::sleep(Duration::from_millis(1200));
        });
    }

    fn hide_sharing_overlays() {
        unsafe {
            let _ = EnumWindows(Some(enum_windows_proc), 0);
        }
    }

    unsafe extern "system" fn enum_windows_proc(hwnd: HWND, _: LPARAM) -> BOOL {
        if unsafe { IsWindowVisible(hwnd) } == 0 {
            return 1;
        }

        let len = unsafe { GetWindowTextLengthW(hwnd) };
        if len <= 0 {
            return 1;
        }

        let mut buffer = vec![0u16; (len as usize) + 1];
        let read_len = unsafe { GetWindowTextW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32) };
        if read_len <= 0 {
            return 1;
        }

        let title = String::from_utf16_lossy(&buffer[..read_len as usize]).to_lowercase();
        if title.contains("is sharing your screen") {
            let _ = unsafe { ShowWindow(hwnd, SW_HIDE) };
        }
        1
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(target_os = "windows")]
    sharing_overlay_hider::start();

    tauri::Builder::default()
        .setup(|app| {
            if let (Some(window), Some(icon)) = (
                app.get_webview_window("main"),
                app.default_window_icon()
            ) {
                let _ = window.set_icon(icon.clone());
            }
            Ok(())
        })
        .on_page_load(|window, _| {
            let script = r#"
              (() => {
                const selectors = [
                  'button[aria-label="Hide"]',
                  'button[data-testid="hide"]',
                  'button'
                ];

                const tryHideSharingBar = () => {
                  for (const selector of selectors) {
                    const elements = document.querySelectorAll(selector);
                    for (const el of elements) {
                      const label = (el.getAttribute('aria-label') || '').trim().toLowerCase();
                      const text = (el.textContent || '').trim().toLowerCase();
                      if (label === 'hide' || text === 'hide') {
                        el.click();
                        return true;
                      }
                    }
                  }
                  return false;
                };

                tryHideSharingBar();
                const observer = new MutationObserver(() => {
                  tryHideSharingBar();
                });
                observer.observe(document.documentElement, { childList: true, subtree: true });
                setInterval(tryHideSharingBar, 1500);
              })();
            "#;

            let _ = window.eval(script);
        })
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
