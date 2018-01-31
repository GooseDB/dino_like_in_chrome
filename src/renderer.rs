use consts::*;
use camera::*;
use coord::*;
use sprite::*;

pub struct Renderer {
    render: Vec<char>,
    camera: Camera,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            render: vec![' '; (WIDTH * HEIGHT) as usize],
            camera: Camera::new(Coord::new(WIDTH, HEIGHT), Coord::new(0, 0)),
        }
    }
    pub fn clear(&mut self) {
        for i in &mut self.render {
            *i = ' ';
        }
    }
    pub fn put_sprite(&mut self, sprite: &Sprite, position: Coord) {
        for i in 0..sprite.get_size().x * sprite.get_size().y {
            let coords_origin = position - self.camera.get_border().min;

            let relative_coord = Coord::new(
                i % sprite.get_size().x,
                (i - i % sprite.get_size().x) / sprite.get_size().x,
            );

            let cell = coords_origin + relative_coord;

            if cell.x > -1 && cell.y > -1 && cell.x < WIDTH && cell.y < HEIGHT {
                self.render[(cell.y * WIDTH + cell.x) as usize] = sprite.get_pixel(i);
            }
        }
    }
    pub fn present(&self) {
        for _ in 0..WIDTH + 2 {
        print!("-");
        }
        println!("");
        for h in 0..HEIGHT {
            for w in 0..WIDTH {
                if w == 0 {
                    print!("|");
                }
                print!("{}", self.render[(h * WIDTH + w) as usize]);
                if w == WIDTH - 1 {
                    print!("|");
                }
            }
            println!("");
        }
        for _ in 0..WIDTH + 2 {
        print!("-")
        }
        println!("");
    }
}