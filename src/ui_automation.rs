use core::fmt;

use uiautomation::Result;
use uiautomation::UIAutomation;
use uiautomation::UIElement;
use uiautomation::UITreeWalker;

use crate::windows_calls::TaskbarData;

pub struct Automation {
    walker: UITreeWalker,
    automation: UIAutomation,
    tb_data: TaskbarData,
    pub current_rect: AutomationRects,
}

impl fmt::Debug for Automation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Automation")
            .field("automation", &self.automation)
            .field("tb_data", &self.tb_data)
            .field("current_rect", &self.current_rect)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct AutomationRects {
    pub tasklist_left: i32,
    pub tasklist_right: i32,
    pub tasklist_up: i32,
    pub tasklist_down: i32,
    pub tray_left: i32,
    pub tray_right: i32,
    pub tray_up: i32,
    pub tray_down: i32,
}

impl Automation {
    pub fn new(tb_data: TaskbarData) -> Self {
        let automation: UIAutomation = UIAutomation::new().expect("Failed to create UIAutomation");
        let walker: UITreeWalker = automation
            .get_control_view_walker()
            .expect("Failed to get walker");
        let current_rect = AutomationRects {
            tasklist_left: 0,
            tasklist_right: 0,
            tasklist_up: 0,
            tasklist_down: 0,
            tray_left: 0,
            tray_right: 0,
            tray_up: 0,
            tray_down: 0,
        };
        Self {
            walker,
            automation,
            tb_data,
            current_rect,
        }
    }

    pub fn update_tb_data(&mut self, tb_data: TaskbarData) {
        self.tb_data = tb_data;
    }

    pub fn update_rects(&mut self) -> Result<()> {
        let mut tasklist: Vec<UIElement> = Vec::new();
        let mut traylist: Vec<UIElement> = Vec::new();
        let element = &self
            .automation
            .element_from_handle(uiautomation::types::Handle::from(
                self.tb_data.taskbar.as_ref().ok_or("No handle found")?.hwnd,
            ))?;
        self.iterate_elements(element, &mut tasklist, &mut traylist, 0)?;
        if tasklist.is_empty() || traylist.is_empty() {
            return Err("No elements found".into());
        }
        self.current_rect = AutomationRects {
            tasklist_left: tasklist[0].get_bounding_rectangle()?.get_left(),
            tasklist_right: tasklist
                .last()
                .ok_or("no elements")?
                .get_bounding_rectangle()?
                .get_right(),
            tasklist_up: tasklist[0].get_bounding_rectangle()?.get_top(),
            tasklist_down: tasklist[0].get_bounding_rectangle()?.get_bottom(),
            tray_left: traylist[0].get_bounding_rectangle()?.get_left(),
            tray_right: traylist
                .last()
                .ok_or("no elements")?
                .get_bounding_rectangle()?
                .get_right(),
            tray_up: traylist[0].get_bounding_rectangle()?.get_top(),
            tray_down: traylist[0].get_bounding_rectangle()?.get_bottom(),
        };
        Ok(())
    }

    fn iterate_elements(
        &mut self,
        element: &UIElement,
        tasklist: &mut Vec<UIElement>,
        traylist: &mut Vec<UIElement>,
        depth: usize,
    ) -> Result<()> {
        if depth == 3 {
            tasklist.push(element.clone());
            return Ok(());
        }
        if depth == 2 {
            if element.get_classname()?.starts_with("SystemTray.") {
                traylist.push(element.clone());
                return Ok(());
            }
        }

        if depth == 2 {
            if let Ok(child) = self.walker.get_first_child(element) {
                self.iterate_elements(&child, tasklist, traylist, depth + 1)?;
            }
            if let Ok(child) = self.walker.get_last_child(element) {
                self.iterate_elements(&child, tasklist, traylist, depth + 1)?;
            }
            return Ok(());
        }

        if let Ok(child) = self.walker.get_first_child(element) {
            self.iterate_elements(&child, tasklist, traylist, depth + 1)?;

            let mut next = child;
            let mut counter = 0;
            while let Ok(sibling) = self.walker.get_next_sibling(&next) {
                if counter == 0 {
                    self.iterate_elements(&sibling, tasklist, traylist, depth + 1)?;
                }
                next = sibling;
                counter += 1;
            }
            if counter > 1 {
                self.iterate_elements(&next, tasklist, traylist, depth + 1)?;
            }
        }
        Ok(())
    }
}
