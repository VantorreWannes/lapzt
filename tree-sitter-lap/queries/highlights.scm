(comment) @string.special

(bit) @constant.builtin @constant
(bit_type) @type.builtin @type

(nand_keyword) @function.builtin @function
(branch_keyword) @keyword
(extern_keyword) @keyword

; Namespaced labels, such as ttt.board and liblapzt.u64.zero.
((label) @property
 (#match? @property "[.]"))

; Simple labels, such as choice and buffer.
((label) @variable
 (#match? @variable "^[a-zA-Z0-9_]+$"))

; Parameters override the generic variable capture.
(parameter name: (label) @variable.parameter)

; Function definitions and calls.
(binding
  target: (typed_identifier name: (label) @function)
  value: (function_definition))
(call_expression function: (label) @function)

; Uppercase labels are constants, such as IO.STDIN and OP.READ.
((label) @constant
 (#match? @constant "^[A-Z0-9_.]+$"))

; Library booleans.
((label) @boolean
 (#match? @boolean "[.](true|false)$"))

; Types override the generic label captures.
(parameter type: (label) @type)
(typed_identifier type: (label) @type)
(function_definition return_type: (label) @type)
(reference_type (label) @type)
(collection_type (label) @type)

; Punctuation.
"=" @operator
"*" @operator

":" @punctuation.delimiter
"," @punctuation.delimiter

"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
