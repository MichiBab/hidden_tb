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
    monitors
}

fn enumerate_monitors() -> Vec<windows::Win32::Graphics::Gdi::MONITORINFOEXW> {
    let monitors = Vec::<windows::Win32::Graphics::Gdi::MONITORINFOEXW>::new();
    unsafe {
        let data: windows::Win32::Foundation::LPARAM = std::mem::transmute(&monitors);
        windows::Win32::Graphics::Gdi::EnumDisplayMonitors(None, None, Some(monitor_callback), data)
    };
    monitors
}

unsafe extern "system" fn monitor_callback(
    monitor: windows::Win32::Graphics::Gdi::HMONITOR,
    _: windows::Win32::Graphics::Gdi::HDC,
    _: *mut windows::Win32::Foundation::RECT,
    userdata: windows::Win32::Foundation::LPARAM,
) -> windows::Win32::Foundation::BOOL {
    let monitors: &mut Vec<windows::Win32::Graphics::Gdi::MONITORINFOEXW> =
        mem::transmute(userdata);
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
