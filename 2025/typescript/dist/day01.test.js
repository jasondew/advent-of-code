"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const day01_1 = require("./day01");
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
            expect((0, day01_1.part1)(exampleInput)).toBe(3);
        });
    });
    describe('Part 2', () => {
        it('returns the answer', () => {
            expect((0, day01_1.part2)(exampleInput)).toBe(6);
        });
    });
});
