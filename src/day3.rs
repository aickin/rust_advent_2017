 use std::collections::HashMap;
 use std::slice::Iter;

enum Direction {
  East,
  North,
  West,
  South,
}

impl Direction {
  fn turn_left(&self) -> Direction {
    match *self {
      Direction::East => Direction::North,
      Direction::North => Direction::West,
      Direction::West => Direction::South,
      Direction::South => Direction::East,
    }
  }
}

#[derive(Debug)]
struct Point {
  x: i32,
  y: i32,
}

impl Point {
  fn go(&self, dir: &Direction, units: i32) -> Point {
    match *dir {
      Direction::East => Point { x: self.x + units, y: self.y },
      Direction::North => Point { x: self.x, y: self.y + units },
      Direction::West => Point { x: self.x - units, y: self.y },
      Direction::South => Point { x: self.x, y: self.y - units },
    }
  }

  fn distance(&self, other: &Point) -> i32 {
    (self.x - other.x).abs() + (self.y - other.y).abs()
  }

  fn clone(other: &Point) -> Point {
    Point { x: other.x, y: other.y }
  }

  fn surrounding_points(&self) -> SurroundingPointIter {
    SurroundingPointIter::new(self)
  }
}

struct SurroundingPointIter {
  points: Vec<Point>,
}

impl SurroundingPointIter {

  fn new(point: &Point) -> SurroundingPointIter {
    SurroundingPointIter {
      points: vec!(
        Point { x: point.x - 1, y: point.y - 1 },
        Point { x: point.x - 1, y: point.y },
        Point { x: point.x - 1, y: point.y + 1},
        Point { x: point.x , y: point.y + 1},
        Point { x: point.x, y: point.y - 1},
        Point { x: point.x + 1, y: point.y - 1},
        Point { x: point.x + 1, y: point.y },
        Point { x: point.x + 1, y: point.y + 1},
      )
    }
  }
}

impl Iterator for SurroundingPointIter {
  type Item = Point;

  fn next(&mut self) -> Option<Self::Item> {
    self.points.pop()
  }
}

struct Spiral {
  point: Point,
  dir: Direction,
  side_length: i32,
  travelled_since_last_turn: i32,
}

impl Spiral {
  fn new() -> Spiral {
    Spiral {
      point: Point { x: 0, y: 0 },
      dir: Direction::East,
      side_length: 1,
      travelled_since_last_turn: 0,
    }
  }
}

impl Iterator for Spiral {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
      let point = Point::clone(&self.point);

      self.point = point.go(&self.dir, 1);

      self.travelled_since_last_turn = self.travelled_since_last_turn + 1;

      if self.travelled_since_last_turn == self.side_length {
        self.dir = self.dir.turn_left();
        self.travelled_since_last_turn = 0;

        match self.dir {
          Direction::West|Direction::East => {
            self.side_length = self.side_length + 1;
          },
          _ => (),
        };
      }
      Some(point)
    }
}

struct Grid {
  values: HashMap<i32, HashMap<i32, i32>>,
}

impl Grid {
  fn new() -> Grid {
    Grid { values: HashMap::new() }
  }

  fn get(&self, point: &Point) -> Option<&i32> {
    match self.values.get(&point.x) {
      Some(map) => map.get(&point.y),
      None => None,
    }
  }

  fn set(&mut self, point: &Point, value: i32) -> () {
    let inner_map = self.values.entry(point.x).or_insert(HashMap::new());
    inner_map.insert(point.y, value);
  }
}

pub fn puzzle1() -> i32 {

  let destination = 347991;

  let mut spiral = Spiral::new();
  let mut point : Option<Point> = None;
  for _ in 0..destination {
      point = spiral.next();
  }
  return point.unwrap().distance(&Point { x: 0, y: 0 });
}

pub fn puzzle2() -> i32 {
  let mut spiral = Spiral::new();
  let mut grid = Grid::new();

  grid.set( &Point { x: 0, y: 0 }, 1);
  spiral.next();

  let mut last_value = 0;
  while last_value < 347991 {
    let point = spiral.next().unwrap();

    last_value = point.surrounding_points().fold(0, |total, point| {
      match grid.get(&point) {
        Some(val) => total + val,
        None => total,
      }
    });

    grid.set(&point, last_value);
  }

  last_value
}
