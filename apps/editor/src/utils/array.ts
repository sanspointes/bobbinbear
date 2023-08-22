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

export const arrayFindAfterIndex = <T>(
  arr: T[],
  startIndex: number,
  predicate: ArrayRemovePredicate<T>,
) => {
  if (startIndex > arr.length) {
    throw new Error(
      `arrayFindAfterIndex: Start index '${startIndex}' is greater than array length (${arr.length})`,
    );
  }
  for (let i = startIndex; i < arr.length; i++) {
    const v = arr[i];
    if (predicate(v as T, i, arr)) {
      return v;
    }
  }
  return undefined;
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

export function arrayGetOffset<T>(arr: T[], index: number, offset: number, ciruclar: true): T;
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
};

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
