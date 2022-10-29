use crate::Sprite;

pub enum GridMode {
    CellSize(cgmath::Vector2<u32>),
    CellCount(cgmath::Vector2<usize>),
}

pub trait SpriteExt {
    fn uniform_grid(&self, grid_mode: GridMode) -> Vec<Sprite>;
}

impl SpriteExt for Sprite {
    fn uniform_grid(&self, grid_mode: GridMode) -> Vec<Sprite> {
        use GridMode::*;
        let cell_count = match grid_mode {
            CellSize(size) => (
                (self.get_size().x / size.x) as usize,
                (self.get_size().y / size.y) as usize,
            )
                .into(),
            CellCount(count) => count,
        };
        let cell_size = match grid_mode {
            CellSize(size) => size,
            CellCount(count) => (
                self.get_size().x / count.x as u32,
                self.get_size().y / count.y as u32,
            )
                .into(),
        };
        let mut cells = vec![];
        for y in 0..cell_count.y {
            for x in 0..cell_count.x {
                let offset = (x as u32 * cell_size.x, y as u32 * cell_size.y).into();
                let slice = self.slice(cell_size, offset);
                cells.push(slice)
            }
        }
        cells
    }
}
