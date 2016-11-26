use cairo::surface::Surface;
use cairo::Cairo;
use cairo::surface::format::Format;
use cubestate::CubeState;

const WIDTH: i32 = 500;
const HEIGHT: i32 = 250;
const STICKER_SIZE: f64 = 40.;
const HALF_STICKER_SIZE: f64 = STICKER_SIZE / 2.;

const FWIDTH: f64 = WIDTH as f64;
const FHEIGHT: f64 = HEIGHT as f64;
const VANISH_AMOUNT: f64 = 200.;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Side {
    U, R, L, D
}

impl Side {
    fn vanishing_point(&self) -> (f64, f64) {
        match self {
            &Side::U => (FWIDTH / 2., FHEIGHT / 2. - VANISH_AMOUNT),
            &Side::R => (FWIDTH / 2. + VANISH_AMOUNT, FHEIGHT / 2.),
            &Side::L => (FWIDTH / 2. - VANISH_AMOUNT, FHEIGHT / 2.),
            &Side::D => (FWIDTH / 2., FHEIGHT / 2. + VANISH_AMOUNT),
        }
    }

    fn offset(&self) -> (f64, f64) {
        match self {
            &Side::U => (1., 0.),
            &Side::R => (0., 1.),
            &Side::L => (0., 1.),
            &Side::D => (1., 0.),
        }
    }

    fn xoff(&self) -> f64 {
        match self {
            &Side::U => 0.,
            &Side::R => 1.,
            &Side::L => -1.,
            &Side::D => 0.,
        }
    }

    fn yoff(&self) -> f64 {
        match self {
            &Side::U => -1.,
            &Side::R => 0.,
            &Side::L => 0.,
            &Side::D => 1.,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Sticker {
    Square { x: f64, y: f64 },
    Vanishing {
        x: f64,
        y: f64,
        vanish_side: Side
    },
}

const VANISH_STICKER_LEN: f64 = 30.;

fn project_to((x1, y1): (f64, f64), (x2, y2): (f64, f64), side: Side, len: f64) -> (f64, f64) {
    let xx = x1 - x2;
    let yy = y1 - y2;
    let dist = (xx*xx + yy*yy).sqrt();
    let mut x = x1 + side.xoff() * VANISH_STICKER_LEN;
    let mut y = y1 + side.yoff() * VANISH_STICKER_LEN;

    match side {
        Side::U | Side::D => {
            x -= (x1 - x2) * len / dist;
            y += side.xoff() * (y1 - y2) * len / dist;
        },
        Side::R | Side::L => {
            x += side.yoff() * (x1 - x2) * len / dist;
            y -= (y1 - y2) * len / dist;
        },
    }

    (x, y)
}

fn draw_sticker(cr: &mut ::cairo::Cairo, s: &Sticker, (r, g, b): (f64, f64, f64)) {
    match s {
        &Sticker::Square { x, y } => {
            cr.new_path();
            cr.line_to(x - HALF_STICKER_SIZE, y - HALF_STICKER_SIZE);
            cr.line_to(x - HALF_STICKER_SIZE, y + HALF_STICKER_SIZE);
            cr.line_to(x + HALF_STICKER_SIZE, y + HALF_STICKER_SIZE);
            cr.line_to(x + HALF_STICKER_SIZE, y - HALF_STICKER_SIZE);
            cr.close_path();
            cr.set_source_rgba(r, g, b, 1.);
            cr.fill_preserve();
            cr.set_source_rgba(0., 0., 0., 1.);
            cr.stroke();
        },
        &Sticker::Vanishing { x, y, vanish_side: ref side } => {
            let (offx, offy) = side.offset();
            let p2 = (x + offx * STICKER_SIZE / 2., y + offy * STICKER_SIZE / 2.);
            let p3 = (x - offx * STICKER_SIZE / 2., y - offy * STICKER_SIZE / 2.);

            let vanish = side.vanishing_point();
            let p1 = project_to(p2, vanish, *side, VANISH_STICKER_LEN);
            let p4 = project_to(p3, vanish, *side, VANISH_STICKER_LEN);

            let (p1x, p1y) = p1;
            let (p2x, p2y) = p2;
            let (p3x, p3y) = p3;
            let (p4x, p4y) = p4;

            cr.new_path();
            cr.line_to(p1x, p1y);
            cr.line_to(p2x, p2y);
            cr.line_to(p3x, p3y);
            cr.line_to(p4x, p4y);
            cr.close_path();
            cr.set_source_rgba(r, g, b, 1.);
            cr.fill_preserve();
            cr.set_source_rgba(0., 0., 0., 1.);
            cr.stroke();
        }
    }
}

enum Color {
    Yellow,
    White,
    Green,
    Blue,
    Orange,
    Red,
}

fn sticker_color(i: u8) -> Color {
    match i {
        0  | 1  | 2  | 3  | 4  | 5  | 6  | 7  | 8  => Color::Yellow,
        9  | 10 | 11 | 18 | 19 | 20 | 27 | 28 | 29 => Color::Red,
        12 | 13 | 14 | 21 | 22 | 23 | 30 | 31 | 32 => Color::Green,
        15 | 16 | 17 | 24 | 25 | 26 | 33 | 34 | 35 => Color::Orange,
        36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 => Color::White,
        45 | 46 | 47 | 48 | 49 | 50 | 51 | 52 | 53 => Color::Blue,
        _ => panic!("Don't have a color for {}", i)
    }
}

impl Color {
    fn rgb(&self) -> (f64, f64, f64) {
        match self {
            &Color::Yellow => (1.,  1., 0.),
            &Color::White  => (1.,  1., 1.),
            &Color::Green  => (0.,  1., 0.),
            &Color::Blue   => (0.,  0., 1.),
            &Color::Red    => (1.,  0., 0.),
            &Color::Orange => (1., 0.5, 0.),
        }
    }
}

const SQUARE_SPACING: f64 = 4.;

pub fn generate_image(c: CubeState, output_filename: &str) {
    // this is inside this function because on the rust 1.15 nightly I get a segfault otherwise
    let stickers: [(usize, Sticker); 21] = [
        (51, Sticker::Vanishing {
            x: (WIDTH / 2) as f64 - 43.5,
            y: (HEIGHT / 2) as f64 - 65.,
            vanish_side: Side::U
        }),
        (52, Sticker::Vanishing {
            x: (WIDTH / 2) as f64,
            y: (HEIGHT / 2) as f64 - 65.,
            vanish_side: Side::U
        }),
        (53, Sticker::Vanishing {
            x: (WIDTH / 2) as f64 + 43.5,
            y: (HEIGHT / 2) as f64 - 65.,
            vanish_side: Side::U
        }),
        (12, Sticker::Vanishing {
            x: (WIDTH / 2) as f64 - 43.5,
            y: (HEIGHT / 2) as f64 + 65.,
            vanish_side: Side::D
        }),
        (13, Sticker::Vanishing {
            x: (WIDTH / 2) as f64,
            y: (HEIGHT / 2) as f64 + 65.,
            vanish_side: Side::D
        }),
        (14, Sticker::Vanishing {
            x: (WIDTH / 2) as f64 + 43.5,
            y: (HEIGHT / 2) as f64 + 65.,
            vanish_side: Side::D
        }),
        (9, Sticker::Vanishing {
            x: (WIDTH / 2) as f64 - 65.,
            y: (HEIGHT / 2) as f64 - 43.5,
            vanish_side: Side::L
        }),
        (10, Sticker::Vanishing {
            x: (WIDTH / 2) as f64 - 65.,
            y: (HEIGHT / 2) as f64,
            vanish_side: Side::L
        }),
        (11, Sticker::Vanishing {
            x: (WIDTH / 2) as f64 - 65.,
            y: (HEIGHT / 2) as f64 + 43.5,
            vanish_side: Side::L
        }),
        (17, Sticker::Vanishing {
            x: (WIDTH / 2) as f64 + 65.,
            y: (HEIGHT / 2) as f64 - 43.5,
            vanish_side: Side::R
        }),
        (16, Sticker::Vanishing {
            x: (WIDTH / 2) as f64 + 65.,
            y: (HEIGHT / 2) as f64,
            vanish_side: Side::R
        }),
        (15, Sticker::Vanishing {
            x: (WIDTH / 2) as f64 + 65.,
            y: (HEIGHT / 2) as f64 + 43.5,
            vanish_side: Side::R
        }),
        (0, Sticker::Square {
            x: (WIDTH / 2) as f64 - STICKER_SIZE - SQUARE_SPACING,
            y: (HEIGHT / 2) as f64 - STICKER_SIZE - SQUARE_SPACING,
        }),
        (1, Sticker::Square {
            x: (WIDTH / 2) as f64,
            y: (HEIGHT / 2) as f64 - STICKER_SIZE - SQUARE_SPACING,
        }),
        (2, Sticker::Square {
            x: (WIDTH / 2) as f64 + STICKER_SIZE + SQUARE_SPACING,
            y: (HEIGHT / 2) as f64 - STICKER_SIZE - SQUARE_SPACING,
        }),
        (3, Sticker::Square {
            x: (WIDTH / 2) as f64 - STICKER_SIZE - SQUARE_SPACING,
            y: (HEIGHT / 2) as f64,
        }),
        (4, Sticker::Square {
            x: (WIDTH / 2) as f64,
            y: (HEIGHT / 2) as f64,
        }),
        (5, Sticker::Square {
            x: (WIDTH / 2) as f64 + STICKER_SIZE + SQUARE_SPACING,
            y: (HEIGHT / 2) as f64,
        }),
        (6, Sticker::Square {
            x: (WIDTH / 2) as f64 - STICKER_SIZE - SQUARE_SPACING,
            y: (HEIGHT / 2) as f64 + STICKER_SIZE + SQUARE_SPACING,
        }),
        (7, Sticker::Square {
            x: (WIDTH / 2) as f64,
            y: (HEIGHT / 2) as f64 + STICKER_SIZE + SQUARE_SPACING,
        }),
        (8, Sticker::Square {
            x: (WIDTH / 2) as f64 + STICKER_SIZE + SQUARE_SPACING,
            y: (HEIGHT / 2) as f64 + STICKER_SIZE + SQUARE_SPACING,
        }),
    ];

    let mut surface = Surface::create_image(Format::ARGB32, WIDTH, HEIGHT);
    let mut cr = Cairo::create(&mut surface);

    cr.set_line_width(4.);

    for &r in stickers.to_vec().iter() {
        let (i, s) = r;
        draw_sticker(&mut cr, &s, sticker_color(c.state[i]).rgb());
    }

    surface.write_to_png(output_filename);
}
