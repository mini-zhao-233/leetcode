const std = @import("std");

pub fn main() !void {
    const v = [_]i32{ 0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1 };
    const result = trap(&v);
    std.debug.print("trap result = {}\n", .{result});
}

pub fn trap(height: []const i32) i32 {
    var max_height: i32 = 0;
    var cl: usize = 0;
    var cr = height.len;
    var total_volume: i32 = 0;
    while (cl != cr) {
        const hl = height[cl];
        const hr = height[cr - 1];
        const current_height = @min(hl, hr);
        if (current_height <= max_height) {
            if (hl < hr) {
                cl += 1;
            } else {
                cr -= 1;
            }
            continue;
        }
        var current_layer_volume: i32 = 0;
        for (cl + 1..cr) |i| {
            const column_volume = @max(current_height - @max(height[i], max_height), 0);
            current_layer_volume += column_volume;
        }
        total_volume += current_layer_volume;
        if (hl < hr) {
            cl += 1;
        } else {
            cr -= 1;
        }
        max_height = current_height;
    }
    return total_volume;
}
