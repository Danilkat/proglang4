
#[derive(Clone)]
struct Point {
  x: f64,
  y: f64,
}
#[derive(Clone)]
struct Line {
  p1: Point,
  p2: Point,
}

impl Copy for Point {}
impl Copy for Line {}

impl std::fmt::Display for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{},{}", self.x, self.y)
  }
}

impl std::fmt::Display for Line {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{} {}", self.p1, self.p2)
  }
}

fn line_from_input_str(input: &str) -> Line {
  let mut input = input.split_whitespace();
  let p1 = input.next().unwrap();
  let p2 = input.next().unwrap();
  let p1 = point_from_input_str(p1);
  let p2 = point_from_input_str(p2);
  Line { p1, p2 }
}

fn point_from_input_str(input: &str) -> Point {
  let mut input = input.split(',');
  let x = input.next().unwrap().parse::<f64>().unwrap();
  let y = input.next().unwrap().parse::<f64>().unwrap();
  Point { x, y }
}

fn line_coef(line: Line) -> (f64, f64, f64) {
  let a = line.p2.y - line.p1.y;
  let b = line.p1.x - line.p2.x;
  let c = a * (line.p1.x) + b * (line.p1.y);
  (a, b, c)
}

fn det(a: f64, b: f64, c: f64, d: f64) -> f64 {
  a * d - b * c
}

fn half_line_intersect(l1: Line, l2: Line) -> Option<Point> {

  let (a1, b1, c1) = line_coef(l1);

  let (a2, b2, c2) = line_coef(l2);

  let determinant = det(a1, b1, a2, b2);

  if determinant == 0.0 {
    return None;
  } else {

    let x = det(c1, b1, c2, b2) / determinant;
    let y = det(a1, c1, a2, c2) / determinant;

    let point = Point { x, y };
    if point_on_half_line(l1, point) {
      return Some(point);
    } else {
      return None;
    }
  }
}

fn point_on_line_segment(line: Line, point: Point) -> bool {
  let x1 = line.p1.x;
  let y1 = line.p1.y;
  let x2 = line.p2.x;
  let y2 = line.p2.y;
  let x3 = point.x;
  let y3 = point.y;

  
  let a = (x3 >= x1) &&
   (x3 <= x2);

  let b = (x3 >= x2) &&
   (x3 <= x1);

  let c = (y3 >= y1) &&
   (y3 <= y2);

  let d = (y3 >= y2) &&
   (y3 <= y1);

  (a || b) && (c || d)
}

fn point_on_half_line(line: Line, point: Point) -> bool {
  let x1 = line.p1.x;
  let y1 = line.p1.y;
  let x2 = line.p2.x;
  let y2 = line.p2.y;
  let x3 = point.x;
  let y3 = point.y;

  let a = ((x3 > x1) == (x2 > x1)) &&
   ((y3 > y1) == (y2 > y1)) &&
    point_on_line_segment(line, point);
  a

}

fn distance_between_points(p1: Point, p2: Point) -> f64 {
  let x1 = p1.x;
  let y1 = p1.y;
  let x2 = p2.x;
  let y2 = p2.y;
  ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

fn main() {

  //Читаем входные данные
  let mut lines = Vec::new();
  loop {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim().is_empty() {
      break;
    }
    let line = line_from_input_str(&input);
    lines.push(line);
  }

  //Убираем и сохраняем луч из массива
  let half_line = lines.swap_remove(0);

  //Находим точки пересечения луча с остальными отрезками
  let mut intersect_points = Vec::new();
  for line in lines.iter() {
    if let Some(point) = half_line_intersect(half_line, *line) {
      intersect_points.push(point);
    }
  }
  
  if intersect_points.is_empty() {
    println!("");
    return;
  }

  //Находим ближайшую к началу луча точку пересечения
  let mut smallest_distance = f64::MAX;
  let mut closest_point = Point { x: 0.0, y: 0.0 };
  for point in intersect_points {
    let distance = distance_between_points(half_line.p1, point);
    if distance < smallest_distance {
      smallest_distance = distance;
      closest_point = point;
    }
  }

  //Находим отрезок, на котором находится ближайшая точка пересечения
  let mut closest_line = Line {
    p1: Point { x: 0.0, y: 0.0 },
    p2: Point { x: 0.0, y: 0.0 },
  };
  for line in lines.iter() {
    if point_on_line_segment(*line, closest_point) {
      closest_line = *line;
      break;
    }
  }

  println!("{}", closest_line);

}
