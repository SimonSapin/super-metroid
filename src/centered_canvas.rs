pub struct CenteredCanvas {
    pub width: u16,
    pub height: u16,
    zero: u16,
    pub buffer: Vec<u8>,
}

impl CenteredCanvas {
    pub fn new(width: u16, height: u16, zero: (u16, u16)) -> Self {
        CenteredCanvas {
            width: width,
            height: height,
            zero: zero.1 * width + zero.0,
            buffer: vec![0; width as usize * height as usize],
        }
    }

    fn offset(&self, x: i16, y: i16) -> usize {
        let mut offset = self.zero as usize;
        if x >= 0 {
            offset += x as usize;
        } else {
            offset -= (-x) as usize;
        }
        if y >= 0 {
            offset += y as usize * self.width as usize;
        } else {
            offset -= ((-y) as usize) * self.width as usize;
        }
        offset
    }

    fn _paint_tile(&mut self, tile: &[u8], offset: usize) {
        let mut index = offset;
        for row in tile.chunks(8) {
            for (n, px) in row.iter().enumerate() {
                if *px == 0 {
                    continue;
                }
                self.buffer[index + n] = *px;
            }
            index += self.width as usize;
        }
    }

    pub fn paint_tile(&mut self, tile: &[u8], x: i16, y: i16) {
        let offset = self.offset(x, y);
        self._paint_tile(&tile, offset);
    }

    pub fn paint_block(&mut self, tile0: &[u8], tile1: &[u8], tile2: &[u8], tile3: &[u8], x: i16, y: i16) {
        let width = self.width as usize;
        let offset = self.offset(x, y);
        self._paint_tile(&tile0, offset);
        self._paint_tile(&tile1, offset + 8);
        self._paint_tile(&tile2, offset + width*8);
        self._paint_tile(&tile3, offset + width*8 + 8);
    }
}
