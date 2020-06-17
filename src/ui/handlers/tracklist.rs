use crate::{network::IOEvent, UI};

macro_rules! early_return {
    ($option:expr) => {
        match $option {
            Some(x) => x,
            None => return,
        }
    };
}

impl UI {
    pub(crate) fn handle_play_prev(&mut self) {
        self.dispatch(IOEvent::PlayPrev)
    }

    pub(crate) fn handle_play_next(&mut self) {
        self.dispatch(IOEvent::PlayNext)
    }

    pub(crate) fn handle_select_next(&mut self) {
        self.handle_track_list_move(|s, n| match s {
            None => 0,
            Some(i) => (i + 1) % n,
        });
    }

    fn handle_track_list_move(&mut self, f: impl FnOnce(Option<usize>, usize) -> usize) {
        let n = self.client.lock().unwrap().state.tracks.len();
        if n == 0 {
            return;
        }
        let s = &mut self.uistate.track_table_state;
        let new_index = f(s.selected(), n);
        s.select(Some(new_index));
    }

    pub(crate) fn handle_select_prev(ui: &mut UI) {
        Self::handle_track_list_move(ui, |s, n| match s {
            None => n - 1,
            Some(i) => (i + n - 1) % n,
        });
    }

    pub(crate) fn handle_queue_append(&mut self) {
        let index = early_return!(self.uistate.track_table_state.selected());
        let track = &self.client.lock().unwrap().state.tracks[index];
        self.dispatch(IOEvent::QueueAppend(track.track_id));
    }

    pub(crate) fn handle_play_track(&mut self) {
        let index = early_return!(self.uistate.track_table_state.selected());
        let track = &self.client.lock().unwrap().state.tracks[index];
        self.dispatch(IOEvent::PlayTrack(track.track_id));
    }

    pub(crate) fn handle_toggle_play(&mut self) {
        self.dispatch(IOEvent::TogglePlay)
    }
}
