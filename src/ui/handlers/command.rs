use crate::cmd::{CmdMode, TrackFilter};
use crate::ui::{Key, UI};

impl UI {
    pub(crate) fn handle_status_bar_input(&mut self, key: Key) {
        match key {
            Key::Char(c) => self.uistate.cmd.push(c),
            Key::Ctrl('c') => self.clear_command(),
            Key::Enter => match self.uistate.cmd_mode {
                CmdMode::Filter => self.set_filter(),
                CmdMode::Cmd => self.process_command(),
                CmdMode::None => unreachable!(),
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

    pub(crate) fn clear_command(&mut self) {
        self.uistate.cmd.clear();
        self.uistate.focused_regions.pop();
        self.uistate.cmd_mode = CmdMode::None;
    }

    pub(crate) fn set_filter(&mut self) {
        self.uistate.filter = TrackFilter::from(&self.uistate.cmd[1..]);
        self.clear_command();
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
