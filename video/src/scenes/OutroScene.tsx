import React from 'react';
import { interpolate, spring, useCurrentFrame, useVideoConfig } from 'remotion';

export const OutroScene: React.FC = () => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();

  const heroSpring = spring({
    frame,
    fps,
    config: { damping: 14, stiffness: 120 },
  });

  const installSpring = spring({
    frame: frame - 25,
    fps,
    config: { damping: 14, stiffness: 120 },
  });

  const platformsSpring = spring({
    frame: frame - 55,
    fps,
    config: { damping: 14, stiffness: 120 },
  });

  return (
    <div
      style={{
        width: '100%',
        height: '100%',
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        padding: '40px 80px',
      }}
    >
      {/* Category Pill */}
      <div
        style={{
          display: 'inline-flex',
          alignItems: 'center',
          gap: 8,
          padding: '6px 18px',
          borderRadius: 20,
          backgroundColor: 'rgba(0, 242, 254, 0.12)',
          border: '1px solid rgba(0, 242, 254, 0.35)',
          marginBottom: 16,
          transform: `scale(${heroSpring})`,
        }}
      >
        <div style={{ width: 8, height: 8, borderRadius: 4, backgroundColor: '#00f2fe' }} />
        <span style={{ fontSize: 13, fontWeight: 700, color: '#00f2fe', letterSpacing: 1.5 }}>
          GET STARTED IN SECONDS
        </span>
      </div>

      {/* Brand Hero */}
      <div
        style={{
          display: 'flex',
          alignItems: 'center',
          gap: 16,
          marginBottom: 10,
          transform: `scale(${heroSpring})`,
        }}
      >
        <span
          style={{
            fontSize: 74,
            fontWeight: 900,
            letterSpacing: '-2px',
            background: 'linear-gradient(135deg, #00f2fe 0%, #38bdf8 100%)',
            WebkitBackgroundClip: 'text',
            WebkitTextFillColor: 'transparent',
            filter: 'drop-shadow(0 0 25px rgba(0, 242, 254, 0.4))',
          }}
        >
          offpkg
        </span>
        <span
          style={{
            fontSize: 74,
            fontWeight: 900,
            color: '#f8fafc',
            letterSpacing: '-2px',
          }}
        >
          v0.1.7
        </span>
      </div>

      <p
        style={{
          fontSize: 24,
          color: '#cbd5e1',
          margin: '0 0 36px 0',
          fontWeight: 600,
          textAlign: 'center',
          opacity: interpolate(frame, [10, 25], [0, 1], { extrapolateRight: 'clamp' }),
        }}
      >
        Code Anywhere. Build Offline. Never Get Blocked Again.
      </p>

      {/* One-Line Install Card */}
      <div
        style={{
          width: '100%',
          maxWidth: 960,
          backgroundColor: 'rgba(15, 23, 42, 0.85)',
          border: '1.5px solid rgba(56, 189, 248, 0.3)',
          borderRadius: 16,
          padding: '24px 32px',
          boxShadow: '0 20px 50px rgba(0,0,0,0.6), 0 0 30px rgba(0, 242, 254, 0.1)',
          display: 'flex',
          flexDirection: 'column',
          gap: 14,
          transform: `translateY(${interpolate(installSpring, [0, 1], [30, 0])}px) scale(${installSpring})`,
          opacity: installSpring,
        }}
      >
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
          <span style={{ fontSize: 13, fontWeight: 700, color: '#38bdf8', letterSpacing: 1 }}>
            ONE-LINE INSTALLATION (LINUX &amp; MACOS)
          </span>
          <span style={{ fontSize: 13, color: '#64748b', fontWeight: 600 }}>Zero runtime dependencies</span>
        </div>

        <div
          style={{
            backgroundColor: '#090d16',
            borderRadius: 10,
            padding: '14px 20px',
            fontFamily: "'JetBrains Mono', 'Fira Code', ui-monospace, monospace",
            fontSize: 15.5,
            color: '#34d399',
            border: '1px solid rgba(255, 255, 255, 0.08)',
            display: 'flex',
            alignItems: 'center',
            gap: 10,
            whiteSpace: 'nowrap',
          }}
        >
          <span style={{ color: '#64748b' }}>$</span>
          <span>curl -fsSL https://raw.githubusercontent.com/aswin402/offpkg/main/install.sh | bash</span>
        </div>

        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginTop: 4 }}>
          <span style={{ fontSize: 13, fontWeight: 700, color: '#a78bfa', letterSpacing: 1 }}>
            WINDOWS POWERSHELL
          </span>
        </div>

        <div
          style={{
            backgroundColor: '#090d16',
            borderRadius: 10,
            padding: '14px 20px',
            fontFamily: "'JetBrains Mono', 'Fira Code', ui-monospace, monospace",
            fontSize: 15.5,
            color: '#a78bfa',
            border: '1px solid rgba(255, 255, 255, 0.08)',
            display: 'flex',
            alignItems: 'center',
            gap: 10,
            whiteSpace: 'nowrap',
          }}
        >
          <span style={{ color: '#64748b' }}>PS&gt;</span>
          <span>irm https://raw.githubusercontent.com/aswin402/offpkg/main/install.ps1 | iex</span>
        </div>
      </div>

      {/* Badges / Platforms Bar */}
      <div
        style={{
          display: 'flex',
          gap: 20,
          marginTop: 34,
          alignItems: 'center',
          transform: `scale(${platformsSpring})`,
          opacity: platformsSpring,
        }}
      >
        <div
          style={{
            padding: '8px 18px',
            borderRadius: 10,
            backgroundColor: 'rgba(255, 255, 255, 0.05)',
            border: '1px solid rgba(255, 255, 255, 0.1)',
            fontSize: 14,
            fontWeight: 600,
            color: '#cbd5e1',
          }}
        >
          ⚡ Pure Rust 2024
        </div>
        <div
          style={{
            padding: '8px 18px',
            borderRadius: 10,
            backgroundColor: 'rgba(255, 255, 255, 0.05)',
            border: '1px solid rgba(255, 255, 255, 0.1)',
            fontSize: 14,
            fontWeight: 600,
            color: '#cbd5e1',
          }}
        >
          📦 Bun • uv • Flutter
        </div>
        <div
          style={{
            padding: '8px 18px',
            borderRadius: 10,
            backgroundColor: 'rgba(255, 255, 255, 0.05)',
            border: '1px solid rgba(255, 255, 255, 0.1)',
            fontSize: 14,
            fontWeight: 600,
            color: '#cbd5e1',
          }}
        >
          ⭐ github.com/aswin402/offpkg
        </div>
      </div>
    </div>
  );
};
