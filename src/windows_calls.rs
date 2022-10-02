use windows::Win32::Foundation::POINT;
use windows::Win32::{Foundation, UI::WindowsAndMessaging::*};
use Foundation::HWND;
use Foundation::RECT;

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
        println!("did not find {name}!");
        dbg!(hwnd);
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

pub fn get_cursor_pos() -> Option<POINT> {
    let mut point = POINT::default();
    /* Safety: returning none if the cursor pos can not be retrieved. */
    unsafe {
        if windows::Win32::UI::WindowsAndMessaging::GetCursorPos(&mut point).as_bool() {
            return Some(point);
        }
    }
    None
}
