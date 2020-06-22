use crate::cmd::{CmdMode, TrackFilter};
use crate::{
    early_return_bool, ui::{Key, UI}
};

impl UI {
    pub(crate) fn handle_status_bar_input(&mut self, key: Key) {
        match key {
            Key::Char(c) => {
                self.uistate.cmd.push(c);
                if self.uistate.cmd_mode == CmdMode::Filter {
                    self.set_filter()
                }
            }
            Key::Ctrl('c') => self.clear_command(),
            Key::Enter => match self.uistate.cmd_mode {
                CmdMode::Cmd => self.process_command(),
                CmdMode::None => unreachable!(),
                CmdMode::Filter => self.return_focus(),
            },
            Key::Backspace => {
                // don't let the start character (:|/) get popped
                if self.uistate.cmd.len() > 1 {
                    self.uistate.cmd.pop();
                }
            }
            _ => {}
        }
    }

    fn return_focus(&mut self) {
        self.uistate.focused_regions.pop();
        self.uistate.cmd_mode = CmdMode::None;
    }

    pub(crate) fn clear_command(&mut self) {
        self.uistate.cmd.clear();
        self.uistate.filter = TrackFilter::default();
        self.return_focus();
    }

    pub(crate) fn set_filter(&mut self) {
        let filter_str = &self.uistate.cmd;
        early_return_bool!(filter_str.len() < 3);
        self.uistate.filter = TrackFilter::from(&filter_str[1..]);
    }

    pub(crate) fn process_command(&mut self) {
        let cmd = std::mem::take(&mut self.uistate.cmd);
        self.clear_command();
        match cmd.as_str() {
            ":q" => self.uistate.should_quit = true,
            _ => {}
        }
    }
}
