use crate::graphics::display_list::DisplayCommand;

use super::layout_box::LayoutBox;

pub fn build_display_list<'a>(layout: &LayoutBox<'a>, commands: &mut Vec<DisplayCommand>) {
    for run in layout.text_runs() {
        let style = run.style();

        commands.push(DisplayCommand::Text {
            text: run.text().to_string(),
            x: run.x(),
            y: run.y(),
            size: style.size,
            bold: style.bold,
            italic: style.italic,
            color: style.color,
        });

        if run.href().is_some() {
            commands.push(DisplayCommand::Rect {
                x: run.x(),
                y: run.y() + 3.0,
                width: run.width(),
                height: 1.0,
                color: style.color,
            });
        }
    }

    for child in layout.children() {
        build_display_list(child, commands);
    }
}
