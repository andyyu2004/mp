use mp_protocol::JoinedTrack;
use rand::seq::SliceRandom;
use std::collections::VecDeque;

pub(crate) struct MPState {
    queue: VecDeque<JoinedTrack>,
    history: Vec<JoinedTrack>,
    /// progress of the current track
    progress: i64,
    is_playing: bool,
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

    /// shuffles the given vector and returns a reference of the first track to be played
    pub fn shuffle_all(&mut self, mut tracks: Vec<JoinedTrack>) -> Option<&JoinedTrack> {
        let mut rng = rand::thread_rng();
        tracks.shuffle(&mut rng);
        self.queue = VecDeque::from(tracks);
        self.peek_q()
    }

    pub fn peek_q(&self) -> Option<&JoinedTrack> {
        self.queue.get(0)
    }

    pub fn play_prev(&mut self) -> Option<&JoinedTrack> {
        let last_played = self.history.pop()?;
        self.queue.push_front(last_played);
        self.peek_q()
    }

    /// mutates the queue and history and returns the new track
    pub fn play_next(&mut self) -> Option<&JoinedTrack> {
        if self.queue.len() <= 1 {
            return None;
        }
        let played = self.queue.pop_front()?;
        self.history.push(played);
        self.peek_q()
    }

    pub fn getq(&self) -> (&Vec<JoinedTrack>, &VecDeque<JoinedTrack>) {
        (&self.history, &self.queue)
    }
}

impl Default for MPState {
    fn default() -> Self {
        Self {
            queue: VecDeque::default(),
            history: Vec::default(),
            progress: i64::default(),
            is_playing: bool::default(),
        }
    }
}
