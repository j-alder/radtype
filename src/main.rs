use piston::{WindowSettings, Position, Events, EventSettings, RenderEvent};
use piston_window::{*, types::Color, rectangle::square};

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

fn key_to_color(k: Key) -> Color {
    match k.code() {
        0x61=>[0.1, 1.0, 0.0, 1.0],
        0x62=>[0.2, 0.9, 0.1, 1.0],
        0x63=>[0.3, 0.8, 0.0, 1.0],
        0x64=>[0.4, 0.7, 0.2, 1.0],
        0x65=>[0.5, 0.6, 0.0, 1.0],
        0x66=>[0.6, 0.5, 0.3, 1.0],
        0x67=>[0.7, 0.4, 0.0, 1.0],
        0x68=>[0.8, 0.3, 0.4, 1.0],
        0x69=>[0.9, 0.2, 0.0, 1.0],
        0x6A=>[1.0, 0.1, 0.5, 1.0],
        0x6B=>[0.1, 0.0, 0.0, 1.0],
        0x6C=>[0.2, 1.0, 0.6, 1.0],
        0x6D=>[0.3, 0.9, 0.0, 1.0],
        0x6E=>[0.4, 0.8, 0.7, 1.0],
        0x6F=>[0.5, 0.7, 0.0, 1.0],
        0x70=>[0.6, 0.6, 0.8, 1.0],
        0x71=>[0.7, 0.5, 0.0, 1.0],
        0x72=>[0.8, 0.4, 0.9, 1.0],
        0x73=>[0.9, 0.3, 0.0, 1.0],
        0x74=>[1.0, 0.2, 1.0, 1.0],
        0x75=>[0.1, 0.1, 0.0, 1.0],
        0x76=>[0.2, 0.0, 0.1, 1.0],
        0x77=>[0.3, 1.0, 0.0, 1.0],
        0x78=>[0.4, 0.9, 0.2, 1.0],
        0x79=>[0.5, 0.8, 0.0, 1.0],
        0x7A=>[0.6, 0.7, 0.3, 1.0],
        _=>[0.0, 0.0, 0.0, 0.0]
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

struct ColoredSquare {
    color: Color,
    size: f64,
}

impl ColoredSquare {
    fn default() -> Self {
        ColoredSquare { color: [1.0; 4], size: 0.0 }
    }
}

struct Game {
    window: PistonWindow,
    glyphs: Glyphs,
    text: String,
    shapes: Vec<ColoredSquare>,
    background_color: Color,
}

const WINDOW_HEIGHT: f64 = 1000.0;
const WINDOW_WIDTH: f64 = 1000.0;

impl Game {
    fn default() -> Self {
        let mut window: PistonWindow = WindowSettings::new(
            "radical typing simulator",
            [WINDOW_WIDTH, WINDOW_HEIGHT]
        ).exit_on_esc(true).build().unwrap();
        let assets = find_folder::Search::ParentsThenKids(0, 0)
            .for_folder("assets")
            .unwrap();

        let font = assets.join("FiraCode-Regular.ttf");

        let glyphs = window
            .load_font(font)
            .unwrap();
        
        Game { 
            window, 
            glyphs, 
            text: String::from(""),
            shapes: Vec::new(),
            background_color: [1.0; 4]
        }
    }

    fn render(&mut self, e: &Event) {
        self.window.draw_2d(e, |ctx, g, d| {
            let mut s: Option<ColoredSquare> = None;
            while self.shapes.len() > 0 && self.shapes[0].size >= WINDOW_HEIGHT {
                s = self.shapes.pop();
                self.background_color = s.unwrap_or(ColoredSquare::default()).color
            }
            for shape in &self.shapes {
                rectangle(
                    shape.color, 
                    square(
                        (WINDOW_WIDTH / 2.0) - (shape.size / 2.0),
                        (WINDOW_HEIGHT / 2.0) - (shape.size / 2.0),
                        shape.size
                    ), 
                    ctx.transform,
                    g
                )
            }
            clear(self.background_color, g);
            draw_text(
                &ctx, 
                g, 
                &mut self.glyphs, 
                [0.0, 0.0, 0.0, 1.0], 
                Position { x: 500, y: 500 }, 
                self.text.as_str()
            );
            self.glyphs.factory.encoder.flush(d);
        });
    }

    fn update_text(&mut self, key: Key) {
        let t = self.text.clone();
        self.text = String::from(key_to_string(key, &self.text));
        if t != self.text {
            self.shapes.push(ColoredSquare { color: key_to_color(key), size: 20.0 });
        }
    }

    fn update_square_sizes(&mut self) {
        for shape in &mut self.shapes {
            shape.size += 2.0
        }
    }
}

fn main() {
    let mut game = Game::default();

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut game.window) {
        if let Some(_args) = e.render_args() {
            game.render(&e);
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            game.update_text(key);
        }
        if let Some(_args) = e.update_args() {
            game.update_square_sizes()
        }
    }
}
