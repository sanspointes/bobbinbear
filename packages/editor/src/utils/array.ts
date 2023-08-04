type ArrayRemovePredicate<T> = (el: T, index: number, obj: T[]) => boolean;

/**
 * Removes an element from an array using array.splice()
 * @template T - 
 * @param arr - 
 * @param predicate - 
 * @returns Whether or not an element was removed
 */
export const arrayRemove = <T>(arr: T[], predicate: ArrayRemovePredicate<T>): boolean => {
  const index = arr.findIndex(predicate);
  if (index >= 0)  {
    arr.splice(index, 1);
    return true;
  }
  return false;
}

export const arrayRemoveEl = <T>(arr: T[], el: T) => {
  return arrayRemove(arr, entry => entry === el);
}

export const arrayLast = <T>(arr: T[]): T|undefined => {
  if (arr.length === 0) return undefined;

  return arr[arr.length -1];
}
