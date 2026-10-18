const std = @import("std");
const Self = @This();
const WHITESPACE = [_]u8{
    ' ',
    '\t',
    '\r',
    '\n',
};

pub const Token = struct { start: usize, length: usize };

source: []const u8,
index: usize,

pub fn init(source: []const u8) Self {
    return .{ .index = 0, .source = source };
}

test init {
    const self = Self.init("");
    try std.testing.expectEqual(0, self.index);
    try std.testing.expectEqualStrings("", self.source);
}

fn isWhitespace(char: u8) bool {
    return switch (char) {
        ' ', '\t', '\r', '\n' => true,
        else => false,
    };
}

test isWhitespace {
    try std.testing.expectEqual(false, isWhitespace('.'));
    try std.testing.expectEqual(true, isWhitespace(' '));
    try std.testing.expectEqual(true, isWhitespace('\t'));
    try std.testing.expectEqual(true, isWhitespace('\r'));
    try std.testing.expectEqual(true, isWhitespace('\n'));
}

fn leadingWhitespaceLength(source: []const u8) usize {
    for (source, 0..) |char, index| if (!isWhitespace(char)) return index;
    return source.len;
}

fn leadingCommentLength(source: []const u8) usize {
    if (!std.mem.startsWith(u8, source, "//")) return 0;
    if (std.mem.findScalar(u8, source, '\n')) |newline| {
        return newline + 1;
    }
    return source.len;
}

fn leadingFillerLength(source: []const u8) usize {
    var index: usize = 0;
    while (index < source.len) {
        var current: usize = 0;
        current += leadingCommentLength(source[index + current ..]);
        current += leadingWhitespaceLength(source[index + current ..]);
        if (current == 0) return index;
        index += current;
    }
    return index;
}

test leadingFillerLength {
    try std.testing.expectEqual(0, leadingFillerLength(""));
    try std.testing.expectEqual(0, leadingFillerLength("code"));
    try std.testing.expectEqual(4, leadingFillerLength(" \n\t\r"));
    try std.testing.expectEqual(4, leadingFillerLength(" \n\t\rcode"));
    try std.testing.expectEqual(6, leadingFillerLength("// hi\ncode"));
    try std.testing.expectEqual(7, leadingFillerLength(" // hi\ncode"));
    try std.testing.expectEqual(8, leadingFillerLength(" // hi\n code"));
    try std.testing.expectEqual(5, leadingFillerLength("// hi"));
    try std.testing.expectEqual(0, leadingFillerLength("/ code"));
}

pub fn slice(self: *const Self, token: Token) []const u8 {
    return self.source[token.start..][0..token.length];
}

test slice {
    var tokenizer = Self.init(" Hello World!\n");
    {
        const token = tokenizer.next().?;
        try std.testing.expectEqualStrings("Hello", tokenizer.slice(token));
    }
    {
        const token = tokenizer.next().?;
        try std.testing.expectEqualStrings("World!", tokenizer.slice(token));
    }
}

fn leadingTokenLength(text: []const u8) usize {
    for (text, 0..) |char, index| if (isWhitespace(char)) return index;
    return text.len;
}

test leadingTokenLength {
    try std.testing.expectEqual(0, leadingTokenLength(""));
    try std.testing.expectEqual(0, leadingTokenLength(" \n\t\r"));
    try std.testing.expectEqual(0, leadingTokenLength(" Hello"));
    try std.testing.expectEqual(5, leadingTokenLength("Hello"));
    try std.testing.expectEqual(5, leadingTokenLength("Hello world"));
}

pub fn seek(self: *Self, index: usize) void {
    self.index = index;
}

pub fn reset(self: *Self) void {
    self.index = 0;
}

pub fn peek(self: *const Self) ?Token {
    const start = self.index + leadingFillerLength(self.source[self.index..]);
    const length = leadingTokenLength(self.source[start..]);
    if (length == 0) return null;
    return Token{ .start = start, .length = length };
}

pub fn next(self: *Self) ?Token {
    if (self.peek()) |token| {
        self.index = token.start + token.length;
        return token;
    }
    return null;
}

test next {
    var self = Self.init(" // Say hello. \n // Start program. \n Hello World!\n // Stop program.");
    try std.testing.expectEqual(Token{ .start = 37, .length = 5 }, self.next().?);
    try std.testing.expectEqual(Token{ .start = 43, .length = 6 }, self.next().?);
    try std.testing.expectEqual(null, self.next());
}

pub fn isEmpty(self: *Self) bool {
    return self.peek() == null;
}


pub fn skip(self: *Self) void {
    _ = self.next();
}
