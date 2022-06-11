pub const SIGHT_RADIUS: f32 = 20.0;
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
    ) {
        if distance >= SIGHT_RADIUS || min >= max {
            return;
        }

        let mut i = (distance * min).ceil();

        while i <= distance * max {
            let x = center.0 + rotate(distance, i).0;
            let y = center.1 + rotate(distance, i).1;

            if self.get(x as isize, y as isize) == BLOCK {
                self.scan_arc(center, distance + 1.0, min, (i - 0.5) / distance, rotate);
                min = (i + 0.5) / distance;
            } else {
                self.set(x as isize, y as isize, VISIBLE);
            }

            i += 1.0;
        }

        self.scan_arc(center, distance + 1.0, min, max, rotate);
    }

    pub fn full_scan(&mut self, center: (f32, f32)) {
        self.scan_arc(center, 0.0, -1.0, 1.0, &|x, y| (x, y));
        self.scan_arc(center, 0.0, -1.0, 1.0, &|x, y| (y, -x));
        self.scan_arc(center, 0.0, -1.0, 1.0, &|x, y| (-x, -y));
        self.scan_arc(center, 0.0, -1.0, 1.0, &|x, y| (-y, x));
    }
}

#[cfg(test)]
mod tests {
    use crate::ShadowMap;
    use crate::BLOCK;
    use crate::PLAYER;

    #[test]
    fn test_raycast() {
        let mut map = ShadowMap::new_with_empty_cells(16, 17);

        let obstacles = vec![
            (3, 3),
            (4, 3),
            (5, 3),
            (8, 7),
            (8, 5),
            (6, 9),
            (6, 10),
            (6, 11),
            (7, 11),
            (7, 10),
            (7, 9),
        ];

        for obstacle in obstacles {
            map.set(obstacle.0, obstacle.1, BLOCK);
        }

        let center = (4.0, 5.0);
        map.full_scan(center);

        map.set(4, 5, PLAYER);
        map.show();

        let expected_map = vec![
            [2, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2],
            [2, 2, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2],
            [2, 2, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2],
            [2, 2, 2, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
            [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0],
            [2, 2, 2, 2, 3, 2, 2, 2, 1, 0, 0, 0, 0, 0, 0, 0],
            [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0],
            [2, 2, 2, 2, 2, 2, 2, 2, 1, 0, 2, 2, 2, 2, 2, 2],
            [2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 2, 2, 2, 2],
            [2, 2, 2, 2, 2, 2, 1, 1, 2, 2, 2, 0, 0, 0, 0, 2],
            [2, 2, 2, 2, 2, 2, 1, 1, 0, 2, 2, 2, 2, 0, 0, 0],
            [2, 2, 2, 2, 2, 2, 1, 1, 0, 0, 2, 2, 2, 2, 0, 0],
            [2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2],
            [2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 2, 2, 2, 2, 2],
            [2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 2, 2, 2, 2],
            [2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 2, 2, 2],
            [2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 2, 2],
        ];

        for y in 0..expected_map.len() {
            for x in 0..expected_map[y].len() {
                assert_eq!(map.get(x as isize, y as isize), expected_map[y][x]);
            }
        }
    }
}
