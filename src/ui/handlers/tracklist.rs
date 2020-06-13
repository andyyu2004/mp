use futures::executor;
use crate::UI;

pub(crate) fn handle_next(ui: &mut UI) {
    handle_list_move(ui, |s, n| match s {
        None => 0,
        Some(i) => (i + 1) % n,
    });

}

fn handle_list_move(ui: &mut UI, f: impl FnOnce(Option<usize>, usize) -> usize) {
    let n = executor::block_on(ui.client.lock()).state.tracks.len();
    if n == 0 { return; }

    let s = &mut ui.uistate.track_list_state;
    let new_index = f(s.selected(), n);
    s.select(Some(new_index));
}

pub(crate) fn handle_prev(ui: &mut UI) {
    handle_list_move(ui, |s, n| match s {
        None => n - 1,
        Some(i) => (i + n - 1) % n,
    });
}
