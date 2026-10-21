(comment) @string.special

(bit) @constant.builtin @constant
(bit_type) @type.builtin @type

(nand_keyword) @function.builtin @function
(branch_keyword) @keyword
(extern_keyword) @keyword

(call_expression function: (label) @function)

(parameter name: (label) @variable.parameter)

(typed_identifier name: (label) @property)

(primary (label) @property)
(reference (label) @property)

; Uppercase labels are constants, such as IO.STDIN and OP.READ.
((label) @constant
 (#match? @constant "^[A-Z0-9_.]+$"))

; Library booleans.
((label) @boolean
 (#match? @boolean "[.](true|false)$"))

; Function definitions override the generic property capture.
(binding
  target: (typed_identifier name: (label) @function)
  value: (function_definition))

(parameter type: (label) @type)
(typed_identifier type: (label) @type)
(function_definition return_type: (label) @type)
(reference_type (label) @type)
(collection_type (label) @type)

"=" @operator
"*" @operator

":" @operator
"," @operator

"(" @operator
")" @operator
"[" @operator
"]" @operator
"{" @operator
"}" @operator
