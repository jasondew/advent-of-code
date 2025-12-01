const std = @import("std");
const helpers = @import("helpers.zig");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const input = try helpers.readInputFile(allocator, "../inputs/01");

    const part1_answer = try part1(input);
    helpers.print("Part 1: {}\n", .{part1_answer});

    const part2_answer = try part2(input);
    helpers.print("Part 2: {}\n", .{part2_answer});
}

pub fn part1(input: []const u8) !usize {
    var zero_count: usize = 0;
    var dial: usize = 50;

    var iterator = std.mem.tokenizeScalar(u8, input, '\n');
    while (iterator.next()) |line| {
        if (line.len != 0) {
            const direction = line[0];
            const value = try std.fmt.parseInt(usize, line[1..], 10);

            switch (direction) {
                'L' => {
                    dial = (dial + 100 - (value % 100)) % 100;
                },
                'R' => {
                    dial = (dial + value) % 100;
                },
                else => {
                    std.debug.print("Unknown direction: {c}\n", .{direction});
                },
            }

            if (dial == 0) {
                zero_count += 1;
            }
        }
    }

    return zero_count;
}

pub fn part2(input: []const u8) !usize {
    var zero_count: usize = 0;
    var dial: usize = 50;

    //    std.debug.print("\nStarting Dial: {d}\n", .{dial});
    var iterator = std.mem.tokenizeScalar(u8, input, '\n');
    while (iterator.next()) |line| {
        if (line.len != 0) {
            //            std.debug.print("Instruction: {s}, Dial (before): {d}", .{ line, dial });
            const direction = line[0];
            const value = try std.fmt.parseInt(usize, line[1..], 10);

            switch (direction) {
                'L' => {
                    zero_count += if (dial == 0) value / 100 else ((100 - dial) + value) / 100;
                    dial = (dial + 100 - (value % 100)) % 100;
                },
                'R' => {
                    zero_count += (dial + value) / 100;
                    dial = (dial + value) % 100;
                },
                else => {
                    std.debug.print("Unknown direction: {c}\n", .{direction});
                },
            }
            //            std.debug.print("  Dial (after): {d}, Zero Count: {d}\n", .{ dial, zero_count });
        }
    }

    return zero_count;
}

test "example input" {
    const input =
        \\L68
        \\L30
        \\R48
        \\L5
        \\R60
        \\L55
        \\L1
        \\L99
        \\R14
        \\L82
    ;
    try std.testing.expectEqual(3, part1(input));
    try std.testing.expectEqual(6, part2(input));
}

test "part 2 crossing 0" {
    try std.testing.expectEqual(1, part2("L50\nR50\n"));
    try std.testing.expectEqual(1, part2("L50\nL50\n"));
    try std.testing.expectEqual(1, part2("R50\nL50\n"));
    try std.testing.expectEqual(1, part2("R50\nR50\n"));
}
