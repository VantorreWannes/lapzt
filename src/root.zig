const std = @import("std");
const Tokenizer = @import("tokenizer.zig");
const Lexer = @import("lexer.zig");

test {
    std.testing.refAllDecls(Tokenizer);
    std.testing.refAllDecls(Lexer);
}