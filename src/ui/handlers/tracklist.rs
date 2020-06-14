use crate::UI;

impl UI {
    pub(crate) fn handle_next_track(ui: &mut UI) {
        Self::handle_track_list_move(ui, |s, n| match s {
            None => 0,
            Some(i) => (i + 1) % n,
        });
    }

    fn handle_track_list_move(ui: &mut UI, f: impl FnOnce(Option<usize>, usize) -> usize) {
        let n = ui.client.lock().unwrap().state.tracks.len();
        if n == 0 {
            return;
        }
        let s = &mut ui.uistate.track_list_state;
        let new_index = f(s.selected(), n);
        s.select(Some(new_index));
    }

    pub(crate) fn handle_prev_track(ui: &mut UI) {
        Self::handle_track_list_move(ui, |s, n| match s {
            None => n - 1,
            Some(i) => (i + n - 1) % n,
        });
    }

    pub(crate) fn handle_play_track(ui: &mut UI) {
        let index = match ui.uistate.track_list_state.selected() {
            Some(i) => i,
            None => return,
        };

        let track = &ui.client.lock().unwrap().state.tracks[index];
        trace!("play track: {}", track);
    }
}
