console.log('hey')
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

  for (let i = index; i < arr.length + offset; i += direction) {
    const realIndex = i % arr.length;
    const v = arr[realIndex];
    console.log(`${i} [${realIndex}] = ${v}`)
    yield v as T;
  }
}
const arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
console.log('hey')
for (const el of arrayOffsetIterCircular(arr, 5)) {
  console.log(el);
}
