const std = @import("std");
const Tokenizer = @import("tokenizer.zig");

const Self = @This();

pub const VariantError = error{UnknownVariant};

pub const Variant = enum {
    label,
    bit,
    nand,
    branch,
    @"extern",
    equals,
    colon,
    comma,
    star,
    left_parenthesis,
    right_parenthesis,
    left_bracket,
    right_bracket,
    left_brace,
    right_brace,

    pub fn isLabel(text: []const u8) bool {
        if (isBit(text)) return false;
        if (isNand(text)) return false;
        if (isBranch(text)) return false;
        if (isExtern(text)) return false;
        if (isEquals(text)) return false;
        if (isColon(text)) return false;
        if (isComma(text)) return false;
        if (isStar(text)) return false;
        if (isLeftParenthesis(text)) return false;
        if (isRightParenthesis(text)) return false;
        if (isLeftBracket(text)) return false;
        if (isRightBracket(text)) return false;
        if (isLeftBrace(text)) return false;
        if (isRightBrace(text)) return false;
        if (std.mem.startsWith(u8, text, ".")) return false;
        if (std.mem.endsWith(u8, text, ".")) return false;
        if (std.mem.allEqual(u8, text, '_')) return false;
        return isValidLabel(text);
    }

    test isLabel {
        try std.testing.expectEqual(true, isLabel("H"));
        try std.testing.expectEqual(true, isLabel("H8"));
        try std.testing.expectEqual(true, isLabel("H_"));
        try std.testing.expectEqual(true, isLabel("H.L"));
        try std.testing.expectEqual(true, isLabel("H"));
        try std.testing.expectEqual(true, isLabel("_H"));
        try std.testing.expectEqual(false, isLabel("BIT"));
        try std.testing.expectEqual(false, isLabel("."));
        try std.testing.expectEqual(false, isLabel("_"));
        try std.testing.expectEqual(false, isLabel(".H"));
    }

    fn isValidLabelChar(char: u8) bool {
        if (std.ascii.isAlphanumeric(char)) return true;
        if (char == '.') return true;
        if (char == '_') return true;
        return false;
    }

    fn isValidLabel(text: []const u8) bool {
        for (text) |char| if (!isValidLabelChar(char)) return false;
        return true;
    }

    pub fn isBit(text: []const u8) bool {
        return std.mem.eql(u8, "BIT", text);
    }

    pub fn isNand(text: []const u8) bool {
        return std.mem.eql(u8, "NAND", text);
    }

    pub fn isBranch(text: []const u8) bool {
        return std.mem.eql(u8, "BRANCH", text);
    }

    pub fn isExtern(text: []const u8) bool {
        return std.mem.eql(u8, "EXTERN", text);
    }

    pub fn isEquals(text: []const u8) bool {
        return std.mem.eql(u8, "=", text);
    }

    pub fn isColon(text: []const u8) bool {
        return std.mem.eql(u8, ":", text);
    }

    pub fn isComma(text: []const u8) bool {
        return std.mem.eql(u8, ",", text);
    }

    pub fn isStar(text: []const u8) bool {
        return std.mem.eql(u8, "*", text);
    }

    pub fn isLeftParenthesis(text: []const u8) bool {
        return std.mem.eql(u8, "(", text);
    }

    pub fn isRightParenthesis(text: []const u8) bool {
        return std.mem.eql(u8, ")", text);
    }

    pub fn isLeftBracket(text: []const u8) bool {
        return std.mem.eql(u8, "[", text);
    }

    pub fn isRightBracket(text: []const u8) bool {
        return std.mem.eql(u8, "]", text);
    }

    pub fn isLeftBrace(text: []const u8) bool {
        return std.mem.eql(u8, "{", text);
    }

    pub fn isRightBrace(text: []const u8) bool {
        return std.mem.eql(u8, "}", text);
    }
};

pub fn variant(text: []const u8) VariantError!Variant {
    if (Variant.isBit(text)) return Variant.bit;
    if (Variant.isNand(text)) return Variant.nand;
    if (Variant.isBranch(text)) return Variant.branch;
    if (Variant.isExtern(text)) return Variant.@"extern";
    if (Variant.isEquals(text)) return Variant.equals;
    if (Variant.isColon(text)) return Variant.colon;
    if (Variant.isComma(text)) return Variant.comma;
    if (Variant.isStar(text)) return Variant.star;
    if (Variant.isLeftParenthesis(text)) return Variant.left_parenthesis;
    if (Variant.isRightParenthesis(text)) return Variant.right_parenthesis;
    if (Variant.isLeftBracket(text)) return Variant.left_bracket;
    if (Variant.isRightBracket(text)) return Variant.right_bracket;
    if (Variant.isLeftBrace(text)) return Variant.left_brace;
    if (Variant.isRightBrace(text)) return Variant.right_brace;
    if (Variant.isLabel(text)) return Variant.label;
    return VariantError.UnknownVariant;
}
