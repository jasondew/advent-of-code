const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const days = [_]u8{1};

    for (days) |day| {
        const name = b.fmt("day{:0>2}", .{day});
        const path = b.fmt("src/day{:0>2}.zig", .{day});

        const exe = b.addExecutable(.{
            .name = name,
            .root_module = b.createModule(.{
                .root_source_file = b.path(path),
                .target = target,
                .optimize = optimize,
            }),
        });
        b.installArtifact(exe);

        const run_cmd = b.addRunArtifact(exe);
        run_cmd.step.dependOn(b.getInstallStep());

        const run_step = b.step(name, b.fmt("Run day {}", .{day}));
        run_step.dependOn(&run_cmd.step);

        const day_tests = b.addTest(.{
            .root_module = b.createModule(.{
                .root_source_file = b.path(path),
                .target = target,
                .optimize = optimize,
            }),
        });

        const run_day_tests = b.addRunArtifact(day_tests);
        if (b.args) |args| {
            run_day_tests.addArgs(args);
        }

        const test_step = b.step(b.fmt("test-{s}", .{name}), b.fmt("Run tests for day {}", .{day}));
        test_step.dependOn(&run_day_tests.step);
    }

    const test_step = b.step("test", "Run all tests");
    for (days) |day| {
        const path = b.fmt("src/day{:0>2}.zig", .{day});

        const day_tests = b.addTest(.{
            .root_module = b.createModule(.{
                .root_source_file = b.path(path),
                .target = target,
                .optimize = optimize,
            }),
        });

        const run_day_tests = b.addRunArtifact(day_tests);
        if (b.args) |args| {
            run_day_tests.addArgs(args);
        }

        test_step.dependOn(&run_day_tests.step);
    }
}
