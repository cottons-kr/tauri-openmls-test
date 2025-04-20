// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use openmls::versions::ProtocolVersion;
use sysinfo::
    System
;

#[tauri::command]
fn greet(name: &str) -> String {
    let protocol_version = ProtocolVersion::default();  
    let mut sys = System::new_all();

    sys.refresh_all();

    let hostname = System::host_name();
    let kernel_version = System::kernel_long_version();
    let cpu_count = sys.cpus().len();

    return format!(
        "Hello, {}! You've been greeted from Rust! The protocol version is: {:?}. The kernel version is: {:?}. The hostname is: {:?}. The CPU count is: {}",
        name, protocol_version, kernel_version, hostname, cpu_count
    )
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
