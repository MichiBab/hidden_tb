use std::ffi::c_void;
use windows::Win32::Foundation::POINT;
use windows::Win32::UI::Shell::{IAppVisibility, APPBARDATA};
use windows::Win32::{Foundation, UI::WindowsAndMessaging::*};
use Foundation::HWND;
use Foundation::RECT;

use crate::monitors;
use crate::taskbar::Taskbar;

pub const _AUTOHIDE: isize = 0x01;
pub const _ALWAYS_ON_TOP: isize = 0x02;

#[derive(Default, Debug)]
pub struct FormEntry {
    pub hwnd: HWND,
    pub rect: RECT,
}

#[derive(Default, Debug)]
pub struct TaskbarData {
    /*
    taskbar is on top.

    tray depends on taskbar.
    rebar depends on taskbar.

    applist depends on rebar.

    apps depend on applist.
    */
    pub taskbar: Option<FormEntry>,

    pub tray: Option<FormEntry>,
    pub rebar: Option<FormEntry>,

    pub applist: Option<FormEntry>,

    pub apps: Option<FormEntry>,
}

impl FormEntry {
    /* Safety: Don't call new with a str {name} that contains a \0 terminating character. */
    unsafe fn new(dependent_hwnd: HWND, name: &str) -> Option<FormEntry> {
        let hwnd = windows::Win32::UI::WindowsAndMessaging::FindWindowExA(
            dependent_hwnd,
            HWND_TOP,
            string_to_pcstr(name),
            windows::core::PCSTR::null(),
        );
        let mut rect = windows::Win32::Foundation::RECT::default();
        let erg = windows::Win32::UI::WindowsAndMessaging::GetWindowRect(hwnd, &mut rect);

        if erg.as_bool() {
            return Some(FormEntry { hwnd, rect });
        }
        /* todo: log error */
        None
    }
}

impl TaskbarData {
    pub fn new() -> Self {
        let mut data = TaskbarData::default();
        /* Safety: Each string does not contain a \0 character, so the string_to_pcstr will not create an unsafe string representation */
        unsafe {
            data.taskbar = FormEntry::new(HWND_TOP, "Shell_TrayWnd");
            if let Some(taskbar) = &data.taskbar {
                data.tray = FormEntry::new(taskbar.hwnd, "TrayNotifyWnd");
                data.rebar = FormEntry::new(taskbar.hwnd, "ReBarWindow32");
                if let Some(rebar) = &data.rebar {
                    data.applist = FormEntry::new(rebar.hwnd, "MSTaskSwWClass");
                    if let Some(applist) = &data.applist {
                        data.apps = FormEntry::new(applist.hwnd, "MSTaskListWClass");
                    }
                }
            }
        }
        data
    }
}

/* if the input str contains \0, this function will be unsafe */
#[inline]
unsafe fn string_to_pcstr(input: &str) -> windows::core::PCSTR {
    windows::core::PCSTR::from_raw(format!("{input}\0").as_bytes().as_ptr())
}

pub fn get_point_in_rect(rect: &RECT, point: &POINT) -> bool {
    /* safety: both have to be checked to be valid as its done in taskbar::is_hovering_on_tb */
    unsafe { windows::Win32::Graphics::Gdi::PtInRect(rect, *point).as_bool() }
}

fn get_rect_of_work_area() -> RECT {
    let mut workarea_rect = RECT::default();
    unsafe {
        if !windows::Win32::UI::WindowsAndMessaging::SystemParametersInfoW(
            SPI_GETWORKAREA,
            0,
            Some(&mut workarea_rect as *mut _ as *mut c_void),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        )
        .as_bool()
        {
            /* todo: log error */
            eprintln!("could not call get workarea!");
        }
    }
    workarea_rect
}

fn compare_rect_to_workspace_region_for_autohide(current_rect: &RECT) -> bool {
    let workarea_rect = get_rect_of_work_area();
    workarea_rect.bottom - workarea_rect.top == current_rect.bottom - 1 - current_rect.top
}

unsafe fn check_and_set_transparency_style(hwnd: &HWND) {
    /* as i32 */
    static __GWL_EXSTYLE: i32 = -20;
    static __WS_EX_TRANSPARENT: i32 = 0x00000020;

    /* check if the style is set to enable transparency first */
    let current_style = windows::Win32::UI::WindowsAndMessaging::GetWindowLongA(*hwnd, GWL_EXSTYLE);
    if (current_style & WS_EX_LAYERED.0 as i32) != WS_EX_LAYERED.0 as i32 {
        /* set the style to enable transparency */
        windows::Win32::UI::WindowsAndMessaging::SetWindowLongA(
            *hwnd,
            GWL_EXSTYLE,
            WS_EX_LAYERED.0 as i32,
        );
    }
}

pub fn set_window_alpha(hwnd: &HWND, value: u8) {
    unsafe {
        check_and_set_transparency_style(hwnd);
        if !(windows::Win32::UI::WindowsAndMessaging::SetLayeredWindowAttributes(
            *hwnd, None, value, LWA_ALPHA,
        ))
        .as_bool()
        {
            /*todo: log error */
            eprintln!("could not change taskbar alpha");
        }
    };
}

pub fn set_handle_to_topmost(hwnd: &HWND) {
    unsafe {
        SetWindowPos(*hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE);
    }
}

pub fn set_app_bar_state(hwnd: &HWND, option: isize) {
    unsafe {
        let mut msg = APPBARDATA::default();
        msg.cbSize = std::mem::size_of::<APPBARDATA>() as u32;
        msg.hWnd = *hwnd;
        msg.lParam = windows::Win32::Foundation::LPARAM(option);
        windows::Win32::UI::Shell::SHAppBarMessage(
            windows::Win32::UI::Shell::ABM_SETSTATE,
            &mut msg,
        );
    }
}

/* this function checks if each monitor is configured correctly for the autohide feature. */
pub fn check_and_update_workspace_region_for_autohide(taskbar: &Taskbar) {
    let mut change_in_workspace = false;
    let monitors = monitors::get_monitors();
    for primary_monitor in monitors.iter().filter(|m| m.is_primary()) {
        let display_area = primary_monitor.get_display();
        if !compare_rect_to_workspace_region_for_autohide(&display_area) {
            /* work area is not configured correctly. Setting to autohide. */
            println!("calling set window region");
            set_window_region_for_autohide(&display_area);
            change_in_workspace = true;
        }
    }
    if change_in_workspace {
        taskbar.refresh_area_and_set_on_top();
    };
}

fn set_window_region_for_autohide(rect: &RECT) {
    let mut mut_rect = rect.clone();
    mut_rect.bottom -= 1;
    unsafe {
        if windows::Win32::UI::WindowsAndMessaging::SystemParametersInfoW(
            SPI_SETWORKAREA,
            0,
            Some(&mut mut_rect as *mut _ as *mut c_void),
            SPIF_SENDCHANGE | SPIF_UPDATEINIFILE,
        )
        .as_bool()
        {}
        /* this happens sometimes on windows 22h2. Call again with spif change */
        if windows::Win32::UI::WindowsAndMessaging::SystemParametersInfoW(
            SPI_SETWORKAREA,
            0,
            Some(&mut mut_rect as *mut _ as *mut c_void),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        )
        .as_bool()
        {
            return;
        }

        /* no call worked, todo: log error */
        eprint!("failed to set workspace area");
    }
    /* */
}

pub fn initialize_windows_calls() {
    unsafe {
        /* Initialize system com to retrieve taskbar state in get start menu open function. Safety: None as parameter. */
        if let Err(_) = windows::Win32::System::Com::CoInitialize(None) { /* todo: log error */ }
    }
}

pub fn get_start_menu_open() -> bool {
    let val = windows::core::GUID::from("7E5FE3D9-985F-4908-91F9-EE19F9FD1514");
    unsafe {
        let start_menu_result: Result<IAppVisibility, _> =
            windows::Win32::System::Com::CoCreateInstance(
                &val,
                None,
                windows::Win32::System::Com::CLSCTX_INPROC_SERVER,
            );
        if let Ok(start_menu) = start_menu_result {
            if let Ok(result) = start_menu.IsLauncherVisible() {
                return result.as_bool();
            }
        }
    }
    false
}

pub fn get_cursor_pos() -> Option<POINT> {
    let mut point = POINT::default();
    /* Safety: returning none if the cursor pos can not be retrieved. */
    unsafe {
        if windows::Win32::UI::WindowsAndMessaging::GetCursorPos(&mut point).as_bool() {
            return Some(point);
        }
        /* todo: log error */
    }
    None
}
