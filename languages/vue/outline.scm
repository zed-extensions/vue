

; Comments appear as annotations in the outline
(comment) @annotation

; ========== template =========
(template_element
  (start_tag
    (tag_name) @name
    (attribute
      (attribute_name) @_lang
      (quoted_attribute_value (attribute_value) @annotation))?)
  (#eq? @_lang "lang")) @item

; Custom elements/components (PascalCase or hyphenated) - self-closing tags
(
  (element
    (self_closing_tag
      (tag_name) @name))
  (#match? @name "^[A-Z]|-")
) @item

; normal tags
(element
  (start_tag
    (tag_name) @name)) @item

; ======= script ======

(script_element
  (start_tag
    (tag_name) @name
    (attribute
      (attribute_name) @_lang
      (quoted_attribute_value (attribute_value) @annotation))?)
  (#eq? @_lang "lang")) @item


; ========= style =========
(style_element
  (start_tag
    (tag_name) @name
    (attribute
      (attribute_name) @_lang
      (quoted_attribute_value (attribute_value) @annotation))?)
  (#eq? @_lang "lang")) @item
