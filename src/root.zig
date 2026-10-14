const std = @import("std");
const Tokenizer = @import("tokenizer.zig");
const Lexer = @import("lexer.zig");
const Symbolizer = @import("symbolizer.zig");

test {
    std.testing.refAllDecls(Tokenizer);
    std.testing.refAllDecls(Lexer);
    std.testing.refAllDecls(Symbolizer);
}
