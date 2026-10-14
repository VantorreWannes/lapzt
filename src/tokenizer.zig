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
    inline for (WHITESPACE) |needle| if (char == needle) return true;
    return false;
}

test isWhitespace {
    try std.testing.expectEqual(false, isWhitespace('.'));
    try std.testing.expectEqual(true, isWhitespace(' '));
    try std.testing.expectEqual(true, isWhitespace('\n'));
    try std.testing.expectEqual(true, isWhitespace('\t'));
    try std.testing.expectEqual(true, isWhitespace('\r'));
}

fn isToken(char: u8) bool {
    return !isWhitespace(char);
}

fn countLeadingWhitespace(text: []const u8) usize {
    for (text, 0..) |char, index| {
        if (!isWhitespace(char)) return index;
    }
    return text.len;
}

test countLeadingWhitespace {
    try std.testing.expectEqual(0, countLeadingWhitespace(""));
    try std.testing.expectEqual(0, countLeadingWhitespace("1 "));
    try std.testing.expectEqual(0, countLeadingWhitespace(". "));
    try std.testing.expectEqual(0, countLeadingWhitespace("A "));
    try std.testing.expectEqual(0, countLeadingWhitespace("a "));
    try std.testing.expectEqual(4, countLeadingWhitespace(" \n\t\r"));
}

fn tokenLength(text: []const u8) usize {
    for (text, 0..) |char, index| {
        if (!isToken(char)) return index;
    }
    return text.len;
}

test tokenLength {
    try std.testing.expectEqual(0, tokenLength(""));
    try std.testing.expectEqual(0, tokenLength(" \n\t\r"));
    try std.testing.expectEqual(0, tokenLength(" Hello"));
    try std.testing.expectEqual(5, tokenLength("Hello"));
    try std.testing.expectEqual(5, tokenLength("Hello world"));
}

fn isComment(text: []const u8) bool {
    return std.mem.startsWith(u8, text, "//");
}

fn commentLength(text: []const u8) usize {
    if (!isComment(text)) return 0;
    for (text, 0..) |char, index| {
        if (char == '\n') return index;
    }
    return text.len;
}

fn leadingFillerLength(text: []const u8) usize {
    var index: usize = 0;

    while (index < text.len) {
        const comment_length = commentLength(text[index..]);
        const whitespace_length = countLeadingWhitespace(text[index..]);
        const filler_length = comment_length + whitespace_length;
        if (filler_length == 0) break;
        index += filler_length;
    }
    return index;
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

pub fn next(self: *Self) ?Token {
    self.index += leadingFillerLength(self.source[self.index..]);
    if (self.index == self.source.len) return null;
    const start = self.index;
    const length = tokenLength(self.source[self.index..]);
    const token = Token{ .start = start, .length = length };
    self.index += length;
    return token;
}

test next {
    var self = Self.init(" // Say hello. \n // Start program. \n Hello World!\n // Stop program.");
    try std.testing.expectEqual(Token{ .start = 37, .length = 5 }, self.next().?);
    try std.testing.expectEqual(Token{ .start = 43, .length = 6 }, self.next().?);
    try std.testing.expectEqual(null, self.next());
}
