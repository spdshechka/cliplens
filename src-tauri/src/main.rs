#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::Write;
use std::process::{Command, Stdio};

#[derive(serde::Serialize)]
struct DepStatus {
    cliphist: bool,
    wl_copy: bool,
}

#[derive(serde::Serialize)]
struct ClipEntry {
    raw: String,
    id: String,
    preview: String,
    is_binary: bool,
}

#[derive(serde::Serialize)]
struct ImagePreview {
    data_url: String,
    format: String,
    size_bytes: usize,
}

fn binary_ratio(bytes: &[u8]) -> f64 {
    let non_print = bytes
        .iter()
        .filter(|&&b| b < 0x09 || (b > 0x0d && b < 0x20) || b == 0x7f)
        .count();
    non_print as f64 / bytes.len().max(1) as f64
}

fn detect_image_format(bytes: &[u8]) -> Option<&'static str> {
    if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        return Some("png");
    }
    if bytes.starts_with(b"\xff\xd8\xff") {
        return Some("jpeg");
    }
    if bytes.len() >= 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WEBP" {
        return Some("webp");
    }
    if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
        return Some("gif");
    }
    None
}

fn base64_encode(data: &[u8]) -> String {
    const T: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out: Vec<u8> = Vec::with_capacity((data.len() + 2) / 3 * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as usize;
        let b1 = chunk.get(1).copied().unwrap_or(0) as usize;
        let b2 = chunk.get(2).copied().unwrap_or(0) as usize;
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(T[(n >> 18) & 0x3f]);
        out.push(T[(n >> 12) & 0x3f]);
        out.push(if chunk.len() > 1 { T[(n >> 6) & 0x3f] } else { b'=' });
        out.push(if chunk.len() > 2 { T[n & 0x3f] } else { b'=' });
    }
    unsafe { String::from_utf8_unchecked(out) }
}

fn run_cliphist_decode(entry: &str) -> Result<Vec<u8>, String> {
    let mut child = Command::new("cliphist")
        .arg("decode")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn cliphist decode: {e}"))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(entry.as_bytes())
            .map_err(|e| format!("Failed to write to cliphist stdin: {e}"))?;
    }

    let out = child
        .wait_with_output()
        .map_err(|e| format!("cliphist decode failed: {e}"))?;

    if !out.status.success() {
        return Err("cliphist decode returned non-zero exit code".to_string());
    }

    Ok(out.stdout)
}

#[tauri::command]
fn list_clipboard_items() -> Result<Vec<ClipEntry>, String> {
    let output = Command::new("cliphist")
        .arg("list")
        .output()
        .map_err(|e| format!("Failed to run cliphist: {e}"))?;

    let entries = output
        .stdout
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let is_binary = binary_ratio(line) > 0.05;
            let raw = String::from_utf8_lossy(line).into_owned();
            let (id, preview) = if let Some(pos) = raw.find('\t') {
                (raw[..pos].to_string(), raw[pos + 1..].to_string())
            } else {
                (raw.clone(), raw.clone())
            };
            let preview = if is_binary {
                String::new()
            } else {
                preview.chars().take(512).collect()
            };
            ClipEntry { raw, id, preview, is_binary }
        })
        .collect();

    Ok(entries)
}

#[tauri::command]
fn copy_clipboard_item(entry: String) -> Result<(), String> {
    let decoded = run_cliphist_decode(&entry)?;

    let mut copy = Command::new("wl-copy")
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn wl-copy: {e}"))?;

    if let Some(mut stdin) = copy.stdin.take() {
        stdin
            .write_all(&decoded)
            .map_err(|e| format!("Failed to write to wl-copy stdin: {e}"))?;
    }

    copy.wait().map_err(|e| format!("wl-copy failed: {e}"))?;

    Ok(())
}

#[tauri::command]
fn preview_image_item(entry: String) -> Result<ImagePreview, String> {
    const MAX_BYTES: usize = 10 * 1024 * 1024;

    let bytes = run_cliphist_decode(&entry)?;
    let size_bytes = bytes.len();

    if size_bytes > MAX_BYTES {
        return Err(format!(
            "Image too large ({:.1} MiB) — preview skipped",
            size_bytes as f64 / 1024.0 / 1024.0
        ));
    }

    let fmt = detect_image_format(&bytes)
        .ok_or_else(|| "Unknown or unsupported image format".to_string())?;

    let data_url = format!("data:image/{fmt};base64,{}", base64_encode(&bytes));

    Ok(ImagePreview {
        data_url,
        format: fmt.to_uppercase(),
        size_bytes,
    })
}

#[tauri::command]
fn clear_clipboard_history() -> Result<(), String> {
    let out = Command::new("cliphist")
        .arg("wipe")
        .output()
        .map_err(|e| format!("Failed to run cliphist wipe: {e}"))?;

    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr);
        return Err(format!("cliphist wipe failed: {err}"));
    }

    Ok(())
}

#[tauri::command]
fn check_dependencies() -> DepStatus {
    let has = |bin: &str| {
        Command::new("which")
            .arg(bin)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    };
    DepStatus {
        cliphist: has("cliphist"),
        wl_copy: has("wl-copy"),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_clipboard_items,
            copy_clipboard_item,
            preview_image_item,
            clear_clipboard_history,
            check_dependencies
        ])
        .run(tauri::generate_context!())
        .expect("error while running ClipLens");
}
