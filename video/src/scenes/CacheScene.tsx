import React from 'react';
import { interpolate, spring, useCurrentFrame, useVideoConfig } from 'remotion';
import { Terminal, TerminalLine } from '../components/Terminal';

export const CacheScene: React.FC = () => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();

  const titleScale = spring({
    frame,
    fps,
    config: { damping: 14, stiffness: 110 },
  });

  const subtitleOpacity = interpolate(frame, [10, 25], [0, 1], {
    extrapolateRight: 'clamp',
  });

  const badge1Spring = spring({ frame: frame - 45, fps, config: { damping: 13, stiffness: 120 } });
  const badge2Spring = spring({ frame: frame - 85, fps, config: { damping: 13, stiffness: 120 } });
  const badge3Spring = spring({ frame: frame - 125, fps, config: { damping: 13, stiffness: 120 } });

  const terminalLines: TerminalLine[] = [
    { type: 'prompt', text: 'offpkg bun install react@18.3.1', delayFrame: 15 },
    { type: 'info', text: 'Fetching react-18.3.1.tgz from registry.npmjs.org...', delayFrame: 35 },
    { type: 'success', text: 'SHA-256 verified (3a1b...e79f) -> ~/.offpkg/npm/', delayFrame: 50 },
    { type: 'prompt', text: 'offpkg uv install fastapi@0.115.0', delayFrame: 70 },
    { type: 'info', text: 'Resolving PyPI wheels + deps (starlette, pydantic)...', delayFrame: 90 },
    { type: 'success', text: 'Cached 3 wheels to ~/.offpkg/wheels/', delayFrame: 105 },
    { type: 'prompt', text: 'offpkg flutter install dio@5.7.0', delayFrame: 120 },
    { type: 'success', text: 'Indexed in SQLite manifest catalog (34 ms)', delayFrame: 140 },
  ];

  const features = [
    {
      icon: '🛡️',
      title: 'SHA-256 Verified',
      desc: '100% cryptographic integrity checks before writing to disk.',
      springVal: badge1Spring,
      color: '#00f2fe',
    },
    {
      icon: '🗄️',
      title: 'SQLite Catalog',
      desc: 'Atomic metadata index with sub-millisecond query performance.',
      springVal: badge2Spring,
      color: '#38bdf8',
    },
    {
      icon: '⚡',
      title: 'Cross-Ecosystem',
      desc: 'Single cache for JavaScript, Python, and Dart dependencies.',
      springVal: badge3Spring,
      color: '#34d399',
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
        padding: '30px 80px',
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
          backgroundColor: 'rgba(56, 189, 248, 0.15)',
          border: '1px solid rgba(56, 189, 248, 0.4)',
          marginBottom: 14,
          transform: `scale(${titleScale})`,
        }}
      >
        <div style={{ width: 8, height: 8, borderRadius: 4, backgroundColor: '#38bdf8' }} />
        <span style={{ fontSize: 13, fontWeight: 700, color: '#38bdf8', letterSpacing: 1.5 }}>
          STEP 1 • ONLINE INGESTION
        </span>
      </div>

      {/* Main Title */}
      <h1
        style={{
          fontSize: 50,
          fontWeight: 800,
          textAlign: 'center',
          margin: '0 0 10px 0',
          letterSpacing: '-1.5px',
          transform: `scale(${titleScale})`,
          background: 'linear-gradient(135deg, #ffffff 0%, #cbd5e1 100%)',
          WebkitBackgroundClip: 'text',
          WebkitTextFillColor: 'transparent',
        }}
      >
        Cache Once While Connected
      </h1>

      <p
        style={{
          fontSize: 22,
          color: '#94a3b8',
          margin: '0 0 36px 0',
          opacity: subtitleOpacity,
          fontWeight: 500,
          textAlign: 'center',
        }}
      >
        Pre-fetch packages from npm, PyPI, and pub.dev directly into local storage.
      </p>

      {/* Content Layout: Terminal Left, Feature Badges Right */}
      <div style={{ display: 'flex', gap: 36, width: '100%', maxWidth: 1380, alignItems: 'center' }}>
        <div style={{ flex: 1.25 }}>
          <Terminal lines={terminalLines} width="100%" height={430} title="bash — offpkg cache ingestion" />
        </div>

        <div style={{ flex: 0.75, display: 'flex', flexDirection: 'column', gap: 18 }}>
          {features.map((feat, idx) => (
            <div
              key={idx}
              style={{
                backgroundColor: 'rgba(15, 23, 42, 0.8)',
                border: '1px solid rgba(255, 255, 255, 0.08)',
                borderLeft: `4px solid ${feat.color}`,
                borderRadius: 14,
                padding: '20px 22px',
                display: 'flex',
                gap: 16,
                alignItems: 'flex-start',
                boxShadow: '0 12px 28px rgba(0,0,0,0.4)',
                transform: `translateX(${interpolate(feat.springVal, [0, 1], [30, 0])}px) scale(${feat.springVal})`,
                opacity: feat.springVal,
              }}
            >
              <div style={{ fontSize: 30, lineHeight: 1 }}>{feat.icon}</div>
              <div>
                <div style={{ fontSize: 20, fontWeight: 700, color: '#f8fafc', marginBottom: 4 }}>
                  {feat.title}
                </div>
                <div style={{ fontSize: 14, color: '#94a3b8', lineHeight: 1.5, fontWeight: 500 }}>
                  {feat.desc}
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};
