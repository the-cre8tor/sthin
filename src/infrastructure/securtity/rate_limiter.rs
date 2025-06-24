use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

#[derive(Debug)]
struct RateLimiterState {
    requests: Vec<Instant>,
    limit: usize,
    window: Duration,
}

pub struct RateLimiter {
    states: Arc<Mutex<HashMap<String, RateLimiterState>>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            states: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn allow_request(&self, client_id: String, limit: usize, window: Duration) -> bool {
        let mut states = self.states.lock().unwrap();

        let state = states.entry(client_id).or_insert(RateLimiterState {
            requests: Vec::new(),
            limit,
            window,
        });

        let now = Instant::now();

        state
            .requests
            .retain(|&req| now.duration_since(req) < state.window);

        if state.requests.len() >= state.limit {
            false
        } else {
            state.requests.push(now);
            true
        }
    }
}
