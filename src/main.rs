mod chrome;
mod dom;
mod graphics;
mod layout;
mod parser;
mod util;

use chrome::Chrome;
use dom::structures::Node;
use graphics::display_list::DisplayCommand;
use graphics::renderer::Renderer;
use graphics::text::FontSet;
use graphics::window::{InputEvent, Window};

use layout::builder::build_layout;
use layout::display_list_builder::build_display_list;
use layout::link::{LinkArea, collect_links};

use util::url::{normalize_url, resolve_href};

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
const PAGE_MARGIN: f32 = 24.0;

const HOME_PAGE_HTML: &str = "<html><body>\
<h1>Syrix</h1>\
<p>A small browser, built from scratch in Rust.</p>\
<p>Type an address above and press Enter to browse the web.</p>\
</body></html>";

fn build_page(dom: &Node, fonts: &mut FontSet) -> (Vec<DisplayCommand>, Vec<LinkArea>) {
    let layout = build_layout(
        dom,
        PAGE_MARGIN,
        chrome::HEIGHT + PAGE_MARGIN,
        WINDOW_WIDTH - PAGE_MARGIN * 2.0,
        20.0,
        fonts,
    );

    let mut commands = Vec::new();
    let mut links = Vec::new();

    if let Some((layout, _)) = &layout {
        build_display_list(layout, &mut commands);
        collect_links(layout, &mut links);
    }

    (commands, links)
}

fn load_page(
    url: &str,
    fonts: &mut FontSet,
) -> Result<(Vec<DisplayCommand>, Vec<LinkArea>), Box<dyn std::error::Error>> {
    let html = util::fetch::fetch(url)?;
    let page = util::page::Page::new(url.to_string(), html);
    let dom = parser::parser::parse_html(page.html()).ok_or("Failed to parse HTML!")?;

    Ok(build_page(&dom, fonts))
}

fn load_home(fonts: &mut FontSet) -> (Vec<DisplayCommand>, Vec<LinkArea>) {
    let dom = parser::parser::parse_html(HOME_PAGE_HTML).expect("home page markup is valid");
    build_page(&dom, fonts)
}

fn hit_test(links: &[LinkArea], x: f32, y: f32) -> Option<&str> {
    links
        .iter()
        .find(|link| {
            x >= link.x && x <= link.x + link.width && y >= link.y && y <= link.y + link.height
        })
        .map(|link| link.href.as_str())
}

enum Action {
    Navigate(String),
    Home,
    Reload,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut window = Window::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32, "Syrix Browser")?;

    let renderer = Renderer::new(WINDOW_WIDTH, WINDOW_HEIGHT)?;

    let mut fonts = FontSet::load("assets/fonts")?;

    let chrome = Chrome::new(WINDOW_WIDTH);

    let mut address = String::new();
    let mut current_url = String::new();

    let (mut page_commands, mut links) = load_home(&mut fonts);

    while !window.should_close() {
        let mut action = None;

        for event in window.poll_events() {
            match event {
                InputEvent::Char(c) => address.push(c),

                InputEvent::Backspace => {
                    address.pop();
                }

                InputEvent::Enter => {
                    action = Some(Action::Navigate(normalize_url(&address)));
                }

                InputEvent::Click(x, y) => {
                    if chrome.home_button.contains(x, y) {
                        action = Some(Action::Home);
                    } else if chrome.reload_button.contains(x, y) {
                        action = Some(Action::Reload);
                    } else if let Some(href) = hit_test(&links, x, y) {
                        if let Some(resolved) = resolve_href(&current_url, href) {
                            action = Some(Action::Navigate(resolved));
                        }
                    }
                }
            }
        }

        match action {
            Some(Action::Home) => {
                address.clear();
                current_url.clear();
                let (commands, new_links) = load_home(&mut fonts);
                page_commands = commands;
                links = new_links;
            }

            Some(Action::Reload) => {
                if current_url.is_empty() {
                    let (commands, new_links) = load_home(&mut fonts);
                    page_commands = commands;
                    links = new_links;
                } else {
                    match load_page(&current_url, &mut fonts) {
                        Ok((commands, new_links)) => {
                            page_commands = commands;
                            links = new_links;
                        }
                        Err(err) => eprintln!("Failed to reload {current_url}: {err}"),
                    }
                }
            }

            Some(Action::Navigate(url)) => match load_page(&url, &mut fonts) {
                Ok((commands, new_links)) => {
                    page_commands = commands;
                    links = new_links;
                    address = url.clone();
                    current_url = url;
                }
                Err(err) => eprintln!("Failed to load {url}: {err}"),
            },

            None => {}
        }

        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        renderer.render(&page_commands, &mut fonts)?;
        renderer.render(&chrome.commands(&address), &mut fonts)?;

        window.swap_buffers();
    }

    Ok(())
}
