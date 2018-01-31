mod consts;
mod coord;
mod camera;
mod renderer;
mod aabb;
mod sprite;

use renderer::*;
use sprite::*;
use coord::*;

fn main() {
    let mut r = Renderer::new();
    let s = Sprite::new(vec![   
                ' ',' ',' ',' ',' ',' ',' ','@','@','@','@','@','@',
                ' ',' ',' ',' ',' ',' ',' ','@','@','@','@','@','@',
                '@',' ',' ',' ',' ','@','@','@','@','@','@',' ',' ',
                '@','@',' ',' ','@','@','@','@','@','@','@','~','~',
                ' ','@','@','@','@','@','@','@','@','@',' ',' ',' ',
                ' ',' ','@','@','@','@','@','@',' ',' ',' ',' ',' ',
                ' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',
                ' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',],Coord::new(13,8));
    let d = Sprite::new(vec![
                '@',' ',' ',' ',' ','@','@','@','@','@','@',' ','@','@','@','@','@','@',
                '@','@',' ',' ','@','@','@','@','@','@','@',' ','@','@','@','@','@','@',
                ' ','@','@','@','@','@','@','@','@','@','@','@','@','@',' ',' ',' ',' ',
                ' ',' ',' ','@','@','@','@','@','@',' ',' ','S',' ',' ',' ',' ',' ',' ',
                ' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',
                ' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',
    ], Coord::new(18,6));
    r.present();
    r.put_sprite(&s, Coord::new(15,0));
    r.present();
    r.clear();
    r.present();
    r.put_sprite(&d, Coord::new(15,0));
    r.present();
}
