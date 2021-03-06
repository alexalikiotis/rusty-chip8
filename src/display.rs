pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

pub struct Display {
    pub buffer: [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    pub buffer_update: bool,
}

impl Display {
    pub fn new() -> Display {
        Display {
            buffer: [0; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            buffer_update: false,
        }
    }

    pub fn clear(&mut self) {
        self.buffer = [0; DISPLAY_WIDTH * DISPLAY_HEIGHT];
    }

    pub fn draw(&mut self, x: u8, y: u8, memory: &[u8]) -> bool {
        let mut collision = false;
        self.buffer_update = true;

        for (block_offset, block) in memory.iter().enumerate() {
            let pos_y = (y as usize + block_offset) % DISPLAY_HEIGHT;
            for pixel_offset in 0..8 {
                // get buffer index from x and y coordinates
                let pos_x = (x as usize + pixel_offset) % DISPLAY_WIDTH;
                let idx = self.from_coordinates(pos_x, pos_y);

                let buffer_pixel = self.buffer[idx];
                let sprite_pixel = (block >> (7 - pixel_offset)) & 1;

                // xor the buffer pixel with the sprite pixel
                let pixel = buffer_pixel ^ sprite_pixel;
                self.buffer[idx] = pixel;

                // check if collision detected
                if sprite_pixel == 1 && pixel == 0 {
                    collision = true
                }
            }
        }
        collision
    }

    pub fn from_coordinates(&self, x: usize, y: usize) -> usize {
        (x + DISPLAY_WIDTH * y) as usize
    }

    pub fn should_update(&self) -> bool {
        self.buffer_update
    }
}
