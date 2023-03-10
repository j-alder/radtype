use opengl_graphics::{OpenGL, GlGraphics};
use piston::{WindowSettings, RenderEvent, Button, PressEvent, UpdateEvent};
use piston::event_loop::{EventSettings, Events};
use glutin_window::GlutinWindow as Window;


const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0; 4];

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("radtype", [200, 200])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

            gl.draw(args.viewport(), |ctx, gl| {
                graphics::clear(WHITE, gl);
                graphics::ellipse(
                    GREEN, 
                    graphics::ellipse::circle(x, y, 5.0),
                    ctx.transform,
                    gl
                );
            })
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            println!("Pressed key '{:?}'", key)
        }

        if let Some(_args) = e.update_args() {
            // create letter borders here
        }
    }
}
