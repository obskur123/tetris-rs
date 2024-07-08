use crate::cell::Cell;
use board::{check_for_whole_line, fill_board, update_board};
use macroquad::prelude::*;
use shape::Shape;
mod board;
mod cell;
mod shape;

#[macroquad::main(window_conf)]
async fn main() {
    let mut index = 0;

    let mut board: Vec<Cell> = Vec::with_capacity(400);

    let shapes = Shape::randomizer(256);

    let step = 20;

    let move_interval = 10;

    let mut frame = 0;

    let mut shape = shapes[index];

    fill_board(&mut board);

    Shape::randomizer(300);

    loop {
        frame += 1;

        clear_background(WHITE);

        shape.move_on_x(&board);

        if frame >= move_interval {
            frame = 0;
            shape.fall_update();
        }

        for p in shape.points {
            draw_rectangle(p.x, p.y, p.w, p.h, shape.color);
        }

        if shape.is_collided(&board) {
            shape.landed = true;
        }

        if shape.landed {
            update_board(&mut board, shape.clone());
            index += 1;
            shape = shapes[index];
        }

        let mut lines_to_remove: Vec<i32> = Vec::new();

        for (index, chunk) in board.chunks(20).enumerate() {
            if check_for_whole_line(chunk, &chunk[0].color) {
                lines_to_remove.push(index as i32);
            }
        }

        for line in lines_to_remove {
            let y_index: usize = line as usize * 20;

            board.drain(y_index..y_index + 20); //saca un rango de posiciones de un vec!!

            for x in (0..400).step_by(20) {
                board.insert(
                    0,
                    Cell {
                        rect: Rect {
                            x: x as f32,
                            y: 0.0,
                            w: 20.0,
                            h: 20.0,
                        },
                        color: WHITE,
                    },
                );
            } //le inserto una linea por linea sacada...

            for (i, cell) in board.iter_mut().enumerate() {
                //division de enteros y luego multiplica por 20
                // de 0 a 19 -> 0/20 = 0, 1/20 = 0, etc...
                // 20/20 = 1
                cell.rect.y = (i / 20 * 20) as f32;
            }
        }

        board.iter().for_each(|cell| {
            if cell.color != WHITE {
                draw_rectangle(cell.rect.x, cell.rect.y, 20.0, 20.0, cell.color);
            }
        });

        draw_grid(step);

        next_frame().await
    }
}

fn draw_grid(step: i32) {
    for n in (0..400).step_by(step as usize) {
        draw_line(n as f32, 0.0, n as f32, 400.0, 1.0, BLACK);
    }

    for n in (0..400).step_by(step as usize) {
        draw_line(0.0, n as f32, 400.0, n as f32, 1.0, BLACK);
    }
}

fn window_conf() -> Conf {
    Conf {
        fullscreen: false,
        window_title: "tetris".to_string(),
        window_width: 400,
        window_height: 400,
        window_resizable: false,
        ..Default::default()
    }
}
