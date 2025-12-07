import * as fs from 'fs';
import * as path from 'path';

export function part1(input: string): number {
  const lines = input.trim().split('\n');
  // TODO: Implement solution
  return 0;
}

export function part2(input: string): number {
  const lines = input.trim().split('\n');
  // TODO: Implement solution
  return 0;
}

// Run if executed directly
if (require.main === module) {
  const input = fs.readFileSync(path.join(__dirname, '../inputs/XX.txt'), 'utf-8');
  console.log('Part 1:', part1(input));
  console.log('Part 2:', part2(input));
}
