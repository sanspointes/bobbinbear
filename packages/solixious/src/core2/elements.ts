import { Constructor } from "./types";

export const createPixiObjectProxy = <TSource extends Constructor>(
  constructor: TSource,
) => {
  const Component = () => {

  }
  return Component as unknown as JSX.Element;
};

export const T;
