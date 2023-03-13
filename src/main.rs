use piston::{WindowSettings, Position, Events, EventSettings, EventLoop, RenderEvent};
use piston_window::{*, types::Color};

fn key_to_string(k: Key, fallback: &str) -> &str {
    match k.code(){
        0x61=>"A",
        0x62=>"B",
        0x63=>"C",
        0x64=>"D",
        0x65=>"E",
        0x66=>"F",
        0x67=>"G",
        0x68=>"H",
        0x69=>"I",
        0x6A=>"J",
        0x6B=>"K",
        0x6C=>"L",
        0x6D=>"M",
        0x6E=>"N",
        0x6F=>"O",
        0x70=>"P",
        0x71=>"Q",
        0x72=>"R",
        0x73=>"S",
        0x74=>"T",
        0x75=>"U",
        0x76=>"V",
        0x77=>"W",
        0x78=>"X",
        0x79=>"Y",
        0x7A=>"Z",
        _=>fallback
    }
}

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

    let mut display_text = "";

    while let Some(e) = events.next(&mut window) {
        if let Some(_args) = e.render_args() {
            window.draw_2d(&e, |ctx, g, d| {
                clear([1.0; 4], g);
                draw_text(
                    &ctx, 
                    g, 
                    &mut glyphs, 
                    [0.0, 0.0, 0.0, 1.0], 
                    Position { x: 500, y: 500 }, 
                    display_text);
                glyphs.factory.encoder.flush(d)
            });
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            display_text = key_to_string(key, &display_text);
        }
    }
}
