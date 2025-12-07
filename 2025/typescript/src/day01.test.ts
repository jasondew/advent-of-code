import { part1, part2 } from './day01';

const exampleInput = `
  L68
  L30
  R48
  L5
  R60
  L55
  L1
  L99
  R14
  L82
`;

describe('Day 01', () => {
  describe('Part 1', () => {
    it('returns the answer', () => {
      expect(part1(exampleInput)).toBe(3);
    });
  });

  describe('Part 2', () => {
    it('returns the answer', () => {
      expect(part2(exampleInput)).toBe(6);
    });
  });
});
