mod tests;

pub const EMPTY: u8 = 0;
pub const BLOCK: u8 = 1;
pub const VISIBLE: u8 = 2;
pub const PLAYER: u8 = 3;

#[derive(Debug)]
pub struct ShadowMap {
    map: Vec<Vec<u8>>,
}

impl ShadowMap {
    pub fn new_with_empty_cells(width: usize, height: usize) -> Self {
        let mut outer_vec = Vec::with_capacity(height);
        let mut inner_vec = Vec::with_capacity(width);

        for _ in 0..width {
            inner_vec.push(EMPTY);
        }

        for _ in 0..height {
            outer_vec.push(inner_vec.clone());
        }

        #[cfg(test)]
        assert_eq!(outer_vec.len(), height);

        #[cfg(test)]
        assert_eq!(inner_vec.len(), width);

        ShadowMap { map: outer_vec }
    }

    pub fn new_with_obstacles(width: usize, height: usize) -> Self {
        let mut outer_vec = Vec::with_capacity(height);
        let mut inner_vec = Vec::with_capacity(width);

        for _ in 0..width {
            inner_vec.push(BLOCK);
        }

        for _ in 0..height {
            outer_vec.push(inner_vec.clone());
        }

        #[cfg(test)]
        assert_eq!(outer_vec.len(), height);

        #[cfg(test)]
        assert_eq!(inner_vec.len(), width);

        ShadowMap { map: outer_vec }
    }

    pub fn get(&self, x: isize, y: isize) -> u8 {
        if x < 0 || y < 0 || x as usize >= self.map[0].len() || y as usize >= self.map.len() {
            return EMPTY;
        }

        return self.map[y as usize][x as usize];
    }

    pub fn set(&mut self, x: isize, y: isize, value: u8) {
        if x >= 0 && y >= 0 && (x as usize) < self.map[0].len() && (y as usize) < self.map.len() {
            self.map[y as usize][x as usize] = value;
        }
    }

    pub fn show(&self) {
        println!("");

        for row in &self.map {
            println!("{:?}", row);
        }
    }

    pub fn scan_arc(
        &mut self,
        center: (f32, f32),
        distance: f32,
        mut min: f32,
        max: f32,
        rotate: &dyn Fn(f32, f32) -> (f32, f32),
        sight_radius: f32,
    ) {
        if distance > sight_radius || min >= max {
            return;
        }

        let mut i = (distance * min).ceil();

        while i <= distance * max {
            let x = center.0 + rotate(distance, i).0;
            let y = center.1 + rotate(distance, i).1;

            if self.get(x as isize, y as isize) == BLOCK {
                self.scan_arc(
                    center,
                    distance + 1.0,
                    min,
                    (i - 0.5) / distance,
                    rotate,
                    sight_radius,
                );
                min = (i + 0.5) / distance;
            } else {
                self.set(x as isize, y as isize, VISIBLE);
            }

            i += 1.0;
        }

        self.scan_arc(center, distance + 1.0, min, max, rotate, sight_radius);
    }

    pub fn full_scan(&mut self, center: (f32, f32), sight_radius: f32) {
        self.scan_arc(center, 0.0, -1.0, 1.0, &|x, y| (x, y), sight_radius);
        self.scan_arc(center, 0.0, -1.0, 1.0, &|x, y| (y, -x), sight_radius);
        self.scan_arc(center, 0.0, -1.0, 1.0, &|x, y| (-x, -y), sight_radius);
        self.scan_arc(center, 0.0, -1.0, 1.0, &|x, y| (-y, x), sight_radius);
    }
}
