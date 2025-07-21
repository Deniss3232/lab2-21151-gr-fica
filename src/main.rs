const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const WHITE: u8 = 255;
const BLACK: u8 = 0;

type Framebuffer = [[u8; WIDTH]; HEIGHT];

fn point(x: usize, y: usize, color: u8, framebuffer: &mut Framebuffer) {
    if x < WIDTH && y < HEIGHT {
        framebuffer[y][x] = color;
    }
}

fn get_color(x: usize, y: usize, framebuffer: &Framebuffer) -> u8 {
    if x < WIDTH && y < HEIGHT {
        framebuffer[y][x]
    } else {
        BLACK
    }
}

fn count_live_neighbors(x: usize, y: usize, fb: &Framebuffer) -> usize {
    let mut count = 0;
    for dy in [-1i32, 0, 1] {
        for dx in [-1i32, 0, 1] {
            if dx == 0 && dy == 0 { continue; }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && ny >= 0 && nx < WIDTH as i32 && ny < HEIGHT as i32 {
                if get_color(nx as usize, ny as usize, fb) == WHITE {
                    count += 1;
                }
            }
        }
    }
    count
}

fn update_framebuffer(current: &Framebuffer, next: &mut Framebuffer) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let live_neighbors = count_live_neighbors(x, y, current);
            let cell = get_color(x, y, current);
            let next_state = match (cell == WHITE, live_neighbors) {
                (true, 2) | (true, 3) => WHITE,
                (false, 3) => WHITE,
                _ => BLACK,
            };
            point(x, y, next_state, next);
        }
    }
}

fn draw_initial_pattern(fb: &mut Framebuffer) {
    let patterns = [
        // 1. Glider
        (vec![(1,0), (2,1), (0,2), (1,2), (2,2)], 5, 5),
        // 2. Blinker
        (vec![(0,1), (1,1), (2,1)], 20, 20),
        // 3. Block
        (vec![(0,0), (0,1), (1,0), (1,1)], 40, 10),
        // 4. Toad
        (vec![(1,0), (2,0), (3,0), (0,1), (1,1), (2,1)], 60, 5),
        // 5. Beacon
        (vec![(0,0), (0,1), (1,0), (2,3), (3,2), (3,3)], 70, 20),
        // 6. Boat
        (vec![(0,0), (1,0), (0,1), (2,1), (1,2)], 10, 40),
        // 7. Loaf
        (vec![(1,0), (2,0), (0,1), (3,1), (1,2), (3,2), (2,3)], 25, 50),
        // 8. Tub
        (vec![(1,0), (0,1), (2,1), (1,2)], 45, 40),
        // 9. Pulsar (centro parcial)
        (vec![(2,0), (3,0), (4,0), (0,2), (5,2), (0,3), (5,3), (0,4), (5,4), (2,5), (3,5), (4,5)], 65, 40),
        // 10. LWSS
        (vec![(1,0), (4,0), (0,1), (0,2), (4,2), (0,3), (1,3), (2,3), (3,3)], 75, 60),
    ];

    for (pattern, dx, dy) in patterns {
        for (x, y) in pattern {
            point(x + dx, y + dy, WHITE, fb);
        }
    }
}

fn print_framebuffer(fb: &Framebuffer) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let pixel = if fb[y][x] == WHITE { '■' } else { ' ' };
            print!("{}", pixel);
        }
        println!();
    }
}

fn main() {
    let mut current = [[BLACK; WIDTH]; HEIGHT];
    let mut next = [[BLACK; WIDTH]; HEIGHT];

    draw_initial_pattern(&mut current);

    for _frame in 0..100 {
        print!("\x1B[2J\x1B[1;1H");
        print_framebuffer(&current);
        update_framebuffer(&current, &mut next);
        std::mem::swap(&mut current, &mut next);
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}
