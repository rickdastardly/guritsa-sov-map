// Gurista Sov Map: desktop app.
// A native window around the real gurista.co.uk/sov-map (Edge WebView2, built into Windows),
// so the app is always an exact copy of the site and every site update reaches it instantly,
// with nothing to download. On top: dark native frame, remembered window size and position,
// F11 fullscreen, outside links open in your browser, offline from cache, and self-update of the
// app itself (a ~300-byte check every 6 hours).
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod update;

use serde_json::{json, Value};
use std::path::PathBuf;
use tao::dpi::{LogicalSize, PhysicalPosition, PhysicalSize};
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tao::window::{Fullscreen, Icon, Theme, WindowBuilder};
use update::{Release, UpdMsg};
use wry::{NewWindowResponse, WebContext, WebViewBuilder};

const START: &str = "https://gurista.co.uk/sov-map/";
const SITE: &str = "https://gurista.co.uk/sov-map";

#[derive(Debug)]
enum Ev {
    Upd(UpdMsg),
    Ready,
    InstallUpdate,
    Fullscreen,
    Open(String),
}

fn dirs() -> Option<directories::ProjectDirs> { directories::ProjectDirs::from("uk.co", "Gurista", "SovMap") }
fn settings_path() -> Option<PathBuf> { dirs().map(|d| d.config_dir().join("window.json")) }
fn load_settings() -> Value {
    settings_path().and_then(|p| std::fs::read_to_string(p).ok()).and_then(|s| serde_json::from_str(&s).ok()).unwrap_or(Value::Null)
}
fn save_settings(v: &Value) {
    if let Some(p) = settings_path() {
        if let Some(d) = p.parent() { let _ = std::fs::create_dir_all(d); }
        let _ = std::fs::write(p, v.to_string());
    }
}

/// Links that leave the site open in the user's normal browser.
fn open_external(url: &str) {
    if !(url.starts_with("https://") || url.starts_with("http://")) { return; }
    #[cfg(windows)]
    { let _ = std::process::Command::new("rundll32").args(["url.dll,FileProtocolHandler", url]).spawn(); }
    #[cfg(not(windows))]
    { let _ = std::process::Command::new("xdg-open").arg(url).spawn(); }
}

/// Only the Sov Map itself stays in the app window; everything else (the gurista.co.uk
/// portfolio, zKillboard, Dotlan...) opens in the user's browser.
fn is_site(url: &str) -> bool {
    url.starts_with(SITE) || url.starts_with("about:") || url.starts_with("data:") || url.starts_with("blob:")
}

fn main() {
    update::cleanup();
    let event_loop = EventLoopBuilder::<Ev>::with_user_event().build();
    let proxy = event_loop.create_proxy();

    let s = load_settings();
    let icon = Icon::from_rgba(include_bytes!("../assets/icon-64.rgba").to_vec(), 64, 64).ok();
    let mut wb = WindowBuilder::new()
        .with_title("Gurista Sov Map")
        .with_theme(Some(Theme::Dark))
        .with_window_icon(icon)
        .with_min_inner_size(LogicalSize::new(900.0, 600.0))
        .with_inner_size(LogicalSize::new(1440.0, 880.0))
        .with_maximized(s["max"].as_bool().unwrap_or(false))
        .with_visible(false);
    if let (Some(w), Some(h)) = (s["w"].as_u64(), s["h"].as_u64()) {
        if w >= 600 && h >= 400 { wb = wb.with_inner_size(PhysicalSize::new(w as u32, h as u32)); }
    }
    if let (Some(x), Some(y)) = (s["x"].as_i64(), s["y"].as_i64()) {
        if x > -2000 && y > -200 { wb = wb.with_position(PhysicalPosition::new(x as i32, y as i32)); }
    }
    let window = match wb.build(&event_loop) { Ok(w) => w, Err(e) => { eprintln!("window: {e}"); return; } };

    // WebView2 keeps its cache here (not next to the .exe, which may be read-only).
    let data_dir = dirs().map(|d| d.data_local_dir().join("WebView2"));
    let mut ctx = WebContext::new(data_dir);
    let script = include_str!("desktop.js").replace("__VERSION__", update::VERSION);

    let p_ipc = proxy.clone();
    let p_nav = proxy.clone();
    let webview = WebViewBuilder::new_with_web_context(&mut ctx)
        .with_url(START)
        .with_background_color((5, 7, 10, 255))
        .with_initialization_script(script)
        .with_devtools(cfg!(debug_assertions))
        .with_ipc_handler(move |req: wry::http::Request<String>| {
            let m = req.body().as_str();
            let ev = match m {
                "ready" => Some(Ev::Ready),
                "update" => Some(Ev::InstallUpdate),
                "fullscreen" => Some(Ev::Fullscreen),
                _ => m.strip_prefix("open:").map(|u| Ev::Open(u.to_owned())),
            };
            if let Some(ev) = ev { let _ = p_ipc.send_event(ev); }
        })
        .with_navigation_handler(move |url: String| {
            if is_site(&url) { return true; }
            let _ = p_nav.send_event(Ev::Open(url));
            false
        })
        .with_new_window_req_handler(move |url: String, _features| {
            open_external(&url);
            NewWindowResponse::Deny
        })
        .build(&window);
    let webview = match webview {
        Ok(w) => w,
        Err(e) => {
            eprintln!("webview: {e}");
            // Most likely the Edge WebView2 runtime is missing (rare on Windows 10/11).
            open_external("https://developer.microsoft.com/microsoft-edge/webview2/#download");
            return;
        }
    };
    window.set_visible(true);

    let p_upd = proxy.clone();
    update::spawn_checker(move |m| { let _ = p_upd.send_event(Ev::Upd(m)); });

    let mut pending: Option<Release> = None;
    let mut page_ready = false;
    let mut installing = false;
    let mut last_save = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        let _keep = &ctx;
        let save = |win: &tao::window::Window| {
            let mut v = load_settings();
            let max = win.is_maximized();
            v["max"] = json!(max);
            if !max && win.fullscreen().is_none() {
                let sz = win.inner_size();
                v["w"] = json!(sz.width); v["h"] = json!(sz.height);
                if let Ok(pos) = win.outer_position() { v["x"] = json!(pos.x); v["y"] = json!(pos.y); }
            }
            save_settings(&v);
        };
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                save(&window);
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent { event: WindowEvent::Resized(_) | WindowEvent::Moved(_), .. } => {
                if last_save.elapsed().as_secs() >= 2 { save(&window); last_save = std::time::Instant::now(); }
            }
            Event::UserEvent(ev) => match ev {
                Ev::Ready => {
                    page_ready = true;
                    if let Some(r) = &pending { show_update(&webview, r); }
                }
                Ev::Upd(UpdMsg::Available(r)) => {
                    if page_ready && !installing { show_update(&webview, &r); }
                    pending = Some(r);
                }
                Ev::InstallUpdate => {
                    if let (Some(r), false) = (pending.clone(), installing) {
                        installing = true;
                        let p = proxy.clone();
                        update::install(r, move |m| { let _ = p.send_event(Ev::Upd(m)); });
                    }
                }
                Ev::Upd(UpdMsg::Progress(f)) => { let _ = webview.evaluate_script(&format!("window.SovMapDesktop&&SovMapDesktop.progress({f})")); }
                Ev::Upd(UpdMsg::Failed(e)) => {
                    installing = false;
                    let _ = webview.evaluate_script(&format!("window.SovMapDesktop&&SovMapDesktop.failed({})", json!(e)));
                }
                Ev::Upd(UpdMsg::Ready) => { save(&window); update::restart(); }
                Ev::Fullscreen => {
                    window.set_fullscreen(if window.fullscreen().is_some() { None } else { Some(Fullscreen::Borderless(None)) });
                }
                Ev::Open(url) => open_external(&url),
            },
            _ => {}
        }
    });
}

fn show_update(webview: &wry::WebView, r: &Release) {
    let js = json!({ "version": r.version, "size": r.size, "notes": r.notes });
    let _ = webview.evaluate_script(&format!("window.SovMapDesktop&&SovMapDesktop.update({js})"));
}
