use macroquad::prelude::*;
use crate::{cell::Cell, shape::Shape};

pub fn fill_board(board: &mut Vec<Cell>) {
    for y in (0..400).step_by(20) {
        for x in (0..400).step_by(20) {
            board.push(Cell {
                rect: Rect {
                    x: x as f32,
                    y: y as f32,
                    w: 20.0,
                    h: 20.0,
                },
                color: WHITE,
            });
        }
    }
}

pub fn update_board(board: &mut Vec<Cell>, shape: Shape) {
    board.iter_mut().for_each(|cell| {
        shape.points.iter().for_each(|rect| {
            if cell.rect.x == rect.x && cell.rect.y == rect.y {
                cell.color = shape.color;
            }
        });
    });
}

pub fn check_for_whole_line(row: &[Cell], color: &Color) -> bool {
    row.iter().all(|cell| cell.color == *color)
}