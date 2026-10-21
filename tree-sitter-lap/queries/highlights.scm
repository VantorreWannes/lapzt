(comment) @comment

(bit) @constant.builtin @constant
(bit_type) @type.builtin @type

(nand_keyword) @function.builtin @function
(branch_keyword) @keyword
(extern_keyword) @keyword

(call_expression function: (label) @function)

(parameter name: (label) @variable.parameter)

(typed_identifier name: (label) @variable)

(primary (label) @variable)
(reference (label) @variable)

(parameter type: (label) @type)
(typed_identifier type: (label) @type)
(function_definition return_type: (label) @type)
(reference_type (label) @type)
(collection_type (label) @type)

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
