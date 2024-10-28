use std::mem;
use windows::Win32::Foundation::RECT;

#[derive(Debug)]
pub struct Monitor {
    display: RECT,
    is_primary: bool,
}

impl Monitor {
    pub fn get_display(&self) -> RECT {
        self.display
    }
    pub fn is_primary(&self) -> bool {
        self.is_primary
    }
}

pub fn get_monitors() -> Vec<Monitor> {
    let mut monitors = vec![];
    println!("Enumerating monitors");
    for monitor in enumerate_monitors() {
        /* currently only the display form is needed. Many more infos can be retrieved. */
        monitors.push(Monitor {
            display: RECT {
                left: monitor.monitorInfo.rcMonitor.left,
                top: monitor.monitorInfo.rcMonitor.top,
                right: monitor.monitorInfo.rcMonitor.right,
                bottom: monitor.monitorInfo.rcMonitor.bottom,
            },
            is_primary: monitor.monitorInfo.dwFlags
                == windows::Win32::UI::WindowsAndMessaging::MONITORINFOF_PRIMARY,
        })
    }
    println!("Monitors: {:?}", monitors);
    monitors
}

fn enumerate_monitors() -> Vec<windows::Win32::Graphics::Gdi::MONITORINFOEXW> {
    let mut monitors = Vec::<windows::Win32::Graphics::Gdi::MONITORINFOEXW>::new();
    let monitors_ptr = Box::into_raw(Box::new(monitors)); // Allocate and get a raw pointer

    unsafe {
        let data = windows::Win32::Foundation::LPARAM(monitors_ptr as isize); // Correct cast to `LPARAM`
        windows::Win32::Graphics::Gdi::EnumDisplayMonitors(
            None,
            None,
            Some(monitor_callback),
            data,
        );

        // After the callback, we need to take ownership back of the pointer
        let boxed_monitors = Box::from_raw(monitors_ptr);
        *boxed_monitors // Deref the box back into the original Vec
    }
}

unsafe extern "system" fn monitor_callback(
    monitor: windows::Win32::Graphics::Gdi::HMONITOR,
    _: windows::Win32::Graphics::Gdi::HDC,
    _: *mut windows::Win32::Foundation::RECT,
    userdata: windows::Win32::Foundation::LPARAM,
) -> windows::Win32::Foundation::BOOL {
    let monitors: &mut Vec<windows::Win32::Graphics::Gdi::MONITORINFOEXW> =
        &mut *(userdata.0 as *mut Vec<windows::Win32::Graphics::Gdi::MONITORINFOEXW>);

    let mut monitor_info: windows::Win32::Graphics::Gdi::MONITORINFOEXW = mem::zeroed();
    monitor_info.monitorInfo.cbSize =
        mem::size_of::<windows::Win32::Graphics::Gdi::MONITORINFOEXW>() as u32;
    let monitor_info_ptr = <*mut _>::cast(&mut monitor_info);

    // Call GetMonitorInfoW
    if windows::Win32::Graphics::Gdi::GetMonitorInfoW(monitor, monitor_info_ptr).as_bool() {
        monitors.push(monitor_info)
    }
    true.into()
}
