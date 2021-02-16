use mp_protocol::JoinedTrack;
use rand::seq::SliceRandom;
use std::collections::VecDeque;

pub(crate) struct MPState {
    queue: VecDeque<JoinedTrack>,
    history: Vec<JoinedTrack>,
}

impl MPState {
    pub fn curr_track(&self) -> Option<&JoinedTrack> {
        self.queue.get(0)
    }

    pub fn push_front(&mut self, track: JoinedTrack) {
        self.queue.push_front(track)
    }

    pub fn append(&mut self, track: JoinedTrack) {
        self.queue.push_back(track)
    }

    pub fn set_next_track(&mut self, track: JoinedTrack) {
        self.queue.insert(1, track);
    }

    /// shuffles the given vector and returns a reference of the first track to be played
    pub fn shuffle_all(&mut self, mut tracks: Vec<JoinedTrack>) -> Option<&JoinedTrack> {
        let mut rng = rand::thread_rng();
        tracks.shuffle(&mut rng);
        self.queue = VecDeque::from(tracks);
        self.peek_queue()
    }

    pub fn peek_queue(&self) -> Option<&JoinedTrack> {
        self.queue.get(0)
    }

    pub fn play_prev(&mut self) -> Option<&JoinedTrack> {
        let last_played = self.history.pop()?;
        self.queue.push_front(last_played);
        self.peek_queue()
    }

    /// mutates the queue and history and returns the new track
    pub fn play_next(&mut self) -> Option<&JoinedTrack> {
        if self.queue.len() <= 1 {
            return None;
        }
        let played = self.queue.pop_front()?;
        self.history.push(played);
        self.peek_queue()
    }

    pub fn get_queue(&self) -> (&Vec<JoinedTrack>, &VecDeque<JoinedTrack>) {
        (&self.history, &self.queue)
    }
}

impl Default for MPState {
    fn default() -> Self {
        Self { queue: Default::default(), history: Default::default() }
    }
}
