//! Self-update. Every 6 hours the app reads a tiny file (app/latest.json, ~300 bytes).
//! Only when there is a newer version does it offer to download the new .exe; it then swaps
//! itself in place (Windows lets a running .exe be renamed) and restarts. The old copy is
//! deleted on the next start, so nothing piles up on disk.
use std::io::Read;
use std::time::Duration;

pub const LATEST: &str = "https://gurista.co.uk/sov-map/app/latest.json";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Debug)]
pub struct Release { pub version: String, pub url: String, pub size: u64, pub notes: String }

#[derive(Debug)]
pub enum UpdMsg { Available(Release), Progress(f32), Failed(String), Ready }

pub fn newer(a: &str, b: &str) -> bool {
    let p = |s: &str| -> Vec<u64> { s.trim_start_matches('v').split('.').map(|x| x.parse().unwrap_or(0)).collect() };
    let (a, b) = (p(a), p(b));
    for k in 0..a.len().max(b.len()) {
        let (x, y) = (a.get(k).copied().unwrap_or(0), b.get(k).copied().unwrap_or(0));
        if x != y { return x > y; }
    }
    false
}

fn agent() -> ureq::Agent {
    ureq::AgentBuilder::new().timeout(Duration::from_secs(60)).user_agent(concat!("GuristaSovMap-desktop/", env!("CARGO_PKG_VERSION"))).build()
}

/// Remove the copy left behind by the last update.
pub fn cleanup() {
    if let Ok(exe) = std::env::current_exe() { let _ = std::fs::remove_file(exe.with_extension("old")); let _ = std::fs::remove_file(exe.with_extension("new")); }
}

pub fn spawn_checker(tx: impl Fn(UpdMsg) + Send + 'static) {
    std::thread::spawn(move || {
        let a = agent();
        loop {
            if let Some(v) = a.get(LATEST).call().ok().and_then(|r| r.into_string().ok()).and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok()) {
                let ver = v["version"].as_str().unwrap_or("").to_owned();
                if !ver.is_empty() && newer(&ver, VERSION) {
                    let url = v["exe"].as_str().map(|e| if e.starts_with("http") { e.to_owned() } else { format!("https://gurista.co.uk/sov-map/app/{e}") }).unwrap_or_default();
                    tx(UpdMsg::Available(Release { version: ver, url, size: v["size"].as_u64().unwrap_or(0), notes: v["notes"].as_str().unwrap_or("").to_owned() }));
                }
            }
            std::thread::sleep(Duration::from_secs(6 * 3600));
        }
    });
}

/// Download and swap in the new exe. Sends Ready when the app should restart.
pub fn install(r: Release, tx: impl Fn(UpdMsg) + Send + 'static) {
    std::thread::spawn(move || {
        let res = (|| -> Result<(), String> {
            let exe = std::env::current_exe().map_err(|e| e.to_string())?;
            let resp = agent().get(&r.url).call().map_err(|e| e.to_string())?;
            let total = resp.header("Content-Length").and_then(|s| s.parse::<u64>().ok()).unwrap_or(r.size).max(1);
            let mut rd = resp.into_reader().take(200 * 1024 * 1024);
            let mut buf = Vec::with_capacity(total as usize);
            let mut chunk = vec![0u8; 64 * 1024];
            loop {
                let n = rd.read(&mut chunk).map_err(|e| e.to_string())?;
                if n == 0 { break; }
                buf.extend_from_slice(&chunk[..n]);
                tx(UpdMsg::Progress(buf.len() as f32 / total as f32));
            }
            if r.size > 0 && buf.len() as u64 != r.size { return Err(format!("download incomplete ({} of {} bytes)", buf.len(), r.size)); }
            if buf.len() < 1_000_000 || &buf[0..2] != b"MZ" { return Err("download isn't a Windows program".into()); }
            let new = exe.with_extension("new");
            let old = exe.with_extension("old");
            std::fs::write(&new, &buf).map_err(|e| e.to_string())?;
            let _ = std::fs::remove_file(&old);
            std::fs::rename(&exe, &old).map_err(|e| e.to_string())?;
            if let Err(e) = std::fs::rename(&new, &exe) { let _ = std::fs::rename(&old, &exe); return Err(e.to_string()); }
            Ok(())
        })();
        tx(match res { Ok(()) => UpdMsg::Ready, Err(e) => UpdMsg::Failed(e) });
    });
}

pub fn restart() -> ! {
    if let Ok(exe) = std::env::current_exe() { let _ = std::process::Command::new(exe).spawn(); }
    std::process::exit(0);
}

#[cfg(test)]
mod tests {
    #[test]
    fn versions() { assert!(super::newer("0.2.1", "0.2.0")); assert!(super::newer("0.10.0", "0.9.9")); assert!(!super::newer("0.2.1", "0.2.1")); assert!(!super::newer("0.2.0", "0.2.1")); }
}
