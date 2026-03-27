(comment) @comment

((tag_name) @tag
  (#match? @tag "^[a-z][a-z0-9]*$"))

((tag_name) @tag @tag.component.type.constructor
  (#match? @tag "(^[A-Z][A-Za-z0-9]*$)|(^[a-z][a-z0-9]*(-[a-z0-9]+)+$)"))

(attribute) @attribute

(directive_attribute) @property

(quoted_attribute_value) @string

(interpolation) @punctuation.special

(raw_text) @embedded

(directive_name) @keyword.directive

(directive_argument) @constant

(start_tag) @tag

(end_tag) @tag

(self_closing_tag) @tag

"=" @operator

[
  "<"
  ">"
  "</"
  "/>"
] @punctuation.bracket
