use super::ANNOUNCEMENT_TIP_URL;
use crate::version::CODEX_CLI_VERSION;
use chrono::NaiveDate;
use chrono::Utc;
use regex_lite::Regex;
use serde::Deserialize;
use std::sync::OnceLock;
use std::thread;
use std::time::Duration;

static ANNOUNCEMENT_TIP: OnceLock<Option<String>> = OnceLock::new();

/// Prewarm the cache of the announcement tip.
pub(crate) fn prewarm() {
    let _ = thread::spawn(|| ANNOUNCEMENT_TIP.get_or_init(init_announcement_tip_in_thread));
}

/// Fetch the announcement tip, return None if the prewarm is not done yet.
pub(crate) fn fetch_announcement_tip() -> Option<String> {
    ANNOUNCEMENT_TIP
        .get()
        .cloned()
        .flatten()
        .and_then(|raw| parse_announcement_tip_toml(&raw))
}

#[derive(Debug, Deserialize)]
struct AnnouncementTipRaw {
    content: String,
    from_date: Option<String>,
    to_date: Option<String>,
    version_regex: Option<String>,
    target_app: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AnnouncementTipDocument {
    announcements: Vec<AnnouncementTipRaw>,
}

#[derive(Debug)]
struct AnnouncementTip {
    content: String,
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    version_regex: Option<Regex>,
    target_app: String,
}

fn init_announcement_tip_in_thread() -> Option<String> {
    thread::spawn(blocking_init_announcement_tip)
        .join()
        .ok()
        .flatten()
}

fn blocking_init_announcement_tip() -> Option<String> {
    // Avoid system proxy detection to prevent macOS system-configuration panics (#8912).
    let client = reqwest::blocking::Client::builder()
        .no_proxy()
        .build()
        .ok()?;
    let response = client
        .get(ANNOUNCEMENT_TIP_URL)
        .timeout(Duration::from_millis(2000))
        .send()
        .ok()?;
    response.error_for_status().ok()?.text().ok()
}

pub(crate) fn parse_announcement_tip_toml(text: &str) -> Option<String> {
    let announcements = toml::from_str::<AnnouncementTipDocument>(text)
        .map(|doc| doc.announcements)
        .or_else(|_| toml::from_str::<Vec<AnnouncementTipRaw>>(text))
        .ok()?;

    let mut latest_match = None;
    let today = Utc::now().date_naive();
    for raw in announcements {
        let Some(tip) = AnnouncementTip::from_raw(raw) else {
            continue;
        };
        if tip.version_matches(CODEX_CLI_VERSION)
            && tip.date_matches(today)
            && tip.target_app == "cli"
        {
            latest_match = Some(tip.content);
        }
    }
    latest_match
}

impl AnnouncementTip {
    fn from_raw(raw: AnnouncementTipRaw) -> Option<Self> {
        let content = raw.content.trim();
        if content.is_empty() {
            return None;
        }

        let from_date = match raw.from_date {
            Some(date) => Some(NaiveDate::parse_from_str(&date, "%Y-%m-%d").ok()?),
            None => None,
        };
        let to_date = match raw.to_date {
            Some(date) => Some(NaiveDate::parse_from_str(&date, "%Y-%m-%d").ok()?),
            None => None,
        };
        let version_regex = match raw.version_regex {
            Some(pattern) => Some(Regex::new(&pattern).ok()?),
            None => None,
        };

        Some(Self {
            content: content.to_string(),
            from_date,
            to_date,
            version_regex,
            target_app: raw.target_app.unwrap_or("cli".to_string()).to_lowercase(),
        })
    }

    fn version_matches(&self, version: &str) -> bool {
        self.version_regex
            .as_ref()
            .is_none_or(|regex| regex.is_match(version))
    }

    fn date_matches(&self, today: NaiveDate) -> bool {
        if let Some(from) = self.from_date
            && today < from
        {
            return false;
        }
        if let Some(to) = self.to_date
            && today >= to
        {
            return false;
        }
        true
    }
}
