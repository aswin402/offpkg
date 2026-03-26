use crate::config::Config;
use anyhow::Result;
use std::io::{self, Write};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;

// ANSI color codes
const CYAN: &str = "\x1b[38;2;0;212;224m";
const GREEN: &str = "\x1b[38;2;0;229;160m";
const AMBER: &str = "\x1b[38;2;245;166;35m";
const CORAL: &str = "\x1b[38;2;255;107;107m";
const PURPLE: &str = "\x1b[38;2;167;139;250m";
const BLUE: &str = "\x1b[38;2;96;165;250m";
const MUTED: &str = "\x1b[38;2;100;116;139m";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";
const CLEAR_LINE: &str = "\x1b[2K\r";

// Braille spinner frames
const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

// ── Label ─────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub enum Label {
    Resolve,
    Cache,
    Link,
    Install,
    Done,
    Warn,
    Error,
    Info,
}

impl Label {
    fn color(&self) -> &'static str {
        match self {
            Label::Resolve => CYAN,
            Label::Cache => PURPLE,
            Label::Link => GREEN,
            Label::Install => GREEN,
            Label::Done => GREEN,
            Label::Warn => AMBER,
            Label::Error => CORAL,
            Label::Info => BLUE,
        }
    }

    fn text(&self) -> &'static str {
        match self {
            Label::Resolve => "resolve",
            Label::Cache => "cache  ",
            Label::Link => "link   ",
            Label::Install => "install",
            Label::Done => "done   ",
            Label::Warn => "warn   ",
            Label::Error => "error  ",
            Label::Info => "info   ",
        }
    }

    fn bold(&self) -> bool {
        matches!(self, Label::Done | Label::Error)
    }
}

// ── Spinner ───────────────────────────────────────────────────────────────────

pub struct Spinner {
    done: Arc<AtomicBool>,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Spinner {
    pub fn start(message: String) -> Self {
        let done = Arc::new(AtomicBool::new(false));
        let done_clone = done.clone();

        let thread = std::thread::spawn(move || {
            let mut frame = 0usize;
            loop {
                if done_clone.load(Ordering::Relaxed) {
                    break;
                }

                let f = SPINNER_FRAMES[frame % SPINNER_FRAMES.len()];
                print!(
                    "{}{}{}{}{} {}{}",
                    CLEAR_LINE, BOLD, CYAN, f, RESET, MUTED, message,
                );
                // trailing reset so color doesn't bleed
                print!("{}", RESET);
                io::stdout().flush().ok();
                std::thread::sleep(Duration::from_millis(80));
                frame += 1;
            }
            print!("{}", CLEAR_LINE);
            io::stdout().flush().ok();
        });

        Self {
            done,
            thread: Some(thread),
        }
    }

    /// Stop spinner and replace with a finished label line.
    pub fn finish(mut self, label: Label, message: &str, secondary: Option<&str>) {
        self.stop_thread();
        print_label_line(label, message, secondary);
    }

    fn stop_thread(&mut self) {
        self.done.store(true, Ordering::Relaxed);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

impl Drop for Spinner {
    fn drop(&mut self) {
        self.done.store(true, Ordering::Relaxed);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
        print!("{}", CLEAR_LINE);
        io::stdout().flush().ok();
    }
}

// ── ProgressBar ───────────────────────────────────────────────────────────────

struct PBarState {
    pct: f32,
    label: String,
}

pub struct ProgressBar {
    done: Arc<AtomicBool>,
    state: Arc<std::sync::Mutex<PBarState>>,
    thread: Option<std::thread::JoinHandle<()>>,
}

// Terminal width for the full-width bar
const TERM_BAR_WIDTH: usize = 55;

impl ProgressBar {
    pub fn start(label: impl Into<String>) -> Self {
        let done = Arc::new(AtomicBool::new(false));
        let state = Arc::new(std::sync::Mutex::new(PBarState {
            pct: 0.0,
            label: label.into(),
        }));

        let done_clone = done.clone();
        let state_clone = state.clone();

        let thread = std::thread::spawn(move || {
            let mut _frame = 0usize;
            // Smooth animation: eased fill position
            let mut display_pct: f32 = 0.0;

            loop {
                if done_clone.load(Ordering::Relaxed) {
                    break;
                }

                let (target_pct, lbl) = {
                    let s = state_clone.lock().unwrap();
                    (s.pct, s.label.clone())
                };

                // Ease toward target
                display_pct += (target_pct - display_pct) * 0.12;
                if (target_pct - display_pct).abs() < 0.002 {
                    display_pct = target_pct;
                }

                let filled = ((display_pct * TERM_BAR_WIDTH as f32) as usize).min(TERM_BAR_WIDTH);
                let empty = TERM_BAR_WIDTH - filled;

                // Glowing head: last filled char is brighter
                let bar = if filled == 0 {
                    format!("{}{}{}", MUTED, "─".repeat(TERM_BAR_WIDTH), RESET)
                } else if filled == TERM_BAR_WIDTH {
                    format!("{}{}{}", CYAN, "─".repeat(TERM_BAR_WIDTH), RESET)
                } else {
                    format!(
                        "{}{}{}{}{}{}{}",
                        CYAN,
                        "─".repeat(filled.saturating_sub(1)),
                        // glowing head
                        "[38;2;180;245;255m",
                        "●",
                        MUTED,
                        "─".repeat(empty),
                        RESET,
                    )
                };

                // Label above bar, bar below — matches the screenshot layout
                print!("{}  {}{}{} {}", CLEAR_LINE, MUTED, lbl, RESET, "");
                // Move to next line and print bar
                print!(
                    "
{}{}{}",
                    CLEAR_LINE, bar, RESET
                );
                // Move cursor back up one line
                print!("[1A");

                io::stdout().flush().ok();
                std::thread::sleep(Duration::from_millis(16)); // ~60fps
                _frame += 1;
            }

            // Print final full bar on its own line
            print!("{}", CLEAR_LINE);
            print!(
                "
{}{}{}
",
                CLEAR_LINE,
                format!("{}{}{}", CYAN, "─".repeat(TERM_BAR_WIDTH), RESET),
                RESET
            );
            io::stdout().flush().ok();
        });

        Self {
            done,
            state,
            thread: Some(thread),
        }
    }

    pub fn set(&self, pct: f32, label: Option<&str>) {
        let mut s = self.state.lock().unwrap();
        s.pct = pct.clamp(0.0, 1.0);
        if let Some(l) = label {
            s.label = l.to_string();
        }
    }

    pub fn finish(mut self, _message: &str) {
        self.stop_thread();
    }

    fn stop_thread(&mut self) {
        self.done.store(true, Ordering::Relaxed);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

impl Drop for ProgressBar {
    fn drop(&mut self) {
        self.done.store(true, Ordering::Relaxed);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
        print!("{}", CLEAR_LINE);
        io::stdout().flush().ok();
    }
}

// ── Shared print helper ───────────────────────────────────────────────────────

fn print_label_line(label: Label, message: &str, secondary: Option<&str>) {
    let bold_on = if label.bold() { BOLD } else { "" };
    let bold_off = if label.bold() { RESET } else { "" };
    let sec = secondary
        .map(|s| format!("  {}{}{}", MUTED, s, RESET))
        .unwrap_or_default();
    println!(
        "{}{}[ {} ]{}  {}{}{}{}",
        bold_on,
        label.color(),
        label.text(),
        RESET,
        bold_on,
        message,
        bold_off,
        sec,
    );
    io::stdout().flush().ok();
}

// ── TUI ───────────────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct TUI {
    pub config: Config,
}

impl TUI {
    pub fn init(config: Config) -> Result<Self> {
        Ok(Self { config })
    }

    /// Print a static labeled line — for steps that complete instantly.
    pub fn print_line(&mut self, label: Label, message: &str, secondary: Option<&str>) {
        print_label_line(label, message, secondary);
    }

    /// Start a braille spinner for a long async wait.
    /// Call `.finish(label, msg, secondary)` when done.
    ///
    /// Example:
    /// ```
    /// let sp = tui.spinner("fetching from npm registry...");
    /// // ... do async work ...
    /// sp.finish(Label::Resolve, "react@18.3.1", Some("npm registry"));
    /// ```
    pub fn spinner(&self, message: &str) -> Spinner {
        Spinner::start(message.to_string())
    }

    /// Start an animated progress bar.
    /// Call `.set(0.0..1.0, label)` to update, `.finish(msg)` when done.
    ///
    /// Example:
    /// ```
    /// let bar = tui.progress_bar("downloading react@18.3.1");
    /// bar.set(0.5, Some("50% downloaded"));
    /// bar.set(1.0, Some("verifying checksum..."));
    /// bar.finish("react@18.3.1 downloaded  (87.4 KB)");
    /// ```
    pub fn progress_bar(&self, label: &str) -> ProgressBar {
        ProgressBar::start(label)
    }

    pub fn print_done_summary(
        &mut self,
        packages: usize,
        downloaded: u64,
        elapsed: std::time::Duration,
    ) {
        let size_str = if downloaded == 0 {
            "0 B".to_string()
        } else if downloaded < 1_000_000 {
            format!("{:.1} KB", downloaded as f64 / 1_000.0)
        } else {
            format!("{:.1} MB", downloaded as f64 / 1_000_000.0)
        };
        let elapsed_str = {
            let ms = elapsed.as_millis();
            if ms < 1000 {
                format!("{}ms", ms)
            } else {
                format!("{:.2}s", elapsed.as_secs_f64())
            }
        };
        let pkg_word = if packages == 1 { "package" } else { "packages" };

        println!();
        println!(
            "  {}{}✓{}  {}{}{} {}{}    {}{}    {}{}",
            BOLD,
            GREEN,
            RESET,
            BOLD,
            packages,
            RESET,
            pkg_word,
            MUTED,
            size_str,
            RESET,
            MUTED,
            elapsed_str,
        );
        println!();
        io::stdout().flush().ok();
    }

    pub fn render_logo(&mut self) {
        println!();
        println!("{}{}╔═╗╔═╗╔═╗╔═╗╦╔═╔═╗{}", BOLD, CYAN, RESET);
        println!("{}{}║ ║╠╣ ╠╣ ╠═╝╠╩╗║ ╦{}", BOLD, CYAN, RESET);
        println!("{}{}╚═╝╚  ╚  ╩  ╩ ╩╚═╝{}", BOLD, CYAN, RESET);
        println!(
            "  {}offpkg v0.1.3 · universal offline package manager{}",
            MUTED, RESET
        );
        println!();
        io::stdout().flush().ok();
    }

    pub fn cleanup(self) -> Result<()> {
        Ok(())
    }
}
