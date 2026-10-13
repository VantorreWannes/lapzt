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

pub fn next(self: *Self) ?Token {
    self.index += countLeadingWhitespace(self.source[self.index..]);
    if (self.index == self.source.len) return null;
    const start = self.index;
    const length = tokenLength(self.source[self.index..]);
    const token = Token{ .start = start, .length = length };
    self.index += length;
    return token;
}

test next {
    var self = Self.init(" Hello World!\n");
    try std.testing.expectEqual(Token{.start = 1, .length = 5}, self.next().?);
    try std.testing.expectEqual(Token{.start = 7, .length = 6}, self.next().?);
    try std.testing.expectEqual(null, self.next());
}
