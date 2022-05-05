extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sdl2_window;

use graphics::rectangle::square;
use graphics::rectangle::Rectangle;
use graphics::Transformed;
use opengl_graphics::*;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use sdl2_window::Sdl2Window as Window;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;
use std::ops::Not;
#[derive(Clone, Eq, PartialEq)]
enum Life {
    Alive,
    Dead,
}
#[derive(Clone, Eq)]
struct Cords {
    x: i32,
    y: i32,
}
impl fmt::Display for Life {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Life::Alive => write!(f, "Alive"),
            Life::Dead => write!(f, "Dead"),
        }
    }
}
impl Not for Life {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Life::Alive => return Life::Dead,
            Life::Dead => return Life::Alive,
        }
    }
}
impl Ord for Cords {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.y > other.y || (self.y == other.y && self.x > other.x) {
            return Ordering::Greater;
        } else if self.y < other.y || (self.y == other.y && self.x < other.x) {
            return Ordering::Less;
        } else {
            return Ordering::Equal;
        }
    }
}
impl PartialOrd for Cords {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cords {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
fn main() {
    // Iterate over everything.
    let cell_size = 5 as usize;
    let mut is_paused = false;
    let mut mouse_coords = [0.0; 2];
    let mut grid = BTreeMap::new();
    let alive = [
        (0, 0),
        (1, 0),
        (2, 0),
        /*(3, 0), 
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (9, 0),
        (10, 0),
        (11, 0),
        (12, 0),
        (13, 0),
        (17, 0),
        (18, 0),
        (19, 0),
        (26, 0),
        (27, 0),
        (28, 0),
        (29, 0),
        (30, 0),
        (31, 0),
        (32, 0),
        (34, 0),
        (35, 0),
        (36, 0),
        (37, 0),
        (38, 0), */
                 (4, 0),
                 (0, 1),
                 (3, 2),
                 (4, 2),
                 (1, 3),
                 (2, 3),
                 (4, 3),
                 (0, 4),
                 (2, 4),
                 (4, 4),
    ];
    set_alive(&alive, &mut grid);
    let alive_rect = Rectangle::new([0.0, 1.0, 0.0, 1.0]);
    let dead_rect = Rectangle::new([0.0, 0.20, 0.0, 1.0]);
    // create window
    let mut window: Window = WindowSettings::new("Conway's Game of life", [1920, 1080])
        .exit_on_esc(true)
        .fullscreen(true)
        .build()
        .unwrap();

    // create OpenGL context
    let mut look_y = 400.0;
    let mut look_x = 400.0;
    let mut gl = GlGraphics::new(OpenGL::V3_2);
    // main loop
    let mut events = Events::new(EventSettings::new());
    let mut grid_copy = grid.clone();
    while let Some(event) = events.next(&mut window) {
        if let Some(m_coords) = event.mouse_cursor_args() {
            mouse_coords = m_coords;
        };
        if let Some(press_args) = event.press_args() {
            if let Button::Keyboard(Key::Space) = press_args {
                is_paused = !is_paused;
            };
            if let Button::Keyboard(Key::Up) = press_args {
                look_y = look_y + 20.0;
            };
            if let Button::Keyboard(Key::Down) = press_args {
                look_y = look_y - 20.0;
            };
            if let Button::Keyboard(Key::Left) = press_args {
                look_x = look_x + 20.0;
            };
            if let Button::Keyboard(Key::Right) = press_args {
                look_x = look_x - 20.0;
            };
            if let Button::Mouse(MouseButton::Left) = press_args {
                let mouse_coords_as_cords = Cords {
                    x: ((mouse_coords[0] - look_x) as usize / cell_size) as i32,
                    y: ((mouse_coords[1] - look_y) as usize / cell_size) as i32,
                };
                match grid.get(&mouse_coords_as_cords) {
                    Some(Life::Alive) => grid_copy.insert(mouse_coords_as_cords, Life::Dead),
                    _ => {insert_neighbours(&mut grid_copy, &mouse_coords_as_cords);
                            grid_copy.insert(mouse_coords_as_cords, Life::Alive)},
                };
            };
        };

        // handle render eventsgetArea
        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |ctx, gl| {
                let transform = ctx.transform.trans(look_x, look_y);
                graphics::clear([0.0, 0.0, 0.0, 0.0], gl);
                //----------

                for (cords, alive) in &grid {
                    let is_alive = alive == &Life::Alive;
                    if !is_paused {
                        let count = get_area(&grid, &cords) - if is_alive { 1 } else { 0 };

                        if (count == 2 && is_alive) || count == 3 {
                            insert_neighbours(&mut grid_copy, cords);
                            &grid_copy.insert(cords.clone(), Life::Alive);
                        } else if get_area(&grid_copy, &cords) == 0 {
                            &grid_copy.remove(cords);
                        }  else {
                            &grid_copy.insert(cords.clone(), Life::Dead);
                        }
                    }
                    if is_alive {
                        let dims = square(
                            (cords.x * cell_size as i32) as f64,
                            (cords.y * cell_size as i32) as f64,
                            (cell_size) as f64,
                        );
                        alive_rect.draw(dims, &ctx.draw_state, transform, gl);
                    } else {
                          let dims = square(
                            (cords.x * cell_size as i32) as f64,
                            (cords.y * cell_size as i32) as f64,
                              cell_size as f64,
                          );
                          dead_rect.draw(dims, &ctx.draw_state, transform, gl);
                      } 
                }
                grid = grid_copy.clone();
                //println!("\n\n\n");
            });
        }
    }
}
fn get_area(grid: &BTreeMap<Cords, Life>, cords: &Cords) -> i32 {
    let x = cords.x;
    let y = cords.y;
    let mut sum = 0;
    for i in -1..2 {
        for j in -1..2 {
            if let Some(&Life::Alive) = grid.get(&Cords { x: x + j, y: y + i }) {
                sum = sum + 1;
            }
        }
    }
    return sum;
}
fn set_alive(arr: &[(i32, i32)], grid: &mut BTreeMap<Cords, Life>) {
    for a in arr {
        grid.insert(Cords { x: a.0, y: a.1 }, Life::Alive);
    }
    for a in arr {
        insert_neighbours(grid, &Cords { x: a.0, y: a.1 });
    }
}
fn insert_neighbours(new_set: &mut BTreeMap<Cords, Life>, cords: &Cords) {
    for i in -1..2 {
        for j in -1..2 {
            let moved_cords = Cords {
                x: cords.x + j,
                y: cords.y + i,
            };
            match new_set.get(&moved_cords) {
                Some(x) => new_set.insert(moved_cords, x.clone()),
                None => new_set.insert(moved_cords, Life::Dead),
            };
        }
    }
}
