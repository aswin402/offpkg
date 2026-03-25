mod fastapi;
mod flutter;
mod hono;
mod mern;
mod pern;
mod react_vite;

use crate::stacks::Stack;

pub fn builtin_stacks() -> Vec<Stack> {
    vec![
        // ── Bun / React ───────────────────────────────────────────
        react_vite::react_vite(),
        react_vite::react_vite_full(),
        // ── Bun / API ─────────────────────────────────────────────
        hono::hono_api(),
        hono::hono_full(),
        // ── Bun / Fullstack ───────────────────────────────────────
        mern::mern(),
        pern::pern(),
        // ── Python ────────────────────────────────────────────────
        fastapi::fastapi(),
        // ── Flutter ───────────────────────────────────────────────
        flutter::flutter_riverpod(None), // name: "flutter-riverpod"
    ]
}
