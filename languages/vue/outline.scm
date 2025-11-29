
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
