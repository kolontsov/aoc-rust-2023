#[derive(Debug, PartialEq, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}
impl Point {
    fn from_csv(input: &str) -> Point {
        let mut iter = input.split(",");
        let x = iter.next().unwrap().trim().parse::<f64>().unwrap();
        let y = iter.next().unwrap().trim().parse::<f64>().unwrap();
        let z = iter.next().unwrap().trim().parse::<f64>().unwrap();
        Point { x, y, z }
    }
}
type Hail = (Point, Point);

fn parse_input(input: &str) -> Vec<(Point, Point)> {
    input.lines().map(|line| {
        let (pos_str, vel_str) = line.split_once(" @ ").unwrap();
        let pos = Point::from_csv(pos_str);
        let vel = Point::from_csv(vel_str);
        (pos, vel)
    }).collect()
}

fn det2d(a: Point, b: Point) -> f64 {
    a.x * b.y - a.y * b.x
}

fn get_intersect(a: Hail, b: Hail) -> Option<Point> {
    let (a_pos, a_vel) = a;
    let (b_pos, b_vel) = b;
    let delta_pos = Point {
        x: b_pos.x - a_pos.x,
        y: b_pos.y - a_pos.y, 
        z: 0.0
    };
    let det = det2d(a_vel, b_vel);
    if det == 0.0 {
        return None;
    }
    let t_a = det2d(delta_pos, b_vel) / det;
    let t_b = det2d(delta_pos, a_vel) / det;
    if t_a < 0.0 || t_b < 0.0 {
        return None;
    }
    Some(Point {
        x: a_pos.x + t_a * a_vel.x,
        y: a_pos.y + t_a * a_vel.y,
        z: 0.0
    })
}

pub fn part1(input: String) -> u64 {
    let hails = parse_input(&input);
    let is_test = hails.len() < 10;
    let min: f64 = if is_test { 7.0 } else { 200000000000000.0 };
    let max: f64 = if is_test { 27.0 } else { 400000000000000.0 };
    let mut cnt = 0;
    for i in 0..hails.len() {
        for j in i+1..hails.len() {
            if let Some(intersect) = get_intersect(hails[i], hails[j]) {
                if intersect.x < min || intersect.x > max { continue; }
                if intersect.y < min || intersect.y > max { continue; }
                cnt += 1;
            }
        }
    }
    cnt as u64
}

pub fn part2(input: String) -> u64 {
    let hails = parse_input(&input);
    // print matlab code (yes, I'm lazy)
    let vars = "x y z vx vy vz t0 t1 t2";
    println!("syms {}", vars);
    println!("eqs = [");
    for i in 0..3 {
        let (pos, vel) = hails[i];
        println!("{} + {} * t{} == t{} * vx + x,", pos.x, vel.x, i, i);
        println!("{} + {} * t{} == t{} * vy + y,", pos.y, vel.y, i, i);
        println!("{} + {} * t{} == t{} * vz + z,", pos.z, vel.z, i, i);
    }
    println!("];");
    println!("S = solve(eqs, [{}]);", vars);
    println!("S.x + S.y + S.z");
    0
}