; Comments appear as annotations in the outline
(comment) @annotation

; ========== template =========
(template_element
  (start_tag) @name
  (attribute (attribute_name) @attr_name (quoted_attribute_value (attribute_value) @attr_value))*
) @item


; Custom elements/components (PascalCase or hyphenated) - self-closing tags
(element
  (start_tag
    (tag_name) @name)
) @item

(element
  (self_closing_tag
    (tag_name) @name)
) @item


; ======= script ======

(script_element
    (start_tag) @name
    (raw_text) @context @item
)

(script_element
    (end_tag) @name @item
)


; ========= style =========
(style_element
    (start_tag) @name
    (raw_text) @context
) @item
