use crate::Sprite;
use std::str::SplitInclusive;

pub enum GridMode {
    CellSize(cgmath::Vector2<u32>),
    CellCount(cgmath::Vector2<usize>),
}

pub trait SpriteExt {
    fn uniform_grid(&self, grid_mode: GridMode) -> Vec<Sprite>;
    fn non_uniform_grid(&self, rows: &[u32], cols: &[u32]) -> Vec<Sprite>;
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

    fn non_uniform_grid(&self, rows: &[u32], cols: &[u32]) -> Vec<Sprite> {
        let mut cells = vec![];
        let mut offset = (0, 0).into();
        for row_size in rows {
            for col_size in cols {
                let cell_size = (*col_size, *row_size).into();
                let slice = self.slice(cell_size, offset);
                cells.push(slice);
                offset.x += col_size;
            }
            offset.x = 0;
            offset.y += row_size;
        }
        cells
    }
}
