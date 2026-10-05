import React from 'react';
import { interpolate, useCurrentFrame } from 'remotion';

export const Background: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const frame = useCurrentFrame();

  const glow1X = 250 + Math.sin(frame / 60) * 80;
  const glow1Y = 200 + Math.cos(frame / 70) * 60;
  const glow2X = 1600 + Math.cos(frame / 65) * 80;
  const glow2Y = 850 + Math.sin(frame / 75) * 60;

  const gridOpacity = interpolate(frame, [0, 30], [0, 0.15], {
    extrapolateRight: 'clamp',
  });

  return (
    <div
      style={{
        width: 1920,
        height: 1080,
        backgroundColor: '#030712',
        position: 'relative',
        overflow: 'hidden',
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        color: '#f8fafc',
        fontFamily: "'Inter', system-ui, -apple-system, sans-serif",
        WebkitFontSmoothing: 'antialiased',
        MozOsxFontSmoothing: 'grayscale',
        textRendering: 'optimizeLegibility',
      }}
    >
      <style>{`
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800;900&family=JetBrains+Mono:wght@400;500;600;700&display=swap');
      `}</style>
      {/* Ambient Radial Neon Lights */}
      <div
        style={{
          position: 'absolute',
          left: glow1X - 350,
          top: glow1Y - 350,
          width: 700,
          height: 700,
          borderRadius: 350,
          background: 'radial-gradient(circle, rgba(0, 242, 254, 0.15) 0%, rgba(0, 242, 254, 0) 70%)',
          pointerEvents: 'none',
        }}
      />
      <div
        style={{
          position: 'absolute',
          left: glow2X - 400,
          top: glow2Y - 400,
          width: 800,
          height: 800,
          borderRadius: 400,
          background: 'radial-gradient(circle, rgba(99, 102, 241, 0.18) 0%, rgba(99, 102, 241, 0) 70%)',
          pointerEvents: 'none',
        }}
      />

      {/* Grid Overlay */}
      <div
        style={{
          position: 'absolute',
          inset: 0,
          backgroundImage:
            'linear-gradient(rgba(255, 255, 255, 0.05) 1px, transparent 1px), linear-gradient(90deg, rgba(255, 255, 255, 0.05) 1px, transparent 1px)',
          backgroundSize: '48px 48px',
          opacity: gridOpacity,
          pointerEvents: 'none',
        }}
      />

      {/* Content */}
      <div style={{ position: 'relative', zIndex: 10, width: '100%', height: '100%' }}>
        {children}
      </div>
    </div>
  );
};
