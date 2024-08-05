use std::io::{stdout, Write};

use crossterm::{execute, QueueableCommand, terminal};
use crossterm::cursor::{DisableBlinking, Hide, MoveTo};
use crossterm::style::Print;
use terminal_size::{Height, terminal_size, Width};
use tokio::time::Duration;
use winapi::shared::windef::POINT;
use winapi::um::winuser::GetCursorPos;
use winapi::um::winuser::GetSystemMetrics;

struct PointWrapper(POINT);

impl PartialEq for PointWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.x == other.0.x && self.0.y == other.0.y
    }
}

fn cursor_to_term_coord(
    window_x: u32,
    window_y: u32,
    cursor_x: u32,
    cursor_y: u32,
    term_x: u32,
    term_y: u32,
) -> (u32, u32) {
    let new_term_x = (cursor_x as f64 / window_x as f64) * term_x as f64;
    let new_term_y = (cursor_y as f64 / window_y as f64) * term_y as f64;
    (new_term_x as u32, new_term_y as u32)
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut wrapped_point = PointWrapper(POINT { x: 0, y: 0 });
    let size_check_interval = 10;
    let mut loop_count = 0;

    let mut terminal_size_x = 0;
    let mut terminal_size_y = 0;

    let window_size_x = 3839;
    let window_size_y = 2159;
    let window_size_x = unsafe { GetSystemMetrics(0) };
    let window_size_y = unsafe { GetSystemMetrics(1) };

    let char_radius_x = 5;
    let char_radius_y = 3;

    let mut stdout = stdout();

    execute!(stdout, DisableBlinking)?;
    execute!(stdout, Hide)?;

    loop {
        if loop_count % size_check_interval == 0 {
            let size = terminal_size();
            match size {
                Some((Width(w), Height(h))) => {
                    if terminal_size_x != w || terminal_size_y != h {
                        terminal_size_x = w;
                        terminal_size_y = h;
                    }
                }
                None => {
                    println!("Unable to get terminal size");
                }
            }
        }
        let mut point = POINT { x: 0, y: 0 };
        unsafe {
            GetCursorPos(&mut point);
        }

        if wrapped_point != PointWrapper(point) {
            let (term_x, term_y) = cursor_to_term_coord(window_size_x as u32, window_size_y as u32, point.x as u32, point.y as u32, terminal_size_x as u32, terminal_size_y as u32);
            execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
            for i in 0..=term_x {
                for j in 0..=term_y {
                    if (i as i32 - term_x as i32).abs() <= char_radius_x && (j as i32 - term_y as i32).abs() <= char_radius_y {
                        stdout
                            .queue(MoveTo(i as u16, j as u16))?
                            .queue(Print("a"))?;
                    } else {
                        stdout
                            .queue(MoveTo(i as u16, j as u16))?
                            .queue(Print(" "))?;
                    }
                }
            }
            stdout.flush()?;
        }
        wrapped_point = PointWrapper(point);


        loop_count += 1;

        tokio::time::sleep(Duration::from_millis(20)).await;
    }
}
