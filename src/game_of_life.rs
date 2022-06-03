use std::collections::HashSet;

pub type Point = (i32, i32);

#[derive(Debug)]
pub struct GameOfLife {
	pub alive_tiles: HashSet<(i32, i32)>,
}

impl GameOfLife {
	pub fn new() -> GameOfLife {
		GameOfLife {
			alive_tiles: HashSet::new(),
		}
	}

	pub fn get_tile(&self, point: Point) -> bool {
		self.alive_tiles.contains(&point)
	}

	pub fn toggle_tile(&mut self, point: Point) {
		if self.alive_tiles.contains(&point) {
			self.alive_tiles.remove(&point);
		} else {
			self.alive_tiles.insert(point);
		}
	}

	fn iter_neighbours(&self, (x, y): Point) -> Vec<Point> {
		(x-1..=x+1)
			.flat_map(move |x| (y-1..=y+1).map(move |y| (x, y)))
			.filter(|point| !(point.0 == x && point.1 == y))
			.collect::<Vec<_>>()
	}

	fn count_neighbours(&self, point: Point) -> u8 {
		self.iter_neighbours(point).iter()
			.filter(|point| self.alive_tiles.contains(point))
			.count() as u8
	}

	fn get_dead_tiles(&self) -> HashSet<Point> {
		self.alive_tiles.iter()
			.flat_map(|point| self.iter_neighbours(*point))
			.filter(|point| !self.alive_tiles.contains(point))
			.collect::<HashSet<_>>()
	}

	pub fn tick(&mut self) {
		let mut new_alive_tiles = HashSet::new();

		for alive_tile in &self.alive_tiles {
			let neighours = self.count_neighbours(*alive_tile);
			if neighours == 2 || neighours == 3 {
				new_alive_tiles.insert(*alive_tile);
			}
		}

		for dead_tile in self.get_dead_tiles() {
			let neighours = self.count_neighbours(dead_tile);
			if neighours == 3 {
				new_alive_tiles.insert(dead_tile);
			}
		}

		self.alive_tiles = new_alive_tiles;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		let mut game = GameOfLife::new();

		game.toggle_tile((3, 3));
		game.toggle_tile((4, 3));
		game.toggle_tile((4, 4));
		game.toggle_tile((3, 4));

		assert_eq!(game.count_neighbours((3, 3)), 3);
	}
}
