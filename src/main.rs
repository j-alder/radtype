use piston::{WindowSettings, Position, Events, EventSettings, EventLoop, RenderEvent};
use piston_window::{*, types::Color};

fn draw_text(
    ctx: &Context,
    graphics: &mut G2d,
    glyphs: &mut Glyphs,
    color: Color,
    pos: Position,
    text: &str,
) {
    text::Text::new_color(color, 32)
        .draw(
            text, 
            glyphs, 
            &ctx.draw_state, 
            ctx.transform.trans(pos.x as f64, pos.y as f64), 
            graphics
        )
        .unwrap()
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "radical typing simulator", 
        [1000.0, 1000.0]
    )
        .exit_on_esc(true)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(0, 0)
        .for_folder("assets")
        .unwrap();

    let font = assets.join("FiraCode-Regular.ttf");

    let mut glyphs = window
        .load_font(font)
        .unwrap();


    let mut events = Events::new(EventSettings::new()).lazy(true);

    let mut display_text = String::from("Hello world!");

    while let Some(e) = events.next(&mut window) {
        if let Some(_args) = e.render_args() {
            window.draw_2d(&e, |ctx, g, d| {
                clear([1.0; 4], g);
                draw_text(&ctx, g, &mut glyphs, [0.0, 0.0, 0.0, 1.0], Position { x: 0, y: 500 }, &display_text.as_str());
                glyphs.factory.encoder.flush(d)
            });
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            display_text = key.code().to_string();
        }
    }
}
