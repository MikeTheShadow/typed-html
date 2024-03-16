use proc_macro2::{Ident, Span, TokenStream};

use crate::map::StringyMap;

pub fn required_children(element: &str) -> &[&str] {
    match element {
        "html" => &["head", "body"],
        "head" => &["title"],
        _ => &[],
    }
}

pub fn global_attrs(span: Span) -> StringyMap<Ident, TokenStream> {
    let mut attrs = StringyMap::new();
    {
        let mut insert =
            |key, value: &str| attrs.insert(Ident::new(key, span), value.parse().unwrap());

        insert("id", "crate::types::Id");
        insert("class", "crate::types::ClassList");

        insert("accesskey", "String");
        insert("autocapitalize", "String");
        insert("contenteditable", "crate::types::Bool");
        insert("contextmenu", "crate::types::Id");
        insert("dir", "crate::types::TextDirection");
        insert("draggable", "crate::types::Bool");
        insert("hidden", "crate::types::Bool");
        insert("is", "String");
        insert("lang", "crate::types::LanguageTag");
        insert("role", "crate::types::Role");
        insert("style", "String");
        insert("tabindex", "isize");
        insert("title", "String");
        
        // Global htmx parameters. Otherwise, we'll spend too much time "fixing" elements.rs 
        insert("hx-post","String");
        insert("hx-get","String");
        insert("hx-on","String");
        insert("hx-push-url","String");
        insert("hx-target","String");
        insert("hx-swap","String");
        insert("hx-select","String");
        insert("hx-trigger","String");
        insert("hx-confirm","String");
        insert("hx-delete","String");
        
        

        // FIXME ARIA and XML attrs missing
    }
    attrs
}

pub static SELF_CLOSING: &[&str] = &[
    "area",
    "base",
    "br",
    "col",
    "command",
    "embed",
    "hr",
    "img",
    "input",
    "keygen",
    "link",
    "meta",
    "param",
    "source",
    "track",
    "wbr",
];
