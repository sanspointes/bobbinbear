type ArrayRemovePredicate<T> = (el: T, index: number, obj: T[]) => boolean;

/**
 * Removes an element from an array using array.splice()
 * @template T -
 * @param arr -
 * @param predicate -
 * @returns Whether or not an element was removed
 */
export const arrayRemove = <T>(
  arr: T[],
  predicate: ArrayRemovePredicate<T>,
): boolean => {
  const index = arr.findIndex(predicate);
  if (index >= 0) {
    arr.splice(index, 1);
    return true;
  }
  return false;
};

export const arrayRemoveEl = <T>(arr: T[], el: T) => {
  return arrayRemove(arr, (entry) => entry === el);
};

export const arrayLast = <T>(arr: T[]): T | undefined => {
  if (arr.length === 0) return undefined;

  return arr[arr.length - 1];
};

export const arrayFirst = <T>(arr: T[]): T | undefined => {
  if (arr.length === 0) return undefined;

  return arr[0];
};

/**
 * Creates a new iterator from an array that starts from an offset index
 */
export function* arrayOffsetIter<T>(
  arr: T[],
  startIndex: number,
  direction = +1,
): IterableIterator<T> {
  let index = startIndex;
  while (index < 0) {
    index += arr.length;
  }
  index = index % arr.length;

  const endIndex = direction > 0 ? arr.length : 0;

  if (direction > 0) {
    for (let i = index; i < endIndex; i += direction) {
      yield arr[i] as T;
    }
  } else {
    for (let i = index; i >= endIndex; i += direction) {
      yield arr[i] as T;
    }
  }
}

/**
 * Iterates over every element of the array including the first one twice (start and end)
 */
export function* arrayIterCircularEndInclusive<T>(arr: T[]) {
  if (arr.length === 0) return;
  for (const v of arr) {
    yield v;
  }
  yield arrayFirst(arr) as T;
}
/**
 * Creates a new iterator from an array that wraps around and starts at and stops before an offset index.
 */
export function* arrayOffsetIterCircular<T>(
  arr: T[],
  startIndex: number,
  direction = +1,
): IterableIterator<T> {
  if (arr.length === 0) return;
  // Wrap it to the bounds of 0-arr.length
  let index = startIndex;
  while (index < 0) {
    index += arr.length;
  }
  index = index % arr.length;
  const offset = index;

  const maxIters = arr.length;
  let actualIters = 0;
  let iterOffset = 0;
  while (actualIters < maxIters) {
    let index = (offset + iterOffset) % arr.length;
    while (index < 0) {
      index += arr.length;
    }
    const v = arr[index];
    yield v as T;
    actualIters += 1;
    iterOffset += direction;
  }
}
/**
 * Creates an iterable of the pairs of an array.  Optionally circular.
 */
export function* arrayIterPairs<T>(iterable: T[], circular: boolean) {
  const iterator = iterable[Symbol.iterator]();
  let a = iterator.next();
  if (a.done) return;
  let b = iterator.next();
  while (!b.done) {
    const toYield = [a.value, b.value] as [prev: T, curr: T];
    yield toYield;
    a = b;
    b = iterator.next();
  }
  if (circular) {
    yield [a.value, iterable[0] as T] as [prev: T, curr: T];
  }
}

export function arrayGetOffset<T>(
  arr: T[],
  index: number,
  offset: number,
  ciruclar: true,
): T;
export function arrayGetOffset<T>(
  arr: T[],
  index: number,
  offset: number,
  ciruclar?: boolean,
) {
  let newIndex = index + offset;
  if (ciruclar) {
    while (newIndex < 0) {
      newIndex += arr.length;
    }
    while (newIndex >= arr.length) {
      newIndex -= arr.length;
    }
  } else {
    if (newIndex < 0 || newIndex >= arr.length) return undefined;
  }
  return arr[newIndex];
}

export const arrayInsertAtIndex = <T>(arr: T[], el: T, index: number) => {
  arr.splice(index, 0, el);
};

export const arrayMoveElToIndex = <T>(
  arr: T[],
  el: T,
  index: number,
): boolean => {
  const success = arrayRemoveEl(arr, el);
  if (success) {
    arrayInsertAtIndex(arr, el, index);
  }
  return success;
};

export const arrayInsertCircular = <T>(
  arr: T[],
  index: number,
  ...values: T[]
) => {
  let idx = index;
  idx = idx % arr.length;
  while (idx < 0) {
    idx += arr.length + 1;
  }
  arr.splice(idx, 0, ...values);
}

export const arrayGetCircular = <T>(
  arr: T[],
  index: number,
) => {
  let idx = index;
  idx = idx % arr.length;
  while (idx < 0) {
    idx += arr.length + 1;
  }
  return arr.at(idx);
}
export const arraySetCircular = <T>(
  arr: T[],
  index: number,
  value: T,
) => {
  let idx = index;
  idx = idx % arr.length;
  while (idx < 0) {
    idx += arr.length + 1;
  }
  return arr.splice(idx, 1, value);
}

export const arrayFindFromCircular = <T>(
  arr: T[],
  index: number,
  predicate: (value: T) => boolean,
) => {
  const iter = arrayOffsetIterCircular(arr, index);
  for (const val of iter) {
    if (predicate(val)) {
      return val;
    }
  }
  return undefined;
}
export const arrayFindFromBackwardsCircular = <T>(
  arr: T[],
  index: number,
  predicate: (value: T) => boolean,
) => {
  const iter = arrayOffsetIterCircular(arr, index, -1);
  for (const val of iter) {
    if (predicate(val)) {
      return val;
    }
  }
  return undefined;
}
