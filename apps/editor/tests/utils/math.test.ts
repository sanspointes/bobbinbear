import { describe, expect, it } from "vitest";
import { circularDifference } from '../../src/utils/math';

describe('circularDifference', () => {
  describe('with floating points ', () => {
    it('Treats 0-0/1 as 0', () => {
      const v = circularDifference(0, 0, 1);
      expect(v).toBe(0);
    })
    it('Treats 0-0.1/1 as 0.1', () => {
      const v = circularDifference(0, 0.1, 1);
      expect(v).toBe(0.1);
    })
    it('Treats 0-0.5/1 as 0.5', () => {
      const v = circularDifference(0, 0.5, 1);
      expect(v).toBe(0.5);
    })
    it('Treats 0-0.9/1 as -0.09999999999999998', () => {
      const v = circularDifference(0, 0.9, 1);
      expect(v).toBe(-0.09999999999999998);
    })
    it('Treats 0-1/1 as 0', () => {
      const v = circularDifference(0, 1, 1);
      expect(v).toBe(0);
    })
  })
  describe('with ints ', () => {
    it('Treats 0-0/20 as 0', () => {
      const v = circularDifference(0, 0, 20);
      expect(v).toBe(0);
    })
    it('Treats 0-5/20 as 5', () => {
      const v = circularDifference(0, 5, 20);
      expect(v).toBe(5);
    })
    it('Treats 0-10/20 as 10', () => {
      const v = circularDifference(0, 10, 20);
      expect(v).toBe(10);
    })
    it('Treats 0-15/20 as -5', () => {
      const v = circularDifference(0, 15, 20);
      expect(v).toBe(-5);
    })
    it('Treats 0-20/20 as 0', () => {
      const v = circularDifference(0, 20, 20);
      expect(v).toBe(0);
    })
  });
})
