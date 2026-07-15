mod dom;
mod graphics;
mod layout;
mod parser;
mod renderer;
mod util;

use graphics::color::Color;
use graphics::mesh::Mesh;
use graphics::renderer::Renderer;
use graphics::shader::Shader;
use graphics::text::{Font, Glyph};
use graphics::window::Window;
use layout::builder::build_layout;
use layout::painter::paint;
use layout::painter::print_layout;
use renderer::text_renderer::render;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut url = String::new();
    std::io::stdin().read_line(&mut url).unwrap();
    url = url.trim().to_string();

    let html = util::fetch::fetch(&url)?;

    let page = util::page::Page::new(url, html);

    let dom = parser::parser::parse_html(page.html()).ok_or("Failed to parse HTML!")?;

    let layout = build_layout(&dom, 0.0, 0.0, 1280.0, 20.0);

    if let Some(layout) = &layout {
        print_layout(&layout.0, 0);
    }
    // render(&dom);

    let mut window = Window::new(1280, 720, "Syrix Browser")?;

    let renderer = Renderer::new(1280.0, 720.0)?;

    let font = Font::load("assets/fonts/comic.ttf")?;
    let glyph = Glyph::new(&font, 'A', 240.0)?;

    while !window.should_close() {
        window.poll_events();

        unsafe {
            gl::ClearColor(0.15, 0.15, 0.18, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // if let Some(layout) = &layout {
        //     paint(&layout.0, &renderer);
        // }

        renderer.draw_texture(
            glyph.texture(),
            100.0,
            100.0,
            glyph.width(),
            glyph.height(),
            &Color::new(1.0, 1.0, 1.0),
        );

        window.swap_buffers();
    }

    Ok(())
}
