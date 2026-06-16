// Small platform shims for things the desktop build does natively but the web
// (wasm) build must handle differently or skip.

/// Open a URL in the user's browser. On the desktop this launches the default
/// handler; on the web the app already *is* in a browser, so for now this is a
/// no-op (a real implementation would call `window.open`).
#[cfg(not(target_arch = "wasm32"))]
pub fn open_url(url: &str) {
    let _ = open::that(url);
}

#[cfg(target_arch = "wasm32")]
pub fn open_url(_url: &str) {
    // TODO(web): web_sys::window().open_with_url(_url)
}

/// Turn an `rfd` file handle into a `PathBuf` the rest of the app keys on.
///
/// Desktop returns the real filesystem path. The browser has no path, so we
/// synthesize one from the file name — enough for the app to compile and track
/// the document name; actual byte I/O on the web reads the handle directly
/// (a follow-up).
#[cfg(not(target_arch = "wasm32"))]
pub fn handle_path(h: &rfd::FileHandle) -> std::path::PathBuf {
    h.path().to_path_buf()
}

#[cfg(target_arch = "wasm32")]
pub fn handle_path(h: &rfd::FileHandle) -> std::path::PathBuf {
    std::path::PathBuf::from(h.file_name())
}
