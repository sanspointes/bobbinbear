import { arrayInsertCircular, arrayOffsetIter, arrayOffsetIterCircular } from '../../src/utils/array';
import { describe, it, expect, beforeEach } from 'vitest';

describe('arrayOffsetIter', () => {
  const testMap = new Map<string, number|boolean>();

  beforeEach(() => {
    testMap.clear();
    new Array(10).fill(0).forEach((_, i) => testMap.set(`${i}`, i));
  })

  it('Traverses from start forwards, yielding all values in array.', () => {
    const arr = [...testMap.keys()];
    for (const k of arrayOffsetIter(arr, 0, 1)) {
      const v = testMap.get(k);
      expect(v).toBeTypeOf('number');
      testMap.set(k, true);
    }

    for (const e of testMap.values()) {
      expect(e).toBe(true);
    }
  })

  it('Traverses from start backwards, yielding the first value, then stops.', () => {
    const arr = [...testMap.keys()];
    for (const k of arrayOffsetIter(arr, 0, -1)) {
      const v = testMap.get(k);
      console.log(`${k}: ${v}`);
      expect(v).to.toBeTypeOf('number');
      testMap.set(k, true);
    }

    for (const [e, v] of testMap.entries()) {
      if (e === '0') expect(v).toBe(true);
      else expect(v).toBeTypeOf('number');
    }
  })

  it('Traverses from `n` forwards, yielding values with an index greater than `n`', () => {
    const arr = [...testMap.keys()];
    const n = 5;

    for (const k of arrayOffsetIter(arr, n)) {
      const v = testMap.get(k);
      expect(v).to.toBeTypeOf('number');
      testMap.set(k, true);
    }

    for (const [e, v] of testMap.entries()) {
      if (Number.parseInt(e) >= n) expect(v).toBe(true);
      else expect(v).toBeTypeOf('number')
    }
  })
  it('Traverses from `n` backwards, yielding values with an index less than `n`.', () => {
    const arr = [...testMap.keys()];
    const n = 5;

    for (const k of arrayOffsetIter(arr, n, -1)) {
      const v = testMap.get(k);
      expect(v).to.toBeTypeOf('number');
      testMap.set(k, true);
    }

    for (const [e, v] of testMap.entries()) {
      if (Number.parseInt(e) <= n) expect(v).toBe(true);
      else expect(v).toBeTypeOf('number')
    }
  })

  it('Traverses from end, yielding a single value then stopping', () => {
    const arr = [...testMap.keys()];
    for (const k of arrayOffsetIter(arr, 9)) {
      const v = testMap.get(k);
      expect(v).to.toBeTypeOf('number');
      testMap.set(k, true);
    }

    for (const [e, v] of testMap.entries()) {
      if (e === '9') expect(v).toBe(true);
      else expect(v).toBeTypeOf('number')
    }
  })
  it('Traverse from end backwards, yielding each value', () => {
    const arr = [...testMap.keys()];
    for (const k of arrayOffsetIter(arr, 9, -1)) {
      const v = testMap.get(k);
      expect(v).to.toBeTypeOf('number');
      testMap.set(k, true);
    }

    for (const e of testMap.values()) {
      expect(e).toBe(true);
    }
  })
})


describe('arrayOffsetIterCircular', () => {
  const testMap = new Map<string, number|boolean>();

  beforeEach(() => {
    testMap.clear();
    new Array(10).fill(0).forEach((_, i) => testMap.set(`${i}`, i));
  })

  it('Traverses array from 0 to end', () => {
    const arr = [...testMap.keys()];
    for (const k of arrayOffsetIterCircular(arr, 0, 1)) {
      const v = testMap.get(k);
      expect(v).toBeTypeOf('number');
      testMap.set(k, true);
    }

    for (const e of testMap.values()) {
      expect(e).toBe(true);
    }
  })

  it('Traverses array from 0 back to 0 (backwards)', () => {
    const arr = [...testMap.keys()];
    for (const k of arrayOffsetIterCircular(arr, 0, -1)) {
      const v = testMap.get(k);
      expect(v).to.toBeTypeOf('number');
      testMap.set(k, true);
    }

    for (const e of testMap.values()) {
      expect(e).toBe(true);
    }
  })

  it('Traverses array from 5 around to 4', () => {
    const arr = [...testMap.keys()];
    for (const k of arrayOffsetIterCircular(arr, 5)) {
      const v = testMap.get(k);
      expect(v).to.toBeTypeOf('number');
      testMap.set(k, true);
    }

    for (const e of testMap.values()) {
      expect(e).toBe(true);
    }
  })
  it('Traverses array from 5 around to 4 (backwards))', () => {
    const arr = [...testMap.keys()];
    for (const k of arrayOffsetIterCircular(arr, 5, -1)) {
      const v = testMap.get(k);
      expect(v).to.toBeTypeOf('number');
      testMap.set(k, true);
    }

    for (const e of testMap.values()) {
      expect(e).toBe(true);
    }
  })

  it('Traverses array from end around to -1', () => {
    const arr = [...testMap.keys()];
    for (const k of arrayOffsetIterCircular(arr, 10)) {
      const v = testMap.get(k);
      expect(v).to.toBeTypeOf('number');
      testMap.set(k, true);
    }

    for (const e of testMap.values()) {
      expect(e).toBe(true);
    }
  })
  it('Traverses array from end around to -1 (backwards))', () => {
    const arr = [...testMap.keys()];
    for (const k of arrayOffsetIterCircular(arr, 10, -1)) {
      const v = testMap.get(k);
      expect(v).to.toBeTypeOf('number');
      testMap.set(k, true);
    }

    for (const e of testMap.values()) {
      expect(e).toBe(true);
    }
  })
})

describe('arrayInsertCircular', () => {
  it('Inserts at start of array when index is 0', () => {
    const arr = [1, 2, 3, 4, 5];

    arrayInsertCircular(arr, 0, 0);

    arr.forEach((el, i) => {
      expect(el).toBe(i);
    })
  })
  it('Inserts at `n` of array when index is `n`', () => {
    const arr = [0, 1, 3, 4, 5];

    arrayInsertCircular(arr, 2, 2);

    arr.forEach((el, i) => {
      expect(el).toBe(i);
    })
  })
  it('Inserts at end of array when index is -1', () => {
    const arr = [0, 1, 2, 3, 4];

    arrayInsertCircular(arr, -1, 5);

    arr.forEach((el, i) => {
      expect(el).toBe(i);
    })
  })
  it('Inserts at end of array when index is -2', () => {
    const arr = [0, 1, 2, 3, 4];

    arrayInsertCircular(arr, -2, 5);

    arr.forEach((el, i) => {
      expect(el).toBe(i);
    })
  })
  it('Inserts at end of array when index is `length`', () => {
    const arr = [0, 1, 2, 3];

    arrayInsertCircular(arr, arr.length, 4);

    arr.forEach((el, i) => {
      expect(el).toBe(i);
    })
  })
  it('Inserts at start of array when index is `length` + 1', () => {
    const arr = [1, 2, 3, 4];

    arrayInsertCircular(arr, arr.length + 1, 0);

    arr.forEach((el, i) => {
      expect(el).toBe(i);
    })
  })
});
