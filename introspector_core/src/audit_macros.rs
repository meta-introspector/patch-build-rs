use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Debug, Clone, Default)]
#[decl(struct, name = "AuditStats", vis = "pub", hash = "32560e10")]
pub struct AuditStats {
    pub fixme_count: usize,
    pub phony_count: usize,
    pub fakedata_count: usize,
    pub issue_count: usize,
    pub todo_count: usize,
    pub concern_count: usize,
    pub unverified_count: usize,
    pub claims: Vec<AuditEntry>,
}

#[derive(Debug, Clone)]
#[decl(struct, name = "AuditEntry", vis = "pub", hash = "ed6ffa75")]
pub struct AuditEntry {
    pub kind: AuditKind,
    pub message: String,
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[decl(enum, name = "AuditKind", vis = "pub", hash = "32e60230")]
pub enum AuditKind {
    Fixme,
    Phony,
    FakeData,
    Issue,
    Todo,
    Concern,
    Unverified,
}

impl AuditKind {
    pub fn emoji(&self) -> &'static str {
        match self {
            AuditKind::Fixme => "🔧",
            AuditKind::Phony => "🎭",
            AuditKind::FakeData => "🧪",
            AuditKind::Issue => "⚠️",
            AuditKind::Todo => "📝",
            AuditKind::Concern => "🤔",
            AuditKind::Unverified => "❓",
        }
    }

    pub fn color_code(&self) -> &'static str {
        match self {
            AuditKind::Fixme => "\x1b[93m",      // Yellow
            AuditKind::Phony => "\x1b[91m",      // Red
            AuditKind::FakeData => "\x1b[95m",   // Magenta
            AuditKind::Issue => "\x1b[91m",      // Red
            AuditKind::Todo => "\x1b[96m",       // Cyan
            AuditKind::Concern => "\x1b[33m",    // Dark yellow
            AuditKind::Unverified => "\x1b[90m", // Gray
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            AuditKind::Fixme => "FIXME",
            AuditKind::Phony => "PHONY",
            AuditKind::FakeData => "FAKEDATA",
            AuditKind::Issue => "ISSUE",
            AuditKind::Todo => "TODO",
            AuditKind::Concern => "CONCERN",
            AuditKind::Unverified => "UNVERIFIED",
        }
    }
}

pub static AUDIT_REGISTRY: Lazy<Mutex<AuditStats>> = Lazy::new(|| Mutex::new(AuditStats::default()));

#[decl(fn, name = "register_audit", vis = "pub", hash = "11183f59")]
pub fn register_audit(kind: AuditKind, message: &str, file: &'static str, line: u32, column: u32) {
    if let Ok(mut stats) = AUDIT_REGISTRY.lock() {
        match kind {
            AuditKind::Fixme => stats.fixme_count += 1,
            AuditKind::Phony => stats.phony_count += 1,
            AuditKind::FakeData => stats.fakedata_count += 1,
            AuditKind::Issue => stats.issue_count += 1,
            AuditKind::Todo => stats.todo_count += 1,
            AuditKind::Concern => stats.concern_count += 1,
            AuditKind::Unverified => stats.unverified_count += 1,
        }
        stats.claims.push(AuditEntry {
            kind,
            message: message.to_string(),
            file,
            line,
            column,
        });
    }
}

#[decl(fn, name = "print_audit_warning", vis = "pub", hash = "53749765")]
pub fn print_audit_warning(kind: AuditKind, message: &str, file: &str, line: u32) {
    let reset = "\x1b[0m";
    eprintln!(
        "{}{} {} [{}:{}]: {}{}",
        kind.color_code(),
        kind.emoji(),
        kind.label(),
        file,
        line,
        message,
        reset
    );
}

#[decl(fn, name = "print_audit_summary", vis = "pub", hash = "f95f6f67")]
pub fn print_audit_summary() {
    if let Ok(stats) = AUDIT_REGISTRY.lock() {
        let reset = "\x1b[0m";
        eprintln!("\n{}", "═".repeat(60));
        eprintln!("📊 AUDIT SUMMARY");
        eprintln!("{}", "═".repeat(60));
        eprintln!(
            "{}🔧 FIXME:      {:>5}{}",
            AuditKind::Fixme.color_code(),
            stats.fixme_count,
            reset
        );
        eprintln!(
            "{}🎭 PHONY:      {:>5}{}",
            AuditKind::Phony.color_code(),
            stats.phony_count,
            reset
        );
        eprintln!(
            "{}🧪 FAKEDATA:   {:>5}{}",
            AuditKind::FakeData.color_code(),
            stats.fakedata_count,
            reset
        );
        eprintln!(
            "{}⚠️  ISSUE:      {:>5}{}",
            AuditKind::Issue.color_code(),
            stats.issue_count,
            reset
        );
        eprintln!(
            "{}📝 TODO:       {:>5}{}",
            AuditKind::Todo.color_code(),
            stats.todo_count,
            reset
        );
        eprintln!(
            "{}🤔 CONCERN:    {:>5}{}",
            AuditKind::Concern.color_code(),
            stats.concern_count,
            reset
        );
        eprintln!(
            "{}❓ UNVERIFIED: {:>5}{}",
            AuditKind::Unverified.color_code(),
            stats.unverified_count,
            reset
        );
        eprintln!("{}", "─".repeat(60));
        eprintln!(
            "   TOTAL:      {:>5}",
            stats.fixme_count
                + stats.phony_count
                + stats.fakedata_count
                + stats.issue_count
                + stats.todo_count
                + stats.concern_count
                + stats.unverified_count
        );
        eprintln!("{}\n", "═".repeat(60));
    }
}

#[decl(fn, name = "get_audit_stats", vis = "pub", hash = "599003ee")]
pub fn get_audit_stats() -> Option<AuditStats> {
    AUDIT_REGISTRY.lock().ok().map(|s| s.clone())
}

#[decl(fn, name = "get_audits_by_kind", vis = "pub", hash = "2fbcefeb")]
pub fn get_audits_by_kind(kind: AuditKind) -> Vec<AuditEntry> {
    AUDIT_REGISTRY
        .lock()
        .ok()
        .map(|s| s.claims.iter().filter(|e| e.kind == kind).cloned().collect())
        .unwrap_or_default()
}

#[decl(fn, name = "clear_audits", vis = "pub", hash = "28fb0532")]
pub fn clear_audits() {
    if let Ok(mut stats) = AUDIT_REGISTRY.lock() {
        *stats = AuditStats::default();
    }
}

#[macro_export]
macro_rules! fixme {
    ($($msg:tt)*) => {{
        $crate::audit_macros::register_audit(
            $crate::audit_macros::AuditKind::Fixme,
            &format!($($msg)*),
            file!(),
            line!(),
            column!(),
        );
        $crate::audit_macros::print_audit_warning(
            $crate::audit_macros::AuditKind::Fixme,
            &format!($($msg)*),
            file!(),
            line!(),
        );
    }};
}

#[macro_export]
macro_rules! phony {
    ($($msg:tt)*) => {{
        $crate::audit_macros::register_audit(
            $crate::audit_macros::AuditKind::Phony,
            &format!($($msg)*),
            file!(),
            line!(),
            column!(),
        );
        $crate::audit_macros::print_audit_warning(
            $crate::audit_macros::AuditKind::Phony,
            &format!($($msg)*),
            file!(),
            line!(),
        );
    }};
}

#[macro_export]
macro_rules! fakedata {
    ($($msg:tt)*) => {{
        $crate::audit_macros::register_audit(
            $crate::audit_macros::AuditKind::FakeData,
            &format!($($msg)*),
            file!(),
            line!(),
            column!(),
        );
        $crate::audit_macros::print_audit_warning(
            $crate::audit_macros::AuditKind::FakeData,
            &format!($($msg)*),
            file!(),
            line!(),
        );
    }};
}

#[macro_export]
macro_rules! issue {
    ($($msg:tt)*) => {{
        $crate::audit_macros::register_audit(
            $crate::audit_macros::AuditKind::Issue,
            &format!($($msg)*),
            file!(),
            line!(),
            column!(),
        );
        $crate::audit_macros::print_audit_warning(
            $crate::audit_macros::AuditKind::Issue,
            &format!($($msg)*),
            file!(),
            line!(),
        );
    }};
}

#[macro_export]
macro_rules! audit_todo {
    ($($msg:tt)*) => {{
        $crate::audit_macros::register_audit(
            $crate::audit_macros::AuditKind::Todo,
            &format!($($msg)*),
            file!(),
            line!(),
            column!(),
        );
        $crate::audit_macros::print_audit_warning(
            $crate::audit_macros::AuditKind::Todo,
            &format!($($msg)*),
            file!(),
            line!(),
        );
    }};
}

#[macro_export]
macro_rules! concern {
    ($($msg:tt)*) => {{
        $crate::audit_macros::register_audit(
            $crate::audit_macros::AuditKind::Concern,
            &format!($($msg)*),
            file!(),
            line!(),
            column!(),
        );
        $crate::audit_macros::print_audit_warning(
            $crate::audit_macros::AuditKind::Concern,
            &format!($($msg)*),
            file!(),
            line!(),
        );
    }};
}

#[macro_export]
macro_rules! unverified {
    ($($msg:tt)*) => {{
        $crate::audit_macros::register_audit(
            $crate::audit_macros::AuditKind::Unverified,
            &format!($($msg)*),
            file!(),
            line!(),
            column!(),
        );
        $crate::audit_macros::print_audit_warning(
            $crate::audit_macros::AuditKind::Unverified,
            &format!($($msg)*),
            file!(),
            line!(),
        );
    }};
}

#[macro_export]
macro_rules! audit_summary {
    () => {
        $crate::audit_macros::print_audit_summary();
    };
}

#[macro_export]
macro_rules! audit_stats {
    () => {
        $crate::audit_macros::get_audit_stats()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_registry() {
        clear_audits();
        register_audit(AuditKind::Fixme, "test fixme", file!(), line!(), column!());
        register_audit(AuditKind::Phony, "test phony", file!(), line!(), column!());

        let stats = get_audit_stats().unwrap();
        assert_eq!(stats.fixme_count, 1);
        assert_eq!(stats.phony_count, 1);
        assert_eq!(stats.claims.len(), 2);
    }

    #[test]
    fn test_emoji_output() {
        assert_eq!(AuditKind::Fixme.emoji(), "🔧");
        assert_eq!(AuditKind::Phony.emoji(), "🎭");
        assert_eq!(AuditKind::FakeData.emoji(), "🧪");
    }
}