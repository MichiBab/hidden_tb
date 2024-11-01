use std::ffi::c_void;
use windows::core::PWSTR;
use windows::Win32::Foundation::{BOOL, LRESULT, MAX_PATH, POINT};
use windows::Win32::Graphics::Gdi::{CombineRgn, CreateRoundRectRgn, SetWindowRgn};
use windows::Win32::System::Threading::{
    GetCurrentProcessId, OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_FORMAT,
    PROCESS_QUERY_INFORMATION, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_VM_READ,
};
use windows::Win32::UI::Shell::{
    IAppVisibility, SHAppBarMessage, ABM_GETSTATE, ABM_SETSTATE, ABS_AUTOHIDE, APPBARDATA,
};
use windows::Win32::{Foundation, UI::WindowsAndMessaging::*};
use Foundation::HWND;
use Foundation::RECT;

use crate::taskbar::Taskbar;
use crate::tb_settings::TbSettings;
use crate::{monitors, restart_process};

pub const _AUTOHIDE: isize = 0x01;
pub const _ALWAYS_ON_TOP: isize = 0x02;

#[derive(Default, Debug, Clone)]
pub struct FormEntry {
    pub hwnd: HWND,
    pub rect: RECT,
}

#[derive(Default, Debug, Clone)]
pub struct WantedHwnds {
    pub taskbar: bool,
    pub tray: bool,
    pub rebar: bool,
    pub applist: bool,
    pub apps: bool,
}

impl WantedHwnds {
    pub fn new(settings: &TbSettings) -> Self {
        //Currently only the taskbar is needed, so set all other values to false
        //Later on detect through settings if more is needed
        let mut wanted_hwnds = WantedHwnds {
            taskbar: true,
            tray: false,
            rebar: false,
            applist: false,
            apps: false,
        };
        if settings.get_merge_tray()
            || settings.get_merge_widgets()
            || settings.get_enable_dynamic_borders()
        {
            wanted_hwnds.tray = true;
            wanted_hwnds.rebar = true;
            wanted_hwnds.applist = true;
            //apps is currently not used, so we can keep it on false and not depend on it.
            //wanted_hwnds.apps = true;
        }
        wanted_hwnds
    }
}

#[derive(Default, Debug, Clone)]
pub struct TaskbarData {
    /*
    taskbar is on top.

    tray depends on taskbar.
    rebar depends on taskbar.

    applist depends on rebar.

    apps depend on applist.
    */
    pub taskbar: Option<FormEntry>,

    pub resolution: f64,

    pub tray: Option<FormEntry>,
    pub rebar: Option<FormEntry>,

    pub applist: Option<FormEntry>,

    pub apps: Option<FormEntry>,

    pub wanted_hwnds: WantedHwnds,

    pub display_rect: Option<RECT>,
}

impl FormEntry {
    /* Safety: Don't call new with a str {name} that contains a \0 terminating character. */
    unsafe fn new(dependent_hwnd: HWND, name: &str) -> Option<FormEntry> {
        let Ok(hwnd) = windows::Win32::UI::WindowsAndMessaging::FindWindowExA(
            dependent_hwnd,
            HWND_TOP,
            string_to_pcstr(name),
            windows::core::PCSTR::null(),
        ) else {
            return None;
        };
        let mut rect = windows::Win32::Foundation::RECT::default();
        let erg = windows::Win32::UI::WindowsAndMessaging::GetWindowRect(hwnd, &mut rect);

        if erg.is_ok() {
            return Some(FormEntry { hwnd, rect });
        }
        /* todo: log error */
        None
    }
}

unsafe fn get_dpi_from_hwnd(hwnd: &HWND) -> f64 {
    let dpi = windows::Win32::UI::HiDpi::GetDpiForWindow(*hwnd);
    dpi.into()
}

impl TaskbarData {
    pub fn new(wanted: &WantedHwnds) -> Self {
        let mut data = TaskbarData::default();
        /* Safety: Each string does not contain a \0 character, so the string_to_pcstr will not create an unsafe string representation */
        unsafe {
            data.taskbar = None;
            if wanted.taskbar {
                data.taskbar = FormEntry::new(HWND_TOP, "Shell_TrayWnd");
                if let Some(taskbar) = &data.taskbar {
                    data.resolution = get_dpi_from_hwnd(&taskbar.hwnd) / 96.0;

                    data.tray = None;
                    if wanted.tray {
                        data.tray = FormEntry::new(taskbar.hwnd, "TrayNotifyWnd");
                    }
                    data.rebar = None;
                    if wanted.rebar {
                        data.rebar = FormEntry::new(taskbar.hwnd, "ReBarWindow32");
                        if let Some(rebar) = &data.rebar {
                            data.applist = None;
                            if wanted.applist {
                                data.applist = FormEntry::new(rebar.hwnd, "MSTaskSwWClass");
                                if let Some(applist) = &data.applist {
                                    data.apps = None;
                                    if wanted.apps {
                                        data.apps =
                                            FormEntry::new(applist.hwnd, "MSTaskListWClass");
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        data.wanted_hwnds = wanted.clone();
        data
    }

    pub fn contains_none(&self) -> bool {
        (self.applist.is_none() && self.wanted_hwnds.applist)
            || (self.apps.is_none() && self.wanted_hwnds.apps)
            || (self.rebar.is_none() && self.wanted_hwnds.rebar)
            || (self.tray.is_none() && self.wanted_hwnds.tray)
            || (self.taskbar.is_none() && self.wanted_hwnds.taskbar)
    }
}

/* if the input str contains \0, this function will be unsafe */
#[inline]
pub(crate) unsafe fn string_to_pcstr(input: &str) -> windows::core::PCSTR {
    windows::core::PCSTR::from_raw(format!("{input}\0").as_bytes().as_ptr())
}

pub fn get_point_in_rect(rect: &RECT, point: &POINT) -> bool {
    /* safety: both have to be checked to be valid as its done in taskbar::is_hovering_on_tb */
    unsafe { windows::Win32::Graphics::Gdi::PtInRect(rect, *point).as_bool() }
}

pub fn set_window_topmost(hwnd: &HWND) {
    unsafe {
        if let Err(e) = windows::Win32::UI::WindowsAndMessaging::SetWindowPos(
            *hwnd,
            HWND_TOPMOST,
            0,
            0,
            0,
            0,
            windows::Win32::UI::WindowsAndMessaging::SWP_NOMOVE
                | windows::Win32::UI::WindowsAndMessaging::SWP_NOSIZE,
        ) {
            eprintln!("Error setting window to topmost: {:?}", e);
        }
    }
}

pub fn create_rounded_region(
    settings: &TbSettings,
    tb_data: &TaskbarData,
    hovering_over_tray: bool,
    _hovering_over_widgets: bool,
) {
    if let Some(taskbar_entry) = &tb_data.taskbar {
        if let Some(tray_entry) = &tb_data.tray {
            if let Some(applist_entry) = &tb_data.applist {
                unsafe {
                    let resolution = tb_data.resolution;

                    let taskbar_dynamic_region = CreateRoundRectRgn(
                        (((applist_entry.rect.left) as f64
                            - settings.get_margin_offset_left() as f64
                            + settings.get_margin_left() as f64)
                            * resolution) as i32,
                        (resolution as i32) + settings.get_margin_top(),
                        (((applist_entry.rect.right + 2) as f64
                            + settings.get_margin_offset_right() as f64
                            - settings.get_margin_right() as f64)
                            * resolution) as i32,
                        ((taskbar_entry.rect.bottom as f64 + 1.0
                            - settings.get_margin_bottom() as f64
                            - taskbar_entry.rect.top as f64)
                            * resolution) as i32,
                        settings.get_rounded_corners_size(),
                        settings.get_rounded_corners_size(),
                    );

                    let mut show_tray = false;
                    if settings.get_dynamic_borders_show_tray() {
                        show_tray = true;
                    } else if settings.get_dynamic_borders_show_tray_if_disabled_on_hover() {
                        show_tray = hovering_over_tray;
                    }
                    if show_tray {
                        let tray_region = CreateRoundRectRgn(
                            ((tray_entry.rect.left as f64 + settings.get_margin_left() as f64
                                - 3.0)
                                * resolution) as i32,
                            (resolution as i32) + settings.get_margin_top(),
                            ((tray_entry.rect.right as f64 - settings.get_margin_right() as f64)
                                * resolution) as i32,
                            ((taskbar_entry.rect.bottom as f64 + 1.0
                                - settings.get_margin_bottom() as f64
                                - taskbar_entry.rect.top as f64)
                                * resolution) as i32,
                            settings.get_rounded_corners_size(),
                            settings.get_rounded_corners_size(),
                        );
                        CombineRgn(
                            taskbar_dynamic_region,
                            taskbar_dynamic_region,
                            tray_region,
                            windows::Win32::Graphics::Gdi::RGN_COMBINE_MODE(2),
                        );
                    }

                    //println!("calling setWindowRgn");
                    SetWindowRgn(taskbar_entry.hwnd, taskbar_dynamic_region, true);
                }
            }
        }
    }
}

pub fn reset_taskbar(hwnd: &HWND, rect: &RECT) {
    unsafe {
        windows::Win32::Graphics::Gdi::SetWindowRgn(
            *hwnd,
            windows::Win32::Graphics::Gdi::HRGN::default(),
            true,
        );
        if let Err(e) = SetLayeredWindowAttributes(*hwnd, None, 255, LWA_ALPHA) {
            eprintln!("Error setting window to topmost: {:?}", e);
        }
        let mut style = GetWindowLongA(*hwnd, GWL_EXSTYLE);
        if (style & (WS_EX_LAYERED.0 as i32)) == (WS_EX_LAYERED.0 as i32) {
            SetWindowLongA(
                *hwnd,
                GWL_EXSTYLE,
                GetWindowLongA(*hwnd, GWL_EXSTYLE) ^ (WS_EX_LAYERED.0 as i32),
            );
        }
        style = GetWindowLongA(*hwnd, GWL_EXSTYLE);
        if (style & (WS_EX_TRANSPARENT.0 as i32)) == (WS_EX_TRANSPARENT.0 as i32) {
            SetWindowLongA(
                *hwnd,
                GWL_EXSTYLE,
                GetWindowLongA(*hwnd, GWL_EXSTYLE) ^ (WS_EX_TRANSPARENT.0 as i32),
            );
        }
        // reset taskbar region
        reset_window_region(rect);
    }
}

fn get_rect_of_work_area() -> RECT {
    let mut workarea_rect = RECT::default();
    unsafe {
        if windows::Win32::UI::WindowsAndMessaging::SystemParametersInfoW(
            SPI_GETWORKAREA,
            0,
            Some(&mut workarea_rect as *mut _ as *mut c_void),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        )
        .is_err()
        {
            /* todo: log error */
            eprintln!("could not call get workarea!");
        }
    }
    workarea_rect
}

fn compare_rect_to_workspace_region_for_autohide(current_rect: &RECT, top_offset: u32) -> bool {
    let workarea_rect = get_rect_of_work_area();
    workarea_rect.bottom - workarea_rect.top
        == current_rect.bottom - 1 - current_rect.top - (top_offset as i32)
}

pub fn check_and_set_transparency_style(hwnd: &HWND) -> bool {
    unsafe {
        /* as i32 */
        static __GWL_EXSTYLE: i32 = -20;
        static __WS_EX_TRANSPARENT: i32 = 0x00000020;

        /* check if the style is set to enable transparency first */
        let current_style =
            windows::Win32::UI::WindowsAndMessaging::GetWindowLongA(*hwnd, GWL_EXSTYLE);
        if current_style != ((WS_EX_LAYERED.0 as i32) | current_style | (WS_EX_TOOLWINDOW.0 as i32))
        {
            /* set the style to enable transparency */
            windows::Win32::UI::WindowsAndMessaging::SetWindowLongW(
                *hwnd,
                GWL_EXSTYLE,
                (WS_EX_LAYERED.0 as i32) | current_style | (WS_EX_TOOLWINDOW.0 as i32),
            );
            println!("setting taskbar to layered");
            return false;
        }
        true
    }
}

pub fn set_window_alpha(hwnd: &HWND, value: u8) -> bool {
    unsafe {
        if let Err(e) = windows::Win32::UI::WindowsAndMessaging::SetLayeredWindowAttributes(
            *hwnd, None, value, LWA_ALPHA,
        ) {
            /*todo: log error */
            eprintln!("could not change taskbar alpha: {e}");
            return false;
        }
    }
    true
}

pub fn set_handle_to_topmost(hwnd: &HWND) {
    unsafe {
        if let Err(e) = SetWindowPos(*hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE) {
            eprintln!("Error setting window to topmost: {:?}", e);
        }
    }
}

pub fn set_app_bar_state(hwnd: &HWND, option: isize) {
    unsafe {
        let mut msg = APPBARDATA {
            cbSize: std::mem::size_of::<APPBARDATA>() as u32,
            hWnd: *hwnd,
            lParam: windows::Win32::Foundation::LPARAM(option),
            ..Default::default()
        };
        windows::Win32::UI::Shell::SHAppBarMessage(
            windows::Win32::UI::Shell::ABM_SETSTATE,
            &mut msg,
        );
    }
}

/* this function checks if each monitor is configured correctly for the autohide feature. */
pub fn check_and_update_workspace_region_for_autohide(taskbar: &mut Taskbar, top_offset: u32) {
    let mut change_in_workspace = false;
    let monitors = monitors::get_monitors();
    for primary_monitor in monitors.iter().filter(|m| m.is_primary()) {
        let display_area = primary_monitor.get_display();
        taskbar.set_display_area(display_area);
        if !compare_rect_to_workspace_region_for_autohide(&display_area, top_offset) {
            /* work area is not configured correctly. Setting to autohide. */
            //println!("calling set window region");
            set_window_region_for_autohide(&display_area, top_offset);
            change_in_workspace = true;
        }
    }
    if change_in_workspace {
        send_workspace_and_display_change_msg();
        taskbar.refresh_area_and_set_on_top();
        taskbar.send_restarts();
    }
}

const HWND_BROADCAST: HWND = HWND((-1isize) as _);

fn send_workspace_and_display_change_msg() {
    unsafe {
        if let Err(e) = PostMessageW(
            HWND_BROADCAST,
            WM_DISPLAYCHANGE,
            Foundation::WPARAM(0),
            Foundation::LPARAM(0),
        ) {
            dbg!(e);
        }
        if let Err(e) = PostMessageW(
            HWND_BROADCAST,
            WM_SETTINGCHANGE,
            Foundation::WPARAM(0),
            Foundation::LPARAM(0),
        ) {
            dbg!(e);
        }
        println!("Send update message to all windows");
    }
}

fn set_window_region_for_autohide(rect: &RECT, top_offset: u32) {
    let mut mut_rect = *rect;
    mut_rect.bottom -= 1;
    mut_rect.top += top_offset as i32;
    unsafe {
        if call_and_check_set_window_region(
            &mut_rect,
            &[
                SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
                SPIF_SENDWININICHANGE | SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
                SPIF_SENDCHANGE,
                SPIF_UPDATEINIFILE,
            ],
        ) {
            return;
        }
        /* no call worked, todo: log error */
        eprint!("failed to reset workspace area");
    }
    /* */
}

unsafe fn call_and_check_set_window_region(
    rect: &RECT,
    call_options: &[SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS],
) -> bool {
    let mut resetted = false;
    for call_option in call_options {
        let mut mut_rect = *rect;
        if windows::Win32::UI::WindowsAndMessaging::SystemParametersInfoW(
            SPI_SETWORKAREA,
            0,
            Some(&mut mut_rect as *mut _ as *mut c_void),
            *call_option,
        )
        .is_ok()
            && get_rect_of_work_area() == mut_rect
        {
            println!("changed workspace correctly");
            resetted = true;
        }
    }
    resetted
}

fn reset_window_region(rect: &RECT) {
    let mut mut_rect = RECT::default();
    let mut found_primary_display = false;
    for primary_monitor in monitors::get_monitors().iter().filter(|m| m.is_primary()) {
        found_primary_display = true;
        mut_rect = primary_monitor.get_display();
        let tb_height = rect.bottom - rect.top;
        mut_rect.bottom -= tb_height;
    }
    if !found_primary_display {
        panic!("could not find primary display while calling reset on exit");
    }

    unsafe {
        if call_and_check_set_window_region(
            &mut_rect,
            &[
                SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
                SPIF_SENDWININICHANGE | SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
                SPIF_SENDCHANGE,
                SPIF_UPDATEINIFILE,
            ],
        ) {
            send_workspace_and_display_change_msg();
            return;
        }
        /* no call worked, todo: log error */
        eprint!("failed to reset workspace area");
    }
    /* */
}

#[allow(dead_code)]
pub fn initialize_windows_calls() {
    unsafe {
        /* Initialize system com to retrieve taskbar state in get start menu open function. Safety: None as parameter. */
        if windows::Win32::System::Com::CoInitialize(None).is_err() { /* todo: log error */ }
    }
}

pub fn move_window_on_tb(hwnd: &HWND, x: i32, y: i32) -> bool {
    unsafe {
        move_window(
            hwnd,
            HWND_TOPMOST,
            x,
            y,
            0,
            0,
            SWP_NOSENDCHANGING | SWP_NOSIZE | SWP_NOACTIVATE | SWP_NOZORDER | SWP_ASYNCWINDOWPOS,
        )
    }
}

unsafe fn move_window(
    hwnd: &HWND,
    position: HWND,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    flag: SET_WINDOW_POS_FLAGS,
) -> bool {
    SetWindowPos(*hwnd, position, x, y, width, height, flag).is_ok()
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
        if windows::Win32::UI::WindowsAndMessaging::GetCursorPos(&mut point).is_ok() {
            return Some(point);
        }
        /* todo: log error */
    }
    None
}
