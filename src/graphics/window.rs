use glfw::{Context, Glfw, GlfwReceiver, PWindow, WindowEvent};

pub enum InputEvent {
    Char(char),
    Backspace,
    Enter,
    Click(f32, f32),
}

pub struct Window {
    glfw: Glfw,
    window: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut glfw = glfw::init(glfw::fail_on_errors)?;

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .ok_or("Failed to create GLFW window")?;

        window.make_current();

        window.set_key_polling(true);
        window.set_char_polling(true);
        window.set_mouse_button_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_close_polling(true);

        gl::load_with(|symbol| {
            window
                .get_proc_address(symbol)
                .map_or(std::ptr::null(), |p| p as *const _)
        });

        Ok(Self {
            glfw,
            window,
            events,
        })
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn poll_events(&mut self) -> Vec<InputEvent> {
        self.glfw.poll_events();

        let mut input = Vec::new();

        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    self.window.set_should_close(true);
                }

                WindowEvent::Key(glfw::Key::Backspace, _, glfw::Action::Press, _)
                | WindowEvent::Key(glfw::Key::Backspace, _, glfw::Action::Repeat, _) => {
                    input.push(InputEvent::Backspace);
                }

                WindowEvent::Key(glfw::Key::Enter, _, glfw::Action::Press, _) => {
                    input.push(InputEvent::Enter);
                }

                WindowEvent::Char(c) => {
                    input.push(InputEvent::Char(c));
                }

                WindowEvent::MouseButton(glfw::MouseButton::Left, glfw::Action::Press, _) => {
                    let (x, y) = self.window.get_cursor_pos();
                    input.push(InputEvent::Click(x as f32, y as f32));
                }

                WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height);
                },

                _ => {}
            }
        }

        input
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }
}
