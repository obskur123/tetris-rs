use macroquad::prelude::*;
use rand::gen_range;
use crate::Cell;

#[derive(Clone, Copy)]
pub struct Shape {
    pub points: [Rect; 4],
    pub color: Color,
    pub landed: bool,
}

impl Shape {
    pub fn randomizer(quantity: i32) -> Vec<Self> {
        let possible_shapes = [
            [(10, 0), (10, 1), (9, 0), (9, 1)],
            [(10, 0), (10, 1), (10, 2), (10, 3)],
        ];

        let possible_colors = [
            GREEN,
            RED,
            BLUE
        ];

        let mut shapes: Vec<Shape> = Vec::with_capacity(quantity as usize);
        for _ in 0..quantity {
            let index_shape = gen_range(0, 2);
            let index_color = gen_range(0, 3);
            let shape_type = &possible_shapes[index_shape];
            let color = &possible_colors[index_color];
            shapes.push(Shape::new(*shape_type, *color));
        }

        shapes
    }

    pub fn new(points: [(i32, i32); 4], color: Color) -> Self {
        let mapped_points: Vec<Rect> = points
            .iter()
            .map(|x| Rect {
                x: (x.0 * 20) as f32,
                y: (x.1 * 20) as f32,
                w: 20.0,
                h: 20.0,
            })
            .collect();

        Shape {
            points: mapped_points.try_into().unwrap(),
            color,
            landed: false,
        }
    }

    pub fn compare_with_landed_shapes(
        &mut self,
        board: &Vec<Cell>,
        cb: fn(Rect, Rect) -> bool,
    ) -> bool {
        for p in self.points {
            for cell in board {
                if cb(p, cell.rect) && cell.color != WHITE
                /*green*/
                {
                    return false;
                }
            }
        }
        true
    }

    pub fn move_on_x(&mut self, board: &Vec<Cell>) {
        if !self.landed {
            match get_last_key_pressed() {
                Some(KeyCode::Right)
                    if self.compare_with_landed_shapes(board, |rect1, rect2| {
                        rect1.x + 20.0 == rect2.x && rect1.y == rect2.y
                    }) =>
                {
                    self.points.iter_mut().for_each(|rect| rect.x += 20.0)
                }
                Some(KeyCode::Left)
                    if self.compare_with_landed_shapes(board, |rect1, rect2| {
                        rect1.x - 20.0 == rect2.x && rect1.y == rect2.y
                    }) =>
                {
                    self.points.iter_mut().for_each(|rect| rect.x -= 20.0);
                }
                _ => (),
            }
        }
    }

    pub fn fall_update(&mut self) {
        let lowest_point = self.points.iter().max_by_key(|x| x.y as i32).unwrap();
        if lowest_point.y < 380.0 {
            self.points.iter_mut().for_each(|rect| rect.y += 20.0);
        } else {
            self.landed = true;
        }
    }

    pub fn is_collided(&mut self, board: &Vec<Cell>) -> bool {
        !self.compare_with_landed_shapes(board, |rect1, rect2| {
            rect1.bottom() >= rect2.top()
                && rect1.bottom() <= rect2.bottom()
                && rect1.right() > rect2.left()
                && rect1.left() < rect2.right()
        })
    }
}
