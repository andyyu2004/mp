use crate::{early_return_bool, early_return_option, network::IOEvent, UI};

pub const SEEK_MILLIS: i64 = 5000;

impl UI {
    pub(crate) fn handle_seek_backward(&mut self) {
        self.dispatch(IOEvent::Seek(-SEEK_MILLIS))
    }

    pub(crate) fn handle_seek_forward(&mut self) {
        self.dispatch(IOEvent::Seek(SEEK_MILLIS))
    }

    pub(crate) fn handle_play_prev(&mut self) {
        self.dispatch(IOEvent::PlayPrev)
    }

    pub(crate) fn handle_set_next(&mut self) {
        let index = early_return_option!(self.uistate.track_table_state.selected());
        let track = &self.client.lock().unwrap().state.tracks[index];
        self.dispatch(IOEvent::SetNextTrack(track.track_id));
    }

    pub(crate) fn handle_shuffle_all(&mut self) {
        self.dispatch(IOEvent::ShuffleAll)
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
        let n = self.uistate.filtered_tracklist_len;
        early_return_bool!(n == 0);
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
        let index = early_return_option!(self.uistate.track_table_state.selected());
        let track = &self.client.lock().unwrap().state.tracks[index];
        self.dispatch(IOEvent::QueueAppend(track.track_id));
    }

    pub(crate) fn handle_play_track(&mut self) {
        let index = early_return_option!(self.uistate.track_table_state.selected());
        let track = &self.client.lock().unwrap().state.tracks[index];
        self.dispatch(IOEvent::PlayTrack(track.track_id));
    }

    pub(crate) fn handle_toggle_play(&mut self) {
        self.dispatch(IOEvent::TogglePlay)
    }
}
