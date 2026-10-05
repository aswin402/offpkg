import React from 'react';
import { interpolate, spring, useCurrentFrame, useVideoConfig } from 'remotion';
import { Terminal, TerminalLine } from '../components/Terminal';

export const ProblemScene: React.FC = () => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();

  const titleScale = spring({
    frame,
    fps,
    config: { damping: 15, stiffness: 100 },
  });

  const subtitleOpacity = interpolate(frame, [15, 30], [0, 1], {
    extrapolateRight: 'clamp',
  });

  const alertScale = spring({
    frame: frame - 100,
    fps,
    config: { damping: 12, stiffness: 140 },
  });

  const terminalLines: TerminalLine[] = [
    { type: 'prompt', text: 'bun add react', delayFrame: 25 },
    { type: 'error', text: 'getaddrinfo ENOTFOUND registry.npmjs.org', delayFrame: 45 },
    { type: 'prompt', text: 'uv add fastapi', delayFrame: 60 },
    { type: 'error', text: 'Could not connect to pypi.org (Network unreachable)', delayFrame: 80 },
    { type: 'prompt', text: 'flutter pub get', delayFrame: 95 },
    { type: 'error', text: 'SocketException: Failed host lookup: pub.dev', delayFrame: 110 },
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
        padding: 60,
      }}
    >
      {/* Category Pill */}
      <div
        style={{
          display: 'inline-flex',
          alignItems: 'center',
          gap: 8,
          padding: '6px 16px',
          borderRadius: 20,
          backgroundColor: 'rgba(244, 63, 94, 0.15)',
          border: '1px solid rgba(244, 63, 94, 0.4)',
          marginBottom: 16,
          transform: `scale(${titleScale})`,
        }}
      >
        <div style={{ width: 8, height: 8, borderRadius: 4, backgroundColor: '#f43f5e' }} />
        <span style={{ fontSize: 13, fontWeight: 700, color: '#f43f5e', letterSpacing: 1.5 }}>
          THE DEVELOPER DILEMMA
        </span>
      </div>

      {/* Main Title */}
      <h1
        style={{
          fontSize: 54,
          fontWeight: 800,
          textAlign: 'center',
          margin: '0 0 12px 0',
          letterSpacing: '-1.5px',
          transform: `scale(${titleScale})`,
          background: 'linear-gradient(135deg, #ffffff 0%, #cbd5e1 100%)',
          WebkitBackgroundClip: 'text',
          WebkitTextFillColor: 'transparent',
        }}
      >
        Ever tried coding completely offline?
      </h1>

      <p
        style={{
          fontSize: 22,
          color: '#94a3b8',
          margin: '0 0 40px 0',
          opacity: subtitleOpacity,
          fontWeight: 500,
        }}
      >
        Flights, remote trains, transit blackouts, or air-gapped networks...
      </p>

      {/* Mock Terminal showing failures */}
      <div style={{ position: 'relative' }}>
        <Terminal lines={terminalLines} width={1000} height={380} title="bash — offline error" />

        {/* Warning Callout Pop-in */}
        {frame >= 100 && (
          <div
            style={{
              position: 'absolute',
              bottom: -24,
              left: '50%',
              transform: `translateX(-50%) scale(${alertScale})`,
              backgroundColor: '#ef4444',
              color: '#ffffff',
              padding: '12px 28px',
              borderRadius: 30,
              fontSize: 18,
              fontWeight: 800,
              boxShadow: '0 10px 30px rgba(239, 68, 68, 0.4)',
              display: 'flex',
              alignItems: 'center',
              gap: 10,
              whiteSpace: 'nowrap',
            }}
          >
            <span>⚠️</span>
            <span>Standard package managers fail without internet.</span>
          </div>
        )}
      </div>
    </div>
  );
};
