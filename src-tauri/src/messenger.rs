use tauri::Handle;

pub fn emit_err<T: 'static>(webview_handle: &Handle<T>, message: &str) {
  tauri::event::emit(
    webview_handle,
    String::from("error"),
    Some(String::from(message)),
  );
}
