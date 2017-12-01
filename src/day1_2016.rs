use std::ops::Range;

#[derive(Debug, Copy, Clone)]
enum CardinalDirection {
    North,
    West,
    South,
    East
}

impl CardinalDirection {
    fn turn(self, td: TurnDirection) -> CardinalDirection {
        match td {
            TurnDirection::Right => {
                match self {
                    CardinalDirection::North => CardinalDirection::East,
                    CardinalDirection::West => CardinalDirection::North,
                    CardinalDirection::South => CardinalDirection::West,
                    CardinalDirection::East => CardinalDirection::South,
                }
            },
            TurnDirection::Left => {
                match self {
                    CardinalDirection::North => CardinalDirection::West,
                    CardinalDirection::West => CardinalDirection::South,
                    CardinalDirection::South => CardinalDirection::East,
                    CardinalDirection::East => CardinalDirection::North,
                }
            }
        }
    }
}

#[derive(Debug)]
enum TurnDirection {
    Right,
    Left
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    direction: CardinalDirection
}

impl Position {
    fn grid_distance(&self, other: &Position) -> i32 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }

    fn go(self, units: i32) -> Position {
        match self.direction {
          CardinalDirection::North => Position { x: self.x, y: self.y + units, direction: self.direction },
          CardinalDirection::West => Position { x: self.x - units, y: self.y, direction: self.direction },
          CardinalDirection::South => Position { x: self.x, y: self.y - units, direction: self.direction },
          CardinalDirection::East => Position { x: self.x + units, y: self.y, direction: self.direction },
        }
    }

    fn path(&self, units: i32) -> Vec<Position> {
      let mut positions : Vec<Position> = vec![];
      for i in (Range { start: 1, end: units + 1 }) {
        positions.push(
          match self.direction {
            CardinalDirection::North => Position { x: self.x, y: self.y + i, direction: self.direction },
            CardinalDirection::West => Position { x: self.x - i, y: self.y, direction: self.direction },
            CardinalDirection::South => Position { x: self.x, y: self.y - i, direction: self.direction },
            CardinalDirection::East => Position { x: self.x + i, y: self.y, direction: self.direction },
          }
        )
      }
      positions
    }

    fn turn(self, td: TurnDirection) -> Position {
      Position { x: self.x, y: self.y, direction: self.direction.turn(td)}
    }

    fn is_same_point(&self, other: &Position) -> bool {
      self.x == other.x && self.y == other.y
    }
}

pub fn puzzle1() -> i32 {
    let data = "L5, R1, L5, L1, R5, R1, R1, L4, L1, L3, R2, R4, L4, L1, L1, R2, R4, R3, L1, R4, L4, L5, L4, R4, L5, R1, R5, L2, R1, R3, L2, L4, L4, R1, L192, R5, R1, R4, L5, L4, R5, L1, L1, R48, R5, R5, L2, R4, R4, R1, R3, L1, L4, L5, R1, L4, L2, L5, R5, L2, R74, R4, L1, R188, R5, L4, L2, R5, R2, L4, R4, R3, R3, R2, R1, L3, L2, L5, L5, L2, L1, R1, R5, R4, L3, R5, L1, L3, R4, L1, L3, L2, R1, R3, R2, R5, L3, L1, L1, R5, L4, L5, R5, R2, L5, R2, L1, L5, L3, L5, L5, L1, R1, L4, L3, L1, R2, R5, L1, L3, R4, R5, L4, L1, R5, L1, R5, R5, R5, R2, R1, R2, L5, L5, L5, R4, L5, L4, L4, R5, L2, R1, R5, L1, L5, R4, L3, R4, L2, R3, R3, R3, L2, L2, L2, L1, L4, R3, L4, L2, R2, R5, L1, R2";

    let mut pos = Position { x: 0, y: 0, direction: CardinalDirection:: North };

    for instruction in data.split(", ") {
      let td = match &instruction[..1] {
        "L" => TurnDirection::Left,
        "R" => TurnDirection::Right,
        _ => panic!("Instruction started with something other than L or R"),
      };

      let units: i32 = instruction[1..].parse().unwrap();

      pos = pos.turn(td).go(units);
    }
    (Position { x: 0, y: 0, direction: CardinalDirection:: North }).grid_distance(&pos)
}

pub fn puzzle2() -> i32 {
  let data = "L5, R1, L5, L1, R5, R1, R1, L4, L1, L3, R2, R4, L4, L1, L1, R2, R4, R3, L1, R4, L4, L5, L4, R4, L5, R1, R5, L2, R1, R3, L2, L4, L4, R1, L192, R5, R1, R4, L5, L4, R5, L1, L1, R48, R5, R5, L2, R4, R4, R1, R3, L1, L4, L5, R1, L4, L2, L5, R5, L2, R74, R4, L1, R188, R5, L4, L2, R5, R2, L4, R4, R3, R3, R2, R1, L3, L2, L5, L5, L2, L1, R1, R5, R4, L3, R5, L1, L3, R4, L1, L3, L2, R1, R3, R2, R5, L3, L1, L1, R5, L4, L5, R5, R2, L5, R2, L1, L5, L3, L5, L5, L1, R1, L4, L3, L1, R2, R5, L1, L3, R4, R5, L4, L1, R5, L1, R5, R5, R5, R2, R1, R2, L5, L5, L5, R4, L5, L4, L4, R5, L2, R1, R5, L1, L5, R4, L3, R4, L2, R3, R3, R3, L2, L2, L2, L1, L4, R3, L4, L2, R2, R5, L1, R2";

  let mut pos = Position { x: 0, y: 0, direction: CardinalDirection:: North };

  let mut positions : Vec<Position> = vec![];

  for instruction in data.split(", ") {
    let td = match &instruction[..1] {
      "L" => TurnDirection::Left,
      "R" => TurnDirection::Right,
      _ => panic!("Instruction started with something other than L or R"),
    };

    let units: i32 = instruction[1..].parse().unwrap();

    pos = pos.turn(td);
    positions.extend(pos.path(units));
    pos = pos.go(units);
  }

  // now find the first repeated x/y position in the list.

  for (i, i_position) in positions.iter().enumerate() {
    for (_, j_position) in positions.iter().take(i).enumerate() {
      if i_position.is_same_point(j_position) {
        return (Position { x: 0, y: 0, direction: CardinalDirection:: North }).grid_distance(&i_position);
      }
    }
  }
//  for i in (Range {start: 0, end: positions.len()}) {
//    for j in (Range {start: 0, end: i}) {
//      let foo : String = i;
//      let iPos : &Position = positions[1];
//      if (*positions.get(i).unwrap()).is_same_point(positions.get(j).unwrap()) {
//        answer = positions.get(i).unwrap();
//      }
//    }
//  }
//  (Position { x: 0, y: 0, direction: CardinalDirection:: North }).grid_distance(&answer)
0
}
