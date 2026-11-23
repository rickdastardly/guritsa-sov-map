//! Intel channels: tails EVE's chat logs (Documents\EVE\logs\Chatlogs, UTF-16 text files) and
//! sends new text to the page, which does all the parsing (the same code the website uses).
//! Only logs touched in the last 6 hours are watched; nothing is stored or sent anywhere.
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

fn dirs() -> Vec<PathBuf> {
    let mut v = vec![];
    if let Some(u) = directories::UserDirs::new() {
        if let Some(d) = u.document_dir() { v.push(d.join("EVE").join("logs").join("Chatlogs")); }
        let h = u.home_dir();
        v.push(h.join("Documents").join("EVE").join("logs").join("Chatlogs"));
        v.push(h.join("OneDrive").join("Documents").join("EVE").join("logs").join("Chatlogs"));
    }
    v.sort();
    v.dedup();
    v.into_iter().filter(|p| p.is_dir()).collect()
}

fn utf16(b: &[u8]) -> String {
    let u: Vec<u16> = b.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
    String::from_utf16_lossy(&u)
}

fn read_range(p: &Path, from: u64, to: u64) -> Option<Vec<u8>> {
    if to <= from { return None; }
    let mut f = File::open(p).ok()?;
    f.seek(SeekFrom::Start(from)).ok()?;
    let mut buf = vec![0u8; (to - from) as usize];
    f.read_exact(&mut buf).ok()?;
    Some(buf)
}

fn name_of(p: &Path) -> String { p.file_name().and_then(|n| n.to_str()).unwrap_or("").to_owned() }

/// Start tailing. `send` gets a JSON array of {f, text, init}. Setting the returned flag makes
/// the thread re-send headers and Local (after the page reloads).
pub fn spawn(send: impl Fn(String) + Send + 'static) -> Arc<AtomicBool> {
    let resync = Arc::new(AtomicBool::new(false));
    let flag = resync.clone();
    std::thread::spawn(move || {
        let mut files: HashMap<PathBuf, u64> = HashMap::new();
        let mut last_scan: Option<Instant> = None;
        let mut first_done = false;
        loop {
            if flag.swap(false, Ordering::SeqCst) { files.clear(); first_done = false; last_scan = None; }
            let mut batch: Vec<Value> = vec![];
            if last_scan.map(|t| t.elapsed() >= Duration::from_secs(15)).unwrap_or(true) {
                last_scan = Some(Instant::now());
                for d in dirs() {
                    let Ok(rd) = fs::read_dir(&d) else { continue };
                    for e in rd.flatten() {
                        let p = e.path();
                        let txt = p.extension().and_then(|x| x.to_str()).map(|x| x.eq_ignore_ascii_case("txt")).unwrap_or(false);
                        if !txt || files.contains_key(&p) { continue; }
                        let Ok(md) = e.metadata() else { continue };
                        let recent = md.modified().ok().and_then(|m| m.elapsed().ok()).map(|a| a.as_secs() < 6 * 3600).unwrap_or(false);
                        if !recent { continue; }
                        // A log that appears after we started is new: read it from the top.
                        if first_done { files.insert(p, 0); continue; }
                        let size = md.len() & !1;
                        let name = name_of(&p);
                        if let Some(b) = read_range(&p, 0, size.min(2048)) { batch.push(json!({ "f": name, "text": utf16(&b), "init": true })); }
                        if name.to_ascii_lowercase().starts_with("local_") {
                            let from = size.saturating_sub(65536) & !1;
                            if let Some(b) = read_range(&p, from, size) { batch.push(json!({ "f": name, "text": utf16(&b), "init": true })); }
                        }
                        files.insert(p, size);
                    }
                }
                first_done = true;
            }
            for (p, off) in files.iter_mut() {
                let Ok(md) = fs::metadata(p) else { continue };
                let size = md.len() & !1;
                if size < *off { *off = 0; }
                if size == *off { continue; }
                let to = size.min(*off + 1_048_576);
                if let Some(b) = read_range(p, *off, to) { batch.push(json!({ "f": name_of(p), "text": utf16(&b), "init": false })); }
                *off = to;
            }
            if !batch.is_empty() { send(Value::Array(batch).to_string()); }
            std::thread::sleep(Duration::from_millis(1000));
        }
    });
    resync
}
