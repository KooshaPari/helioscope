use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

static DEFAULT_PALETTE_VERSION: AtomicU64 = AtomicU64::new(0);

fn bump_palette_version() {
    DEFAULT_PALETTE_VERSION.fetch_add(1, Ordering::Relaxed);
}

pub fn requery_default_colors() {
    imp::requery_default_colors();
    bump_palette_version();
}

#[derive(Clone, Copy)]
pub struct DefaultColors {
    fg: (u8, u8, u8),
    bg: (u8, u8, u8),
}

pub fn default_colors() -> Option<DefaultColors> {
    imp::default_colors()
}

pub fn default_fg() -> Option<(u8, u8, u8)> {
    default_colors().map(|c| c.fg)
}

pub fn default_bg() -> Option<(u8, u8, u8)> {
    default_colors().map(|c| c.bg)
}

#[cfg(all(unix, not(test)))]
mod imp {
    use super::DefaultColors;

    pub(super) fn default_colors() -> Option<DefaultColors> {
        None
    }

    pub(super) fn requery_default_colors() {}
}

#[cfg(not(all(unix, not(test))))]
mod imp {
    use super::DefaultColors;

    pub(super) fn default_colors() -> Option<DefaultColors> {
        None
    }

    pub(super) fn requery_default_colors() {}
}
