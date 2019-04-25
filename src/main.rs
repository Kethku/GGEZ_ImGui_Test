mod imgui_wrapper;

use ggez::*;

use imgui_wrapper::ImGuiWrapper;

struct State {
    pos_x: f32,
    imgui_wrapper: ImGuiWrapper,
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            mint::Point2{x: self.pos_x, y: 330.0},
            100.0,
            1.0,
            graphics::WHITE)?;
        graphics::draw(ctx, &circle, graphics::DrawParam::default())?;

        // self.imgui_wrapper.render(ctx);

        graphics::present(ctx)
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        x: f32,
        y: f32,
        _xrel: f32,
        _yrel: f32,
    ) {
        self.imgui_wrapper.update_mouse_pos(x, y);
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: input::mouse::MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.imgui_wrapper.update_mouse_down((
            button == input::mouse::MouseButton::Left,
            button == input::mouse::MouseButton::Right,
            button == input::mouse::MouseButton::Middle,
        ));
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: input::mouse::MouseButton, _x: f32, _y: f32) {
        self.imgui_wrapper.update_mouse_down((
            match button {
                input::mouse::MouseButton::Left => false,
                _ => true,
            },
            match button {
                input::mouse::MouseButton::Right => false,
                _ => true,
            },
            match button {
                input::mouse::MouseButton::Middle => false,
                _ => true,
            },
        ));
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: input::keyboard::KeyCode,
        _keymod: input::keyboard::KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            input::keyboard::KeyCode::P => {
                self.imgui_wrapper.open_popup();
            }
            _ => (),
        }
    }
}

pub fn main() {
    let c = conf::Conf::new();

    let(ref mut ctx, ref mut event_loop) = ContextBuilder::new("ggez_imgui_test", "Keith Simmons")
        .conf(c)
        .build()
        .unwrap();

    let state = &mut State {
        pos_x: 0.0,
        imgui_wrapper: ImGuiWrapper::new(ctx) };

    event::run(ctx, event_loop, state).unwrap();
}
