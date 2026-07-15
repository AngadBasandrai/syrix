use super::layout_box::LayoutBox;

pub struct LinkArea {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub href: String,
}

pub fn collect_links<'a>(layout: &LayoutBox<'a>, out: &mut Vec<LinkArea>) {
    for run in layout.text_runs() {
        if let Some(href) = run.href() {
            let size = run.style().size;

            out.push(LinkArea {
                x: run.x(),
                y: run.y() - size * 0.8,
                width: run.width(),
                height: size,
                href: href.to_string(),
            });
        }
    }

    for child in layout.children() {
        collect_links(child, out);
    }
}
