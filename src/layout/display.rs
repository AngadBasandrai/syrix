#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Display {
    Block,
    Inline,
    None,
}

pub fn default_display(tag: &str) -> Display {
    match tag {
        "div" | "p" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => Display::Block,
        "span" | "a" | "b" | "strong" | "i" | "em" => Display::Inline,
        "head" | "style" | "script" | "title" | "meta" | "link" => Display::None,
        _ => Display::Block,
    }
}
