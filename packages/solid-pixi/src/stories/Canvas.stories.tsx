/** @jsxImportSource solid-js */
import { Texture } from 'pixi.js';
import type { Meta, StoryObj } from 'storybook-solidjs';

import { Canvas } from '../index';

// More on how to set up stories at: https://storybook.js.org/docs/7.0/solid/writing-stories/introduction
const meta = {
  title: 'Example/Canvas',
  component: Canvas,
  tags: ['autodocs'],
  argTypes: {
  },
} satisfies Meta<typeof Canvas>;

export default meta;
// type Story = StoryObj<typeof meta>;

// More on writing stories with args: https://storybook.js.org/docs/7.0/solid/writing-stories/args

export const Primary = () => (
  <Canvas>
    <mesh position={[200, 200]}>
      <planeGeometry />
      <meshMaterial texture={Texture.WHITE}/>
    </mesh>
  </Canvas>
)
Primary.story = {
  name: 'Default',
}
