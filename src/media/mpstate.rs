use mp_protocol::JoinedTrack;
use std::collections::VecDeque;

pub(crate) struct MPState {
    queue: VecDeque<JoinedTrack>,
    history: Vec<JoinedTrack>,
}

impl MPState {
    pub fn current_track(&self) -> Option<&JoinedTrack> {
        self.queue.get(0)
    }

    pub fn push_front(&mut self, track: JoinedTrack) {
        self.queue.push_front(track)
    }

    pub fn append(&mut self, track: JoinedTrack) {
        self.queue.push_back(track)
    }

    pub fn play_next(&mut self) {
        let played = self.queue.pop_front().unwrap();
        self.history.push(played);
    }

    pub fn get(&self) -> (&Vec<JoinedTrack>, &VecDeque<JoinedTrack>) {
        (&self.history, &self.queue)
    }
}

impl Default for MPState {
    fn default() -> Self {
        Self {
            queue: VecDeque::new(),
            history: Vec::new(),
        }
    }
}
