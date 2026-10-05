import React from 'react';
import { Series } from 'remotion';
import { Background } from './components/Background';
import { ProblemScene } from './scenes/ProblemScene';
import { IntroScene } from './scenes/IntroScene';
import { CacheScene } from './scenes/CacheScene';
import { OfflineScene } from './scenes/OfflineScene';
import { OutroScene } from './scenes/OutroScene';

export const HowItWorks: React.FC = () => {
  return (
    <Background>
      <Series>
        {/* Scene 1: The Problem (0s - 5s = 150 frames) */}
        <Series.Sequence durationInFrames={150}>
          <ProblemScene />
        </Series.Sequence>

        {/* Scene 2: Introducing offpkg (5s - 11s = 180 frames) */}
        <Series.Sequence durationInFrames={180}>
          <IntroScene />
        </Series.Sequence>

        {/* Scene 3: Online Cache Ingestion (11s - 18s = 210 frames) */}
        <Series.Sequence durationInFrames={210}>
          <CacheScene />
        </Series.Sequence>

        {/* Scene 4: Offline Scaffolding & Docs (18s - 25s = 210 frames) */}
        <Series.Sequence durationInFrames={210}>
          <OfflineScene />
        </Series.Sequence>

        {/* Scene 5: Outro & Installation (25s - 30s = 150 frames) */}
        <Series.Sequence durationInFrames={150}>
          <OutroScene />
        </Series.Sequence>
      </Series>
    </Background>
  );
};
