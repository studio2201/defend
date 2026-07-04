//! Process-level counters surfaced via `/metrics`.

use std::sync::atomic::{AtomicU64, Ordering};

/// In-process metric counters.
pub struct Metrics {
    /// Total HTTP requests handled.
    pub requests_total: AtomicU64,
    /// Total requests that returned `429 Too Many Requests`.
    pub requests_429_total: AtomicU64,
    /// Last-seen count of active session tokens.
    pub active_sessions: AtomicU64,
    /// Last-seen count of leaderboard entries.
    pub leaderboard_entries: AtomicU64,
    /// `CARGO_PKG_VERSION` snapshot.
    pub version: String,
}

impl Metrics {
    pub fn new(version: impl Into<String>, active_sessions: u64, leaderboard_entries: u64) -> Self {
        Self {
            requests_total: AtomicU64::new(0),
            requests_429_total: AtomicU64::new(0),
            active_sessions: AtomicU64::new(active_sessions),
            leaderboard_entries: AtomicU64::new(leaderboard_entries),
            version: version.into(),
        }
    }

    /// Increment `defend_requests_total`.
    pub fn inc_requests(&self) {
        self.requests_total.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment `defend_requests_429_total`.
    pub fn inc_rate_limited(&self) {
        self.requests_429_total.fetch_add(1, Ordering::Relaxed);
    }

    /// Overwrite the active-sessions gauge.
    pub fn set_active_sessions(&self, n: u64) {
        self.active_sessions.store(n, Ordering::Relaxed);
    }

    /// Overwrite the leaderboard-entries gauge.
    pub fn set_leaderboard_entries(&self, n: u64) {
        self.leaderboard_entries.store(n, Ordering::Relaxed);
    }
}

/// Render `metrics` as a Prometheus text-format payload.
#[must_use]
pub fn prometheus_text(metrics: &Metrics) -> String {
    format!(
        "defend_requests_total {}\ndefend_requests_429_total {}\ndefend_active_sessions {}\ndefend_leaderboard_entries {}\ndefend_build_info{{version=\"{}\"}} 1\n",
        metrics.requests_total.load(Ordering::Relaxed),
        metrics.requests_429_total.load(Ordering::Relaxed),
        metrics.active_sessions.load(Ordering::Relaxed),
        metrics.leaderboard_entries.load(Ordering::Relaxed),
        metrics.version,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> Metrics {
        Metrics::new("1.2.3", 0, 0)
    }

    #[test]
    fn render_produces_prometheus_text() {
        let m = fixture();
        m.inc_requests();
        m.inc_requests();
        m.inc_rate_limited();
        m.set_active_sessions(7);
        m.set_leaderboard_entries(4);

        let out = prometheus_text(&m);
        let expected = "defend_requests_total 2\n\
                        defend_requests_429_total 1\n\
                        defend_active_sessions 7\n\
                        defend_leaderboard_entries 4\n\
                        defend_build_info{version=\"1.2.3\"} 1\n";
        assert_eq!(out, expected);
    }

    #[test]
    fn inc_rate_limited_visible_after_increment() {
        let m = fixture();
        assert_eq!(m.requests_429_total.load(Ordering::Relaxed), 0);
        m.inc_rate_limited();
        m.inc_rate_limited();
        m.inc_rate_limited();
        assert_eq!(m.requests_429_total.load(Ordering::Relaxed), 3);
        let out = prometheus_text(&m);
        assert!(out.contains("defend_requests_429_total 3\n"));
    }

    #[test]
    fn render_is_zero_by_default() {
        let m = fixture();
        let out = prometheus_text(&m);
        assert!(out.starts_with("defend_requests_total 0\n"));
        assert!(out.contains("defend_requests_429_total 0\n"));
        assert!(out.contains("defend_active_sessions 0\n"));
        assert!(out.contains("defend_leaderboard_entries 0\n"));
        assert!(out.ends_with("defend_build_info{version=\"1.2.3\"} 1\n"));
    }
}
