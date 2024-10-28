use std::path::PathBuf;
use std::process::Command;
use windows::core::PWSTR;
use windows::Win32::Foundation::*;
use windows::Win32::System::Diagnostics::ToolHelp::*;
use windows::Win32::System::ProcessStatus::*;
use windows::Win32::System::Threading::*;

#[derive(Debug)]
pub enum RestartError {
    ProcessNotFound,
    FailedToTerminate,
    FailedToStart,
}

pub fn restart_process(process_name: &str) -> Result<(), RestartError> {
    // Find the process and get its full path
    let process_path =
        unsafe { find_process_path(process_name).ok_or(RestartError::ProcessNotFound) }?;

    println!("Found process path: {:?}", process_path);

    // Terminate all instances of the process
    unsafe { terminate_processes(process_name).map_err(|_| RestartError::FailedToTerminate) }?;

    // Start the process again
    unsafe { start_process(&process_path).map_err(|_| RestartError::FailedToStart) }?;

    Ok(())
}

unsafe fn find_process_path(process_name: &str) -> Option<PathBuf> {
    let Ok(h_snapshot) = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) else {
        return None;
    };

    let mut process_entry = PROCESSENTRY32W {
        dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
        ..Default::default()
    };

    if Process32FirstW(h_snapshot, &mut process_entry).is_err() {
        return None;
    }

    loop {
        let current_name = String::from_utf16_lossy(
            &process_entry.szExeFile[..process_entry
                .szExeFile
                .iter()
                .position(|&x| x == 0)
                .unwrap_or(process_entry.szExeFile.len())],
        );

        if current_name.to_lowercase() == process_name.to_lowercase() {
            let process_handle = OpenProcess(
                PROCESS_QUERY_LIMITED_INFORMATION,
                false,
                process_entry.th32ProcessID,
            );

            if let Ok(process_handle) = process_handle {
                let mut buffer = [0u16; MAX_PATH as usize];
                let mut size = buffer.len() as u32;

                if QueryFullProcessImageNameW(
                    process_handle,
                    PROCESS_NAME_FORMAT(0),
                    PWSTR(buffer.as_mut_ptr()),
                    &mut size,
                )
                .is_ok()
                {
                    let path = String::from_utf16_lossy(&buffer[..size as usize]);
                    return Some(PathBuf::from(path));
                }
            }
        }

        if Process32NextW(h_snapshot, &mut process_entry).is_err() {
            break;
        }
    }

    None
}

unsafe fn terminate_processes(process_name: &str) -> Result<(), ()> {
    let Ok(h_snapshot) = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) else {
        return Err(());
    };

    let mut process_entry = PROCESSENTRY32W {
        dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
        ..Default::default()
    };

    if Process32FirstW(h_snapshot, &mut process_entry).is_err() {
        return Err(());
    }

    let mut terminated_any = false;

    loop {
        let current_name = String::from_utf16_lossy(
            &process_entry.szExeFile[..process_entry
                .szExeFile
                .iter()
                .position(|&x| x == 0)
                .unwrap_or(process_entry.szExeFile.len())],
        );

        if current_name.to_lowercase() == process_name.to_lowercase() {
            println!("Terminating process ID: {}", process_entry.th32ProcessID);

            let process_handle = OpenProcess(PROCESS_TERMINATE, false, process_entry.th32ProcessID);

            if process_handle.is_ok() && TerminateProcess(process_handle.unwrap(), 0).is_ok() {
                terminated_any = true;
            }
        }

        if Process32NextW(h_snapshot, &mut process_entry).is_err() {
            break;
        }
    }

    if terminated_any {
        Ok(())
    } else {
        Err(())
    }
}
use windows::Win32::UI::Shell::{ShellExecuteW, SEE_MASK_NOCLOSEPROCESS};

fn escape_powershell_string(s: &str) -> String {
    s.replace('`', "``").replace('"', "`\"")
}

unsafe fn start_process(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let path_str = path.to_string_lossy();

    let mut startup_info: STARTUPINFOW = std::mem::zeroed();
    startup_info.cb = std::mem::size_of::<STARTUPINFOW>() as u32;

    let mut process_info: PROCESS_INFORMATION = std::mem::zeroed();

    // Convert path to wide string
    let wide_path: Vec<u16> = path_str.encode_utf16().chain(std::iter::once(0)).collect();

    println!("Starting process: {:?}", path_str);

    // Escape the file path for PowerShell.
    let escaped_path = escape_powershell_string(&path_str);

    // Construct the PowerShell command.
    let command = format!("Start-Process -FilePath \"{}\" -Verb Open", escaped_path);

    // Execute the PowerShell command.
    let status = Command::new("powershell")
        .arg("-Command")
        .arg(&command)
        .status();

    if status.is_err() {
        println!("Failed to start process: {:?}", status);
        return Err(Box::new(std::io::Error::last_os_error()));
    }

    Ok(())
}
