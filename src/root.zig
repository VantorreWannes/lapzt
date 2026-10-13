const std = @import("std");
const Tokenizer = @import("tokenizer.zig");

test {
    std.testing.refAllDecls(Tokenizer);
}