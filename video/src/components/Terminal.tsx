import React from 'react';
import { interpolate, spring, useCurrentFrame, useVideoConfig } from 'remotion';

export interface TerminalLine {
  type: 'prompt' | 'success' | 'error' | 'info' | 'output';
  text: string;
  delayFrame: number;
}

interface TerminalProps {
  title?: string;
  lines: TerminalLine[];
  width?: number | string;
  height?: number | string;
}

export const Terminal: React.FC<TerminalProps> = ({
  title = 'bash ~ offpkg',
  lines,
  width = 960,
  height = 480,
}) => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();

  const scale = spring({
    frame,
    fps,
    config: { damping: 14, stiffness: 120 },
  });

  const opacity = interpolate(frame, [0, 8], [0, 1], {
    extrapolateRight: 'clamp',
  });

  return (
    <div
      style={{
        width,
        height,
        backgroundColor: 'rgba(9, 13, 22, 0.95)',
        border: '1.5px solid rgba(56, 189, 248, 0.25)',
        borderRadius: 16,
        boxShadow: '0 25px 60px -15px rgba(0, 0, 0, 0.8), 0 0 40px rgba(0, 242, 254, 0.1)',
        overflow: 'hidden',
        display: 'flex',
        flexDirection: 'column',
        transform: `scale(${scale})`,
        opacity,
        fontFamily: "'JetBrains Mono', 'Fira Code', ui-monospace, monospace",
      }}
    >
      {/* Terminal Title Bar */}
      <div
        style={{
          height: 42,
          backgroundColor: '#0f172a',
          borderBottom: '1px solid rgba(255, 255, 255, 0.08)',
          display: 'flex',
          alignItems: 'center',
          padding: '0 18px',
          position: 'relative',
        }}
      >
        <div style={{ display: 'flex', gap: 8 }}>
          <div style={{ width: 12, height: 12, borderRadius: 6, backgroundColor: '#ff5f56' }} />
          <div style={{ width: 12, height: 12, borderRadius: 6, backgroundColor: '#ffbd2e' }} />
          <div style={{ width: 12, height: 12, borderRadius: 6, backgroundColor: '#27c93f' }} />
        </div>
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
          {title}
        </div>
      </div>

      {/* Terminal Body */}
      <div
        style={{
          padding: 24,
          flex: 1,
          display: 'flex',
          flexDirection: 'column',
          gap: 12,
          overflow: 'hidden',
        }}
      >
        {lines.map((line, idx) => {
          if (frame < line.delayFrame) return null;

          const lineFrame = frame - line.delayFrame;
          const lineOpacity = interpolate(lineFrame, [0, 6], [0, 1], {
            extrapolateRight: 'clamp',
          });
          const translateY = interpolate(lineFrame, [0, 6], [8, 0], {
            extrapolateRight: 'clamp',
          });

          // Typing effect for prompt commands
          let displayText = line.text;
          if (line.type === 'prompt') {
            const charCount = Math.floor(
              interpolate(lineFrame, [0, 20], [0, line.text.length], {
                extrapolateRight: 'clamp',
              })
            );
            displayText = line.text.slice(0, charCount);
          }

          let color = '#f8fafc';
          let prefix = '';

          if (line.type === 'prompt') {
            color = '#38bdf8';
            prefix = '$ ';
          } else if (line.type === 'success') {
            color = '#34d399';
            prefix = '✓ ';
          } else if (line.type === 'error') {
            color = '#f87171';
            prefix = '✗ ';
          } else if (line.type === 'info') {
            color = '#00f2fe';
            prefix = '[ info ] ';
          }

          return (
            <div
              key={idx}
              style={{
                opacity: lineOpacity,
                transform: `translateY(${translateY}px)`,
                fontSize: 17,
                lineHeight: 1.5,
                color,
                display: 'flex',
                alignItems: 'center',
                gap: 6,
              }}
            >
              {prefix && <span style={{ fontWeight: 700, opacity: 0.9 }}>{prefix}</span>}
              <span>{displayText}</span>
              {line.type === 'prompt' && lineFrame < 24 && (
                <span
                  style={{
                    display: 'inline-block',
                    width: 9,
                    height: 18,
                    backgroundColor: '#00f2fe',
                    marginLeft: 2,
                    opacity: Math.sin(frame / 3) > 0 ? 1 : 0,
                  }}
                />
              )}
            </div>
          );
        })}
      </div>
    </div>
  );
};
