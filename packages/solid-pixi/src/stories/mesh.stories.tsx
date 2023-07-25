import { Texture } from 'pixi.js';
import type { Meta, StoryObj } from 'storybook-solidjs';

import { Canvas, MeshProps } from '../index';

const MeshWrapper = (props: MeshProps) => {
  return <mesh {...props}>
    <planeGeometry />
    <meshMaterial args={[Texture.WHITE, {}]}/>
  </mesh>
}
// More on how to set up stories at: https://storybook.js.org/docs/7.0/solid/writing-stories/introduction
const meta = {
  title: 'Example/mesh',
  component: MeshWrapper,
  tags: ['autodocs'],
  decorators: [(storyfn) => {
    <Canvas>
      {storyfn()}
    </Canvas>
  }],
  argTypes: {
  },
} satisfies Meta<typeof MeshWrapper>;
type Story = StoryObj<typeof meta>;

export default meta;
// type Story = StoryObj<typeof meta>;

// More on writing stories with args: https://storybook.js.org/docs/7.0/solid/writing-stories/args

export const Primary: Story = {
  args: {
    position: [50, 50],
  },
};
