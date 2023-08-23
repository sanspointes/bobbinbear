import { onCleanup } from "solid-js";

type UnmountableRefSetter<T> = (el: T|undefined) => void;

declare module "solid-js" {
  // eslint-disable-next-line @typescript-eslint/no-namespace
  namespace JSX {
    interface Directives { // use:model
      unmountableRef: UnmountableRefSetter<HTMLElement>;
    }
  }
}

export function unmountableRef<T>(el: T, setter: UnmountableRefSetter<T>) {
  setter(el)
  onCleanup(() => setter(undefined));
}
