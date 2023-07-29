import { Context, createContext } from 'solid-js';
import { createRoot } from './renderer';
import { WrapConstructableOptions, wrapConstructable } from './elements';
import { Constructable } from './types';

// Solixi: Pixi renderer for solid.js 

export const createRenderer = <TState extends {}>(initialState: TState) => {
  const Context = createContext<TState>(initialState);
  return {
    createRoot: <TRootObject>(rootObject: TRootObject) => createRoot<TRootObject, TState>(rootObject, Context, initialState),
    wrapConstructable: <TSource extends Constructable>(source: TSource, options: WrapConstructableOptions<TSource>) => wrapConstructable(source, options),
  }
}

