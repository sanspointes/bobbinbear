import { unstable_clientOnly } from "solid-start";

const Editor = unstable_clientOnly(() => import('../editor'));

export default function App() {
  return (
    <main class="text-center mx-auto text-gray-700">
      <Editor />
    </main>
  )
}
