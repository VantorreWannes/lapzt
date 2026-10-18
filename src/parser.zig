const std = @import("std");
const Tokenizer = @import("tokenizer.zig");
const Lexer = @import("lexer.zig");
const Self = @This();

pub const Label = Tokenizer.Token;

pub const Type = union(enum) {
    bit,
    label: Label,
    referenced: Type,
    collection: []Type,
};

pub const Parameter = struct {
    label: Label,
    type: Type,
};

pub const Function = struct {
    parameters: []Parameter,
    type: Type,
    block: Block,
};

pub const CallExpression = struct {
    label: Label,
    parameters: []Expression,
};

pub const CollectionExpression = struct {
    fields: []Expression,
};
pub const ExternExpression = struct {
    parameters: []Expression,
};
pub const BranchExpression = struct {
    condition: Expression,
    left: Block,
    right: Block,
};
pub const NandExpression = struct {
    left: Block,
    right: Block,
};
pub const LabelExpression = union(enum) {
    bit,
    copied: Label,
    referenced: Label,
};
pub const Expression = union(enum) {
    call: CallExpression,
    collection: CollectionExpression,
    @"extern": ExternExpression,
    branch: BranchExpression,
    nand: NandExpression,
    label: LabelExpression,
};
pub const Identifier = struct {
    label: Label,
    type: ?Type,
};
pub const BindingTarget = union(enum) {
    copied: Identifier,
    referenced: Identifier,
    collection: []BindingTarget,
};
pub const BindingSource = union(enum) {
    expression: Expression,
    function: Function,
};
pub const Binding = struct {
    target: BindingTarget,
    source: BindingSource,
};
pub const Statement = union(enum) {
    binding: Binding,
    expression: Expression,
};
pub const Block = struct {
    statements: []Statement,
    expression: ?Expression,
};
pub const Program = []Statement;

pub const ParseTypeError = error{
    MissingToken
};

// fn parseType(tokenizer: *Tokenizer) ParseTypeError!Type {
//     if(tokenizer.next()) |token| {
// 
//     }
// }

// pub fn parse(source: []const u8) Error!Program {
//     var tokenizer = Tokenizer.init(source);
// }
