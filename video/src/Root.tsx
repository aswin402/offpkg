import React from 'react';
import { Composition } from 'remotion';
import { HowItWorks } from './HowItWorks';
import { RealTuiScene } from './scenes/RealTuiScene';

export const RemotionRoot: React.FC = () => {
  return (
    <>
      <Composition
        id="HowItWorks"
        component={HowItWorks}
        durationInFrames={900}
        fps={30}
        width={1920}
        height={1080}
        defaultProps={{}}
      />
      <Composition
        id="RealTui"
        component={RealTuiScene}
        durationInFrames={360}
        fps={30}
        width={1200}
        height={750}
        defaultProps={{}}
      />
    </>
  );
};
