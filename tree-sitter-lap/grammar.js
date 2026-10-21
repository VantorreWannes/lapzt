module.exports = grammar({
  name: 'lap',

  extras: $ => [
    /\s/,
    $.comment,
  ],

  conflicts: $ => [
    [$.typed_identifier, $.primary],
    [$.typed_identifier, $.reference],
    [$.collection_target, $.collection_expression],
  ],

  rules: {
    program: $ => repeat($._statement),

    _statement: $ => choice($.binding, $.expression),

    binding: $ => seq(
      field('target', $._target),
      '=',
      field('value', choice($.expression, $.function_definition)),
    ),

    _target: $ => choice(
      $.typed_identifier,
      $.reference_target,
      $.collection_target,
    ),

    typed_identifier: $ => seq(
      field('name', $.label),
      optional(seq(':', field('type', $._type))),
    ),

    reference_target: $ => seq('*', $.typed_identifier),

    collection_target: $ => seq(
      '[',
      optional(seq($._target, repeat(seq(',', $._target)))),
      ']',
    ),

    expression: $ => choice(
      $.primary,
      $.extern_expression,
      $.branch_expression,
      $.nand_expression,
      $.collection_expression,
      $.call_expression,
    ),

    primary: $ => choice(
      $.label,
      $.reference,
      $.bit,
    ),

    reference: $ => seq('*', $.label),

    bit: $ => $.bit_keyword,

    nand_expression: $ => seq(
      $.nand_keyword,
      '(',
      field('left', $.expression),
      ',',
      field('right', $.expression),
      ')',
    ),

    branch_expression: $ => seq(
      $.branch_keyword,
      '(',
      field('condition', $.expression),
      ')',
      field('then', $.block),
      field('else', $.block),
    ),

    extern_expression: $ => seq(
      $.extern_keyword,
      '(',
      optional($._expression_list),
      ')',
    ),

    collection_expression: $ => seq(
      '[',
      optional($._expression_list),
      ']',
    ),

    call_expression: $ => prec(1, seq(
      field('function', $.label),
      '(',
      optional($._expression_list),
      ')',
    )),

    _expression_list: $ => seq($.expression, repeat(seq(',', $.expression))),

    function_definition: $ => seq(
      '(',
      optional($.parameter_list),
      ')',
      field('return_type', $._type),
      field('body', $.block),
    ),

    parameter_list: $ => seq($.parameter, repeat(seq(',', $.parameter))),

    parameter: $ => seq(
      field('name', $.label),
      ':',
      field('type', $._type),
    ),

    block: $ => seq(
      '{',
      repeat($._statement),
      '}',
    ),

    _type: $ => choice(
      $.bit_type,
      $.label,
      $.reference_type,
      $.collection_type,
    ),

    bit_type: $ => $.bit_keyword,

    reference_type: $ => seq('*', $._type),

    collection_type: $ => seq(
      '[',
      optional(seq($._type, repeat(seq(',', $._type)))),
      ']',
    ),

    label: $ => token(seq(
      /[a-zA-Z0-9_]*[a-zA-Z0-9][a-zA-Z0-9_]*/,
      repeat(seq('.', /[a-zA-Z0-9_]*[a-zA-Z0-9][a-zA-Z0-9_]*/)),
    )),

    bit_keyword: $ => 'BIT',
    nand_keyword: $ => 'NAND',
    branch_keyword: $ => 'BRANCH',
    extern_keyword: $ => 'EXTERN',

    comment: $ => token(seq('//', /[^\n]*/)),
  },
});
