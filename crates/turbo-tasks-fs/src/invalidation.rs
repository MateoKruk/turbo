use std::fmt::{Display, Formatter};

use indexmap::IndexSet;
use turbo_tasks::{util::StaticOrArc, InvalidationReason, InvalidationReasonKind};

/// Invalidation was caused by a file change detected by the file watcher
#[derive(PartialEq, Eq, Hash)]
pub struct WatchChange {
    pub path: String,
}

impl InvalidationReason for WatchChange {
    fn kind(&self) -> Option<StaticOrArc<dyn InvalidationReasonKind>> {
        Some(StaticOrArc::Static(&WATCH_CHANGE_KIND))
    }
}

impl Display for WatchChange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} changed", self.path)
    }
}

/// Invalidation kind for [WatchChange]
#[derive(PartialEq, Eq, Hash)]
struct WatchChangeKind;

static WATCH_CHANGE_KIND: WatchChangeKind = WatchChangeKind;

impl InvalidationReasonKind for WatchChangeKind {
    fn fmt(
        &self,
        reasons: &IndexSet<StaticOrArc<dyn InvalidationReason>>,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{} files changed ({}, ...)",
            reasons.len(),
            reasons[0]
                .as_any()
                .downcast_ref::<WatchChange>()
                .unwrap()
                .path
        )
    }
}

/// Invalidation was caused by a directory starting to watch from which was read
/// before.
#[derive(PartialEq, Eq, Hash)]
pub struct WatchStart {
    pub name: String,
}

impl InvalidationReason for WatchStart {
    fn kind(&self) -> Option<StaticOrArc<dyn InvalidationReasonKind>> {
        Some(StaticOrArc::Static(&WATCH_START_KIND))
    }
}

impl Display for WatchStart {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} started watching", self.name)
    }
}

/// Invalidation kind for [WatchStart]
#[derive(PartialEq, Eq, Hash)]
struct WatchStartKind;

static WATCH_START_KIND: WatchStartKind = WatchStartKind;

impl InvalidationReasonKind for WatchStartKind {
    fn fmt(
        &self,
        reasons: &IndexSet<StaticOrArc<dyn InvalidationReason>>,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{} directories started watching (e. g. {})",
            reasons.len(),
            reasons[0]
                .as_any()
                .downcast_ref::<WatchStart>()
                .unwrap()
                .name
        )
    }
}
