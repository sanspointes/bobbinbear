
export function iterFind<T>(iterable: IterableIterator<T>, predicate: (el: T) => boolean) {
  for (const el of iterable) {
    if (predicate(el)) return el;
  }
}

export function iterSome<T>(iterable: IterableIterator<T>, predicate: (el: T) => boolean) {
  for (const el of iterable) {
    if (predicate(el)) return true;
  }
}
