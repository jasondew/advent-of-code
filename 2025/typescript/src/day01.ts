import * as fs from 'fs';
import * as path from 'path';

export function part1(input: string): number {
  let zeroCount = 0;
  let dial = 50;

  for (const rawLine of input.trim().split('\n')) {
    const line = rawLine.trim();
    const direction = line[0];
    const amount = parseInt(line.slice(1), 10);

    if (direction === 'L') {
      dial -= amount;
    } else if (direction === 'R') {
      dial += amount;
    } else {
      throw new Error(`Unknown direction: ${direction}`);
    }

    dial = (dial + 100) % 100;

    if (dial === 0) {
      zeroCount += 1;
    }
  }

  return zeroCount;
}

export function part2(input: string): number {
  let zeroCount = 0;
  let dial = 50;

  for (const rawLine of input.trim().split('\n')) {
    const line = rawLine.trim();
    const direction = line[0];
    const amount = parseInt(line.slice(1), 10);

    if (direction === 'L') {
      if (dial === 0) {
        zeroCount += Math.floor(amount / 100);
      } else {
        zeroCount += Math.floor((100 - dial + amount) / 100);
      }

      dial = dial + 100 - (amount % 100);
    } else if (direction === 'R') {
      zeroCount += Math.floor((dial + amount) / 100);
      dial += amount % 100;
    } else {
      throw new Error(`Unknown direction: ${direction}`);
    }

    dial %= 100;
  }

  return zeroCount;
}

if (require.main === module) {
  const input = fs.readFileSync(path.join(__dirname, '../../inputs/01'), 'utf-8');
  console.log('Part 1:', part1(input));
  console.log('Part 2:', part2(input));
}
