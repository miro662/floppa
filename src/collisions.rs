use cgmath::Vector2;

#[derive(Debug, Copy, Clone)]
pub struct Bounds {
    pub position: Vector2<i32>,
    pub size: Vector2<i32>,
}

impl Bounds {
    pub fn from_min_max(min: Vector2<i32>, max: Vector2<i32>) -> Bounds {
        Bounds {
            position: min,
            size: max - min,
        }
    }
    pub fn min(&self) -> Vector2<i32> {
        self.position
    }

    pub fn max(&self) -> Vector2<i32> {
        self.position + self.size
    }

    pub fn overlap(&self, other: &Bounds) -> Option<Bounds> {
        fn overlap_axis(a_min: i32, a_max: i32, b_min: i32, b_max: i32) -> Option<(i32, i32)> {
            let a_len = a_max - a_min;
            let b_len = b_max - b_min;
            let total_min = a_min.min(b_min);
            let total_max = a_max.max(b_max);
            let total_len = total_max - total_min;
            let difference = (a_len + b_len) - total_len;
            if difference >= 0 {
                let left_max = a_min.max(b_min);
                Some((left_max, left_max + difference))
            } else {
                None
            }
        }
        let x = overlap_axis(self.min().x, self.max().x, other.min().x, other.max().x)?;
        let y = overlap_axis(self.min().y, self.max().y, other.min().y, other.max().y)?;
        Some(Bounds::from_min_max((x.0, y.0).into(), (x.1, y.1).into()))
    }
}
