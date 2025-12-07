const std = @import("std");

pub fn print(comptime fmt: []const u8, args: anytype) void {
    var stdout_buffer: [4096]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout = &stdout_writer.interface;

    stdout.print(fmt, args) catch {};
    stdout.flush() catch {};
}

pub const InputError = error{
    FileNotFound,
    OutOfMemory,
    GenericIOError,
};

/// Reads the entire contents of a file into memory using the provided allocator.
/// The caller is responsible for de-initializing the allocator (using `defer`).
pub fn readInputFile(
    allocator: std.mem.Allocator,
    filename: []const u8,
) InputError![]const u8 {
    // 1. Open the file. This can fail if the file doesn't exist.
    const file = std.fs.cwd().openFile(filename, .{}) catch |err| {
        return switch (err) {
            error.FileNotFound => InputError.FileNotFound,
            // Re-map other potential errors to a generic IO error
            else => InputError.GenericIOError,
        };
    };
    defer file.close(); // Crucial: defer ensures the file handle is closed

    // 2. Read the entire file content into a buffer using the provided allocator.
    // This can fail if memory allocation fails (OutOfMemory).
    const content = file.readToEndAlloc(allocator, std.math.maxInt(usize)) catch |err| {
        return switch (err) {
            error.OutOfMemory => InputError.OutOfMemory,
            // Re-map other potential read errors
            else => InputError.GenericIOError,
        };
    };

    // 3. The content slice is owned by the allocator. We return the slice.
    return content;
}
