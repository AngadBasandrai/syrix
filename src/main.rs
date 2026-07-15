mod dom;
mod graphics;
mod layout;
mod parser;
mod renderer;
mod util;

use graphics::color::Color;
use graphics::display_list::DisplayCommand;
use graphics::renderer::Renderer;
use graphics::text::FontSet;
use graphics::window::{InputEvent, Window};

use layout::builder::build_layout;
use layout::display_list_builder::build_display_list;

use util::url::normalize_url;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
const ADDRESS_BAR_HEIGHT: f32 = 36.0;

fn load_page(
    url: &str,
    fonts: &mut FontSet,
) -> Result<Vec<DisplayCommand>, Box<dyn std::error::Error>> {
    let html = util::fetch::fetch(url)?;

    let page = util::page::Page::new(url.to_string(), html);

    let dom = parser::parser::parse_html(page.html()).ok_or("Failed to parse HTML!")?;

    let layout = build_layout(&dom, 0.0, ADDRESS_BAR_HEIGHT, WINDOW_WIDTH, 20.0, fonts);

    let mut commands = Vec::new();

    if let Some((layout, _)) = &layout {
        build_display_list(layout, &mut commands);
    }

    Ok(commands)
}

fn address_bar_commands(address: &str) -> Vec<DisplayCommand> {
    let mut commands = Vec::new();

    commands.push(DisplayCommand::Rect {
        x: 0.0,
        y: 0.0,
        width: WINDOW_WIDTH,
        height: ADDRESS_BAR_HEIGHT,
        color: Color::new(0.9, 0.9, 0.9),
    });

    commands.push(DisplayCommand::Text {
        text: format!("{address}|"),
        x: 10.0,
        y: 10.0,
        size: 18.0,
        bold: false,
        italic: false,
        color: Color::new(0.0, 0.0, 0.0),
    });

    commands
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut window = Window::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32, "Syrix Browser")?;

    let renderer = Renderer::new(WINDOW_WIDTH, WINDOW_HEIGHT)?;

    let mut fonts = FontSet::load("assets/fonts")?;

    let mut address = String::new();
    let mut page_commands = Vec::new();

    while !window.should_close() {
        for event in window.poll_events() {
            match event {
                InputEvent::Char(c) => address.push(c),

                InputEvent::Backspace => {
                    address.pop();
                }

                InputEvent::Enter => {
                    let url = normalize_url(&address);

                    match load_page(&url, &mut fonts) {
                        Ok(commands) => {
                            page_commands = commands;
                            address = url;
                        }
                        Err(err) => {
                            eprintln!("Failed to load {url}: {err}");
                        }
                    }
                }
            }
        }

        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        renderer.render(&page_commands, &mut fonts)?;
        renderer.render(&address_bar_commands(&address), &mut fonts)?;

        window.swap_buffers();
    }

    Ok(())
}
