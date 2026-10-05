import React from 'react';
import { interpolate, useCurrentFrame } from 'remotion';

export const RealTuiScene: React.FC = () => {
  const frame = useCurrentFrame();

  // Timeline events (total 360 frames = 12s at 30fps)
  // 0 - 90: offpkg doctor
  // 90 - 180: offpkg stack list
  // 180 - 270: offpkg docs show --runtime uv fastapi
  // 270 - 360: offpkg list --runtime bun

  let activeCommand = '';
  let content: React.ReactNode = null;

  if (frame < 95) {
    // Stage 1: offpkg doctor
    const cmdProgress = Math.min(1, Math.max(0, (frame - 5) / 18));
    const fullCmd = 'offpkg doctor';
    activeCommand = fullCmd.slice(0, Math.floor(cmdProgress * fullCmd.length));

    const showLogo = frame >= 25;
    const showChecks = frame >= 38;

    content = (
      <div style={{ display: 'flex', flexDirection: 'column', gap: 10 }}>
        {showLogo && (
          <div style={{ color: '#38bdf8', fontWeight: 700, lineHeight: 1.15 }}>
            <pre style={{ margin: 0, fontFamily: 'inherit' }}>
{`╔═╗╔═╗╔═╗╔═╗╦╔═╔═╗
║ ║╠╣ ╠╣ ╠═╝╠╩╗║ ╦
╚═╝╚  ╚  ╩  ╩ ╩╚═╝
  offpkg v0.1.7 · universal offline package manager`}
            </pre>
          </div>
        )}

        {showChecks && (
          <div style={{ display: 'flex', flexDirection: 'column', gap: 6, marginTop: 4 }}>
            <div style={{ color: '#818cf8' }}>
              <span style={{ fontWeight: 700 }}>[ info    ]</span>  offpkg doctor  running environment checks
            </div>
            {frame >= 45 && (
              <div style={{ color: '#34d399' }}>
                <span style={{ fontWeight: 700 }}>[ done    ]</span>  <span style={{ color: '#f8fafc' }}>bun</span>  1.4.2
              </div>
            )}
            {frame >= 52 && (
              <div style={{ color: '#34d399' }}>
                <span style={{ fontWeight: 700 }}>[ done    ]</span>  <span style={{ color: '#f8fafc' }}>uv</span>  uv 0.12.23 (x86_64-unknown-linux-gnu)
              </div>
            )}
            {frame >= 59 && (
              <div style={{ color: '#34d399' }}>
                <span style={{ fontWeight: 700 }}>[ done    ]</span>  <span style={{ color: '#f8fafc' }}>flutter</span>  Flutter 3.47.6 • channel stable
              </div>
            )}
            {frame >= 66 && (
              <div style={{ color: '#34d399' }}>
                <span style={{ fontWeight: 700 }}>[ done    ]</span>  <span style={{ color: '#f8fafc' }}>cache directory</span>  ~/.offpkg/cache — 5 entries
              </div>
            )}
            {frame >= 73 && (
              <div style={{ color: '#34d399' }}>
                <span style={{ fontWeight: 700 }}>[ done    ]</span>  <span style={{ color: '#f8fafc' }}>database</span>  262 package(s) cached — integrity ok
              </div>
            )}
            {frame >= 80 && (
              <div style={{ color: '#38bdf8', fontWeight: 700, marginTop: 4 }}>
                [ done    ]  doctor complete — system healthy &amp; air-gap ready!
              </div>
            )}
          </div>
        )}
      </div>
    );
  } else if (frame < 190) {
    // Stage 2: offpkg stack list
    const f = frame - 95;
    const cmdProgress = Math.min(1, Math.max(0, f / 18));
    const fullCmd = 'offpkg stack list';
    activeCommand = fullCmd.slice(0, Math.floor(cmdProgress * fullCmd.length));

    const showStacks = f >= 22;

    content = (
      <div style={{ display: 'flex', flexDirection: 'column', gap: 10 }}>
        {showStacks && (
          <div style={{ display: 'flex', flexDirection: 'column', gap: 7 }}>
            <div style={{ color: '#818cf8', fontWeight: 700 }}>
              [ info    ]  available stacks
            </div>
            {f >= 28 && (
              <div>
                <span style={{ color: '#34d399', fontWeight: 700 }}>[ done    ]</span>{' '}
                <span style={{ color: '#f8fafc', fontWeight: 700 }}>react-vite</span>{' '}
                <span style={{ color: '#fbbf24' }}>[bun]</span>{' '}
                <span style={{ color: '#38bdf8' }}>12 packages</span>{' '}
                <span style={{ color: '#94a3b8' }}>32 files — React 19 + Vite 8 + Tailwind 4 + Zustand</span>
              </div>
            )}
            {f >= 36 && (
              <div>
                <span style={{ color: '#34d399', fontWeight: 700 }}>[ done    ]</span>{' '}
                <span style={{ color: '#f8fafc', fontWeight: 700 }}>fastapi</span>{' '}
                <span style={{ color: '#fbbf24' }}>[uv]</span>{' '}
                <span style={{ color: '#38bdf8' }}>9 packages</span>{' '}
                <span style={{ color: '#94a3b8' }}>24 files — FastAPI + SQLAlchemy (Async) + Pydantic v2</span>
              </div>
            )}
            {f >= 44 && (
              <div>
                <span style={{ color: '#34d399', fontWeight: 700 }}>[ done    ]</span>{' '}
                <span style={{ color: '#f8fafc', fontWeight: 700 }}>flutter-riverpod</span>{' '}
                <span style={{ color: '#fbbf24' }}>[flutter]</span>{' '}
                <span style={{ color: '#38bdf8' }}>7 packages</span>{' '}
                <span style={{ color: '#94a3b8' }}>30 files — Flutter + Riverpod + GoRouter + Dio</span>
              </div>
            )}
            {f >= 52 && (
              <div>
                <span style={{ color: '#34d399', fontWeight: 700 }}>[ done    ]</span>{' '}
                <span style={{ color: '#f8fafc', fontWeight: 700 }}>next-template</span>{' '}
                <span style={{ color: '#fbbf24' }}>[bun]</span>{' '}
                <span style={{ color: '#38bdf8' }}>44 packages</span>{' '}
                <span style={{ color: '#94a3b8' }}>61 files — Next.js 16 + Tailwind CSS v4 + Prisma 7</span>
              </div>
            )}
            {f >= 60 && (
              <div>
                <span style={{ color: '#34d399', fontWeight: 700 }}>[ done    ]</span>{' '}
                <span style={{ color: '#f8fafc', fontWeight: 700 }}>hono-full</span>{' '}
                <span style={{ color: '#fbbf24' }}>[bun]</span>{' '}
                <span style={{ color: '#38bdf8' }}>13 packages</span>{' '}
                <span style={{ color: '#94a3b8' }}>8 files — Hono + Prisma + Zod + Better Auth</span>
              </div>
            )}
            {f >= 68 && (
              <div style={{ color: '#64748b', fontStyle: 'italic', marginTop: 4 }}>
                💡 Tip: run `offpkg stack add react-vite` to unpack 100% offline
              </div>
            )}
          </div>
        )}
      </div>
    );
  } else if (frame < 280) {
    // Stage 3: offpkg docs show --runtime uv fastapi
    const f = frame - 190;
    const cmdProgress = Math.min(1, Math.max(0, f / 20));
    const fullCmd = 'offpkg docs show --runtime uv fastapi';
    activeCommand = fullCmd.slice(0, Math.floor(cmdProgress * fullCmd.length));

    const showDocs = f >= 26;

    content = (
      <div style={{ display: 'flex', flexDirection: 'column', gap: 10 }}>
        {showDocs && (
          <div style={{ display: 'flex', flexDirection: 'column', gap: 6 }}>
            <div style={{ fontSize: 20, fontWeight: 800, color: '#38bdf8' }}>
              # fastapi
            </div>
            <div style={{ color: '#94a3b8' }}>
              &gt; High-performance Python API framework — async-first, auto-docs, type-safe
            </div>
            <div style={{ color: '#64748b' }}>
              &gt; <span style={{ color: '#f8fafc' }}>Version:</span> 0.135.1 · <span style={{ color: '#fbbf24' }}>Runtime:</span> uv · <span style={{ color: '#34d399' }}>Cached:</span> ~/.offpkg/docs/uv/fastapi.md
            </div>
            <div style={{ height: 1, backgroundColor: 'rgba(255,255,255,0.1)', margin: '4px 0' }} />
            {f >= 34 && (
              <>
                <div style={{ color: '#f8fafc', fontWeight: 700 }}>## Install (Offline Wheel Link)</div>
                <div style={{ backgroundColor: 'rgba(0,0,0,0.4)', padding: '6px 12px', borderRadius: 6, color: '#34d399' }}>
                  uv add "fastapi[standard]"
                </div>
              </>
            )}
            {f >= 46 && (
              <>
                <div style={{ color: '#f8fafc', fontWeight: 700, marginTop: 4 }}>## Run</div>
                <div style={{ backgroundColor: 'rgba(0,0,0,0.4)', padding: '6px 12px', borderRadius: 6, color: '#38bdf8' }}>
                  fastapi dev main.py
                </div>
              </>
            )}
          </div>
        )}
      </div>
    );
  } else {
    // Stage 4: offpkg list --runtime bun
    const f = frame - 280;
    const cmdProgress = Math.min(1, Math.max(0, f / 18));
    const fullCmd = 'offpkg list --runtime bun';
    activeCommand = fullCmd.slice(0, Math.floor(cmdProgress * fullCmd.length));

    const showList = f >= 22;

    content = (
      <div style={{ display: 'flex', flexDirection: 'column', gap: 10 }}>
        {showList && (
          <div style={{ display: 'flex', flexDirection: 'column', gap: 7 }}>
            <div style={{ color: '#818cf8', fontWeight: 700 }}>
              [ info    ]  cached packages (runtime: bun, total: 209)
            </div>
            {f >= 28 && (
              <div>
                <span style={{ color: '#34d399', fontWeight: 700 }}>[ done    ]</span>{' '}
                <span style={{ color: '#f8fafc' }}>react@19.0.0 (bun)</span>{' '}
                <span style={{ color: '#64748b' }}>~/.offpkg/cache/bun/react@19.0.0.tgz</span>
              </div>
            )}
            {f >= 35 && (
              <div>
                <span style={{ color: '#34d399', fontWeight: 700 }}>[ done    ]</span>{' '}
                <span style={{ color: '#f8fafc' }}>vite@6.2.0 (bun)</span>{' '}
                <span style={{ color: '#64748b' }}>~/.offpkg/cache/bun/vite@6.2.0.tgz</span>
              </div>
            )}
            {f >= 42 && (
              <div>
                <span style={{ color: '#34d399', fontWeight: 700 }}>[ done    ]</span>{' '}
                <span style={{ color: '#f8fafc' }}>tailwindcss@4.0.0 (bun)</span>{' '}
                <span style={{ color: '#64748b' }}>~/.offpkg/cache/bun/tailwindcss@4.0.0.tgz</span>
              </div>
            )}
            {f >= 49 && (
              <div>
                <span style={{ color: '#34d399', fontWeight: 700 }}>[ done    ]</span>{' '}
                <span style={{ color: '#f8fafc' }}>zustand@5.0.3 (bun)</span>{' '}
                <span style={{ color: '#64748b' }}>~/.offpkg/cache/bun/zustand@5.0.3.tgz</span>
              </div>
            )}
            {f >= 56 && (
              <div>
                <span style={{ color: '#34d399', fontWeight: 700 }}>[ done    ]</span>{' '}
                <span style={{ color: '#f8fafc' }}>@tanstack/react-query@5.66.0 (bun)</span>{' '}
                <span style={{ color: '#64748b' }}>~/.offpkg/cache/bun/__tanstack__react-query.tgz</span>
              </div>
            )}
            {f >= 64 && (
              <div style={{ color: '#38bdf8', fontWeight: 700, marginTop: 4 }}>
                ✓ All packages verified with SHA-256 integrity in SQLite index
              </div>
            )}
          </div>
        )}
      </div>
    );
  }

  const cursorBlink = Math.sin(frame / 3) > 0;

  return (
    <div
      style={{
        width: '100%',
        height: '100%',
        backgroundColor: '#070a0f',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        padding: 40,
        fontFamily: "'JetBrains Mono', 'Fira Code', 'DejaVu Sans Mono', ui-monospace, monospace",
      }}
    >
      {/* Outer Glow / Shell Container */}
      <div
        style={{
          width: '100%',
          maxWidth: 1120,
          height: 640,
          backgroundColor: 'rgba(13, 17, 23, 0.98)',
          borderRadius: 14,
          border: '1px solid rgba(56, 189, 248, 0.35)',
          boxShadow: '0 25px 60px rgba(0, 0, 0, 0.8), 0 0 35px rgba(0, 242, 254, 0.12)',
          display: 'flex',
          flexDirection: 'column',
          overflow: 'hidden',
        }}
      >
        {/* Window Title Bar */}
        <div
          style={{
            height: 38,
            backgroundColor: '#161b22',
            borderBottom: '1px solid rgba(255, 255, 255, 0.08)',
            display: 'flex',
            alignItems: 'center',
            padding: '0 16px',
            position: 'relative',
          }}
        >
          {/* Traffic Lights */}
          <div style={{ display: 'flex', gap: 8 }}>
            <div style={{ width: 12, height: 12, borderRadius: 6, backgroundColor: '#ff5f56' }} />
            <div style={{ width: 12, height: 12, borderRadius: 6, backgroundColor: '#ffbd2e' }} />
            <div style={{ width: 12, height: 12, borderRadius: 6, backgroundColor: '#27c93f' }} />
          </div>

          {/* Title */}
          <div
            style={{
              position: 'absolute',
              left: 0,
              right: 0,
              textAlign: 'center',
              fontSize: 13,
              fontWeight: 600,
              color: '#94a3b8',
              letterSpacing: 0.5,
            }}
          >
            aswin@workstation: ~/projects/offpkg (bash)
          </div>
        </div>

        {/* Terminal Content */}
        <div
          style={{
            flex: 1,
            padding: '24px 28px',
            fontSize: 15.5,
            lineHeight: 1.5,
            color: '#f8fafc',
            display: 'flex',
            flexDirection: 'column',
            gap: 16,
            overflow: 'hidden',
          }}
        >
          {/* Command Prompt */}
          <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
            <span style={{ color: '#34d399', fontWeight: 700 }}>➜</span>
            <span style={{ color: '#38bdf8', fontWeight: 700 }}>~/projects</span>
            <span style={{ color: '#cbd5e1' }}>$ {activeCommand}</span>
            {cursorBlink && (
              <span
                style={{
                  display: 'inline-block',
                  width: 8,
                  height: 17,
                  backgroundColor: '#38bdf8',
                  marginLeft: 1,
                }}
              />
            )}
          </div>

          {/* Dynamic Body Content */}
          <div style={{ flex: 1 }}>{content}</div>
        </div>
      </div>
    </div>
  );
};
