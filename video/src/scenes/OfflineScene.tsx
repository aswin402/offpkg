import React from 'react';
import { interpolate, spring, useCurrentFrame, useVideoConfig } from 'remotion';
import { Terminal, TerminalLine } from '../components/Terminal';

export const OfflineScene: React.FC = () => {
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

  const card1Spring = spring({ frame: frame - 45, fps, config: { damping: 13, stiffness: 120 } });
  const card2Spring = spring({ frame: frame - 85, fps, config: { damping: 13, stiffness: 120 } });
  const card3Spring = spring({ frame: frame - 125, fps, config: { damping: 13, stiffness: 120 } });

  const terminalLines: TerminalLine[] = [
    { type: 'prompt', text: 'offpkg stack list', delayFrame: 15 },
    { type: 'info', text: 'Stacks: react-vite, fastapi-uv, flutter-riverpod, bun-elysia', delayFrame: 35 },
    { type: 'prompt', text: 'offpkg stack add react-vite my-app', delayFrame: 55 },
    { type: 'info', text: "Scaffolding stack 'react-vite' into ./my-app...", delayFrame: 75 },
    { type: 'success', text: 'Unpacked 32 production template files (4.2 ms)', delayFrame: 90 },
    { type: 'success', text: 'Linked cached node_modules from ~/.offpkg/npm/', delayFrame: 105 },
    { type: 'prompt', text: 'offpkg docs show react --offline', delayFrame: 120 },
    { type: 'success', text: 'Loaded offline cheatsheet & API reference (< 1 ms)', delayFrame: 140 },
  ];

  const highlights = [
    {
      icon: '⚡',
      title: 'Instant Scaffolding',
      desc: 'Complete project architectures unpacked in less than 5 milliseconds.',
      springVal: card1Spring,
      color: '#34d399',
    },
    {
      icon: '🔗',
      title: 'Symlink Cache Linking',
      desc: 'Zero duplicate storage waste — shared across all your projects.',
      springVal: card2Spring,
      color: '#00f2fe',
    },
    {
      icon: '📖',
      title: 'Embedded Offline Docs',
      desc: 'Rich terminal doc viewer for quick syntax & API reference mid-flight.',
      springVal: card3Spring,
      color: '#a78bfa',
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
      {/* Category Pill with Airplane / Offline icon */}
      <div
        style={{
          display: 'inline-flex',
          alignItems: 'center',
          gap: 8,
          padding: '6px 18px',
          borderRadius: 20,
          backgroundColor: 'rgba(52, 211, 153, 0.15)',
          border: '1px solid rgba(52, 211, 153, 0.4)',
          marginBottom: 14,
          transform: `scale(${titleScale})`,
        }}
      >
        <div style={{ width: 8, height: 8, borderRadius: 4, backgroundColor: '#34d399' }} />
        <span style={{ fontSize: 13, fontWeight: 700, color: '#34d399', letterSpacing: 1.5 }}>
          STEP 2 • ZERO INTERNET REQUIRED
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
        Scaffold &amp; Build Anywhere Offline
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
        On flights, trains, remote escapes, or air-gapped secure workstations.
      </p>

      {/* Content Layout: Terminal Left, Feature Badges Right */}
      <div style={{ display: 'flex', gap: 36, width: '100%', maxWidth: 1380, alignItems: 'center' }}>
        <div style={{ flex: 1.25 }}>
          <Terminal lines={terminalLines} width="100%" height={430} title="bash — offline project scaffolding" />
        </div>

        <div style={{ flex: 0.75, display: 'flex', flexDirection: 'column', gap: 18 }}>
          {highlights.map((item, idx) => (
            <div
              key={idx}
              style={{
                backgroundColor: 'rgba(15, 23, 42, 0.8)',
                border: '1px solid rgba(255, 255, 255, 0.08)',
                borderLeft: `4px solid ${item.color}`,
                borderRadius: 14,
                padding: '20px 22px',
                display: 'flex',
                gap: 16,
                alignItems: 'flex-start',
                boxShadow: '0 12px 28px rgba(0,0,0,0.4)',
                transform: `translateX(${interpolate(item.springVal, [0, 1], [30, 0])}px) scale(${item.springVal})`,
                opacity: item.springVal,
              }}
            >
              <div style={{ fontSize: 30, lineHeight: 1 }}>{item.icon}</div>
              <div>
                <div style={{ fontSize: 20, fontWeight: 700, color: '#f8fafc', marginBottom: 4 }}>
                  {item.title}
                </div>
                <div style={{ fontSize: 14, color: '#94a3b8', lineHeight: 1.5, fontWeight: 500 }}>
                  {item.desc}
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};
