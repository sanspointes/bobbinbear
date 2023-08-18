import { createSignal, Show } from "solid-js";
import { FaRegularFaceSadCry } from "solid-icons/fa";
import { Command } from "../store/commands";
import { Button } from "./generics/Button";
import { CommandStack } from "./CommandStack";
import { TbX } from "solid-icons/tb";
import { IoWarningOutline } from "solid-icons/io";
import RError from "rerror";
import { Collapsible as KCollapsible } from "@kobalte/core";


type ErrorReasonProps = {
  error: Error;
};
const ErrorReason = (props: ErrorReasonProps) => {
  return (
    <KCollapsible.Root class="overflow-hidden mb-2 w-full rounded-md bg-orange-950">
      <KCollapsible.Trigger class="w-full text-left">
        <p class="text-orange-200 p-4">
          <IoWarningOutline class="inline mr-1" />Reason:{" "}
          {props.error.message}
        </p>
      </KCollapsible.Trigger>
      <KCollapsible.Content class="pl-6 overflow-scroll">
        <Show when={props.error.stack}>
          {(stack) => <pre class="text-orange-200 text-xs">{stack()}</pre>}
        </Show>
      </KCollapsible.Content>
      <Show when={props.error.cause}>
        {(cause) => <ErrorReason error={cause()} />}
      </Show>
    </KCollapsible.Root>
  );
};

type ErrorProps = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  error: Error | RError;
  stack?: Command[];
};
export const ErrorView = (props: ErrorProps) => {
  const [showExtra, setShowExtra] = createSignal(true);

  const error = props.error;
  if (error instanceof Error) {
    console.error(error);
  }

  const handleReload = async (_isReporting: boolean) => {
    window.location.reload();
  };

  return (
    <div class="flex justify-center p-8 w-full h-full bg-orange-900">
      <div class="relative w-[90%] max-w-[600px] h-full">
        <div
          class="flex absolute left-1/2 top-1/3 flex-col gap-2 justify-center w-full -translate-x-1/2 -translate-y-1/2"
          classList={{
            "h-full": showExtra(),
          }}
        >
          <FaRegularFaceSadCry class="w-32 h-32 fill-orange-200" />
          <h1 class="w-full text-3xl font-bold text-orange-200">
            Uh oh! A very bad error has occured.
          </h1>

          <p class="mb-6 w-full text-orange-200">
            <span class="line-through">
              Help me out! Please consider reporting the error so I can fix it.
            </span>{" "}
            <br />{" "}
            I don't have error reporting set up yet. Just hit the reload button.
          </p>

          <div class="flex gap-4">
            <Button onClick={() => handleReload(true)}>
              Report error and reload
            </Button>
            <Button
              variant="secondary"
              inverted={true}
              class="text-orange-200 border-orange-200"
              onClick={() => setShowExtra(!showExtra())}
            >
              What am I reporting?
            </Button>
            <Button
              variant="secondary"
              inverted={true}
              class="text-orange-200 border-orange-200"
              onClick={() => handleReload(false)}
            >
              Just reload
            </Button>
          </div>
        </div>

        <Show when={showExtra()}>
          <div class="flex absolute bottom-6 left-1/2 flex-col gap-2 items-start w-full text-sm -translate-x-1/2 bg-orange-900 max-h-[80vh] py-4">
            <div class="flex justify-between w-full">
              <h2 class="text-lg font-bold text-orange-200">
                What gets reported?
              </h2>
              <Button
                inverted={true}
                size="small"
                onClick={() => setShowExtra(false)}
              >
                <TbX />
              </Button>
            </div>
            <p class="mb-6 text-orange-200">
              We send the following error data and where it came from in the
              code. We also send data on your last few changes. See the Command
              stack button below.
            </p>
            <ErrorReason error={error} />
            <Show when={props.stack}>
              {(stack) => (
                <CommandStack
                  class="w-[80vw] max-w-[80vw] h-[80vh] max-h-[80vh]"
                  stack={stack()}
                />
              )}
            </Show>
          </div>
        </Show>
      </div>
    </div>
  );
};
