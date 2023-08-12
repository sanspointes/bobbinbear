import { createSignal, Show } from "solid-js";
import { FaRegularFaceSadCry } from 'solid-icons/fa'
import { Command } from "../store/commands";
import { Button } from "./generics/Button";
import { CommandStack } from "./CommandStack";
import { TbX } from "solid-icons/tb";

type ErrorProps = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  error: Error;
  reset: () => void;
  stack: Command[];
};
export const ErrorView = (props: ErrorProps) => {
  const [showExtra, setShowExtra] = createSignal(true);

  const error = props.error;
  if (error instanceof Error) {
    console.error(error);
  }

  const handleReload = async (_isReporting: boolean) => {
    window.location.reload();
  }

  return (
    <div class="p-8 w-full h-full bg-yellow-900 flex justify-center">
      <div class="relative w-[90%] max-w-[600px] h-full">
        <div
          class="absolute top-1/3 left-1/2 -translate-x-1/2 -translate-y-1/2 flex flex-col gap-2 justify-center w-full"
          classList={{
            "h-full": showExtra(),
          }}
        >
          <FaRegularFaceSadCry class="w-32 h-32 fill-yellow-200"/>
          <h1 class="w-full text-3xl font-bold text-yellow-200">
            Uh oh! A very bad error has occured.
          </h1>

          <p class="w-full text-yellow-200 mb-6">
            <span class="line-through">Help me out! Please consider reporting the error so I can fix it.</span>  <br/> I don't have error reporting set up yet. Just hit the reload button.
          </p>

          <div class="flex gap-4">
            <Button onClick={() => handleReload(true)}>Report error and reload</Button>
            <Button
              variant="secondary"
              inverted={true}
              class="text-yellow-200 border-yellow-200"
              onClick={() => setShowExtra(!showExtra())}
            >
              What am I reporting?
            </Button>
            <Button
              variant="secondary"
              inverted={true}
              class="text-yellow-200 border-yellow-200"
              onClick={() => handleReload(false)}
            >
              Just reload
            </Button>
          </div>

        </div>


        <Show when={showExtra()}>
          <div class="absolute w-full left-1/2 -translate-x-1/2 bottom-6 flex flex-col gap-2 items-start text-sm">
            <div class="flex w-full justify-between">
              <h2 class="text-yellow-200 text-lg font-bold">What gets reported?</h2>
              <Button inverted={true} size="small" onClick={() => setShowExtra(false)}><TbX /></Button>
            </div>
            <p class="text-yellow-200 mb-6">We send the following error data and where it came from in the code.  We also send data on your last few changes.  See the Command stack button below.</p>
            <div class="overflow-hidden p-4 mb-2 w-full rounded-md bg-yellow-950">
              <Show when={props.error.message}>
                <p class="text-yellow-200">Reason: {props.error.message}</p>
              </Show>
            </div>
            <CommandStack class="w-[80vw] max-w-[80vw] h-[80vh] max-h-[80vh]" stack={props.stack} />
          </div>
        </Show>
      </div>
    </div>
  );
};
