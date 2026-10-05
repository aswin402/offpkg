import React from 'react';
import { interpolate, spring, useCurrentFrame, useVideoConfig } from 'remotion';

export const IntroScene: React.FC = () => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();

  const logoScale = spring({
    frame,
    fps,
    config: { damping: 13, stiffness: 110 },
  });

  const card1Spring = spring({ frame: frame - 40, fps, config: { damping: 14, stiffness: 120 } });
  const card2Spring = spring({ frame: frame - 60, fps, config: { damping: 14, stiffness: 120 } });
  const card3Spring = spring({ frame: frame - 80, fps, config: { damping: 14, stiffness: 120 } });

  const cards = [
    {
      icon: '📦',
      title: 'Cache Once Online',
      desc: 'Fetch dependencies from npm, PyPI, or pub.dev once with SHA-256 verification.',
      springVal: card1Spring,
      color: '#00f2fe',
    },
    {
      icon: '⚡',
      title: 'Offline Forever',
      desc: 'Instantly add packages and scaffold full project templates with zero network calls.',
      springVal: card2Spring,
      color: '#818cf8',
    },
    {
      icon: '🦀',
      title: 'Pure Rust Performance',
      desc: '< 15ms CLI dispatch, ~8.2MB static binary, and embedded SQLite manifest catalog.',
      springVal: card3Spring,
      color: '#e05d44',
    },
  ];

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
          marginBottom: 18,
          transform: `scale(${logoScale})`,
        }}
      >
        <div style={{ width: 8, height: 8, borderRadius: 4, backgroundColor: '#00f2fe' }} />
        <span style={{ fontSize: 13, fontWeight: 700, color: '#00f2fe', letterSpacing: 1.5 }}>
          THE ARCHITECTURAL SHIFT
        </span>
      </div>

      {/* Hero Title with ASCII Logo */}
      <div
        style={{
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'center',
          gap: 10,
          marginBottom: 12,
          transform: `scale(${logoScale})`,
        }}
      >
        <pre
          style={{
            margin: 0,
            fontSize: 20,
            fontWeight: 800,
            color: '#38bdf8',
            fontFamily: "'JetBrains Mono', monospace",
            lineHeight: 1.1,
            letterSpacing: 2,
            filter: 'drop-shadow(0 0 15px rgba(56, 189, 248, 0.5))',
          }}
        >
{`╔═╗╔═╗╔═╗╔═╗╦╔═╔═╗
║ ║╠╣ ╠╣ ╠═╝╠╩╗║ ╦
╚═╝╚  ╚  ╩  ╩ ╩╚═╝`}
        </pre>
        <div style={{ display: 'flex', alignItems: 'center', gap: 16 }}>
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
      </div>

      {/* Subtitle */}
      <p
        style={{
          fontSize: 26,
          color: '#cbd5e1',
          margin: '0 0 54px 0',
          fontWeight: 600,
          textAlign: 'center',
          maxWidth: 900,
          lineHeight: 1.4,
          opacity: interpolate(frame, [10, 25], [0, 1], { extrapolateRight: 'clamp' }),
        }}
      >
        Universal Offline Package Manager &amp; Template Engine for modern developers.
      </p>

      {/* 3 Pillar Cards */}
      <div style={{ display: 'flex', gap: 28, width: '100%', maxWidth: 1280 }}>
        {cards.map((card, i) => (
          <div
            key={i}
            style={{
              flex: 1,
              backgroundColor: 'rgba(15, 23, 42, 0.75)',
              border: `1.5px solid rgba(255, 255, 255, 0.08)`,
              borderTop: `3px solid ${card.color}`,
              borderRadius: 18,
              padding: '32px 28px',
              display: 'flex',
              flexDirection: 'column',
              gap: 14,
              boxShadow: '0 20px 40px rgba(0,0,0,0.5)',
              transform: `translateY(${interpolate(card.springVal, [0, 1], [40, 0])}px) scale(${card.springVal})`,
              opacity: card.springVal,
            }}
          >
            <div style={{ fontSize: 38 }}>{card.icon}</div>
            <div style={{ fontSize: 24, fontWeight: 800, color: '#f8fafc', letterSpacing: '-0.5px' }}>
              {card.title}
            </div>
            <div style={{ fontSize: 16, color: '#94a3b8', lineHeight: 1.6, fontWeight: 500 }}>
              {card.desc}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
