use piston::{WindowSettings, Position};
use piston_window::{PistonWindow, G2d, Context, Glyphs, types::Color, text, clear, Transformed};

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

    while let Some(e) = window.next() {
        window.draw_2d(&e, |ctx, g, d| {
            clear([1.0; 4], g);
            draw_text(&ctx, g, &mut glyphs, [0.0, 0.0, 0.0, 1.0], Position { x: 0, y: 500 }, "Hello World");
            glyphs.factory.encoder.flush(d)
        });
    }
}
