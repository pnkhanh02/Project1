extern crate piston_window;
extern crate rand;

mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;

use crate::game::Game;
use crate::draw::to_coord_u32;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];



fn main() {
    //Kich thuoc cua so tro choi
    let (width, height) = (30, 30);

    //khoi tao cua so tro choi
    let mut window: PistonWindow =
        WindowSettings::new("Snake Game", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    //tao cau truc game voi chieu dai va chieu rong
    let mut game = Game::new(width, height);

    //xu ly cac su kien trong cua so tro choi(nhan phim, cap nhat)
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        //xoa cua so tro choi bang mau nen, goi phuong thuc draw de ve tro choi
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        //cap nhat trang thai tro choi voi thoi gian da troi qua ke tu lan cap nhat cuoi cung
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
