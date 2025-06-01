use windows::Win32::Foundation::{HANDLE, CloseHandle};
use windows::Win32::System::Threading::*;
use windows::Win32::System::ProcessStatus::*;
use windows::Win32::System::Diagnostics::ToolHelp::*;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

pub fn list_running_processes() -> Vec<String> {
    let mut results = Vec::new();
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).unwrap();
        let mut entry = PROCESSENTRY32 {
            dwSize: std::mem::size_of::<PROCESSENTRY32>() as u32,
            ..Default::default()
        };

        if Process32First(snapshot, &mut entry).as_bool() {
            loop {
                let name = OsString::from_wide(&entry.szExeFile);
                results.push(name.to_string_lossy().to_string());

                if !Process32Next(snapshot, &mut entry).as_bool() {
                    break;
                }
            }
        }
        CloseHandle(snapshot);
    }
    results
}
