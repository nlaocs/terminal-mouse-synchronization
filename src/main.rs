use winapi::um::winuser::GetCursorPos;
use winapi::shared::windef::POINT;
use tokio::time::Duration;
use terminal_size::{Width, Height, terminal_size};

struct PointWrapper(POINT);

impl PartialEq for PointWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.x == other.0.x && self.0.y == other.0.y
    }
}

#[tokio::main]
async fn main() {
    let mut wrapped_point = PointWrapper(POINT { x: 0, y: 0 });
    let size_check_interval = 100;
    let mut loop_count = 0;
    loop {
        if loop_count % size_check_interval == 0 {
            let size = terminal_size();
            match size {
                Some((Width(w), Height(h))) => {
                    println!("Terminal size: {}x{}", w, h);
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
            println!("Cursor position: ({}, {})", point.x, point.y);
        }

        wrapped_point = PointWrapper(point);

        loop_count += 1;

        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}
