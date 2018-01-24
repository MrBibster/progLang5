use std::io;
use std::mem;
#[derive(Clone)]
struct Point
{
    x: f64,
    y: f64
}
#[derive(Clone)]
struct Seg
{
    s: Point,
    e: Point
}
#[derive(Clone)]
struct Line
{
    a: f64,
    b: f64,
    c: f64
}
static EPS: f64 = 0.00001;
fn read_segment() -> Option<Seg>
{
    let mut buf = String::new();
    let res = io::stdin().read_line(&mut buf);
    match res {
        Ok(n) => {
            if n == 0 {
                return None;
            }
            let mut iter = buf.split(' ');
            let mut iter2 = iter.next().unwrap().split(',');
            let mut x1 = iter2.next().unwrap().trim().parse::<f64>().expect("invalid input");
            let mut y1 = iter2.next().unwrap().trim().parse::<f64>().expect("invalid input");
            iter2 = iter.next().unwrap().split(',');
            let mut x2 = iter2.next().unwrap().trim().parse::<f64>().expect("invalid input");
            let mut y2 = iter2.next().unwrap().trim().parse::<f64>().expect("invalid input");
            if x1 > x2 {
                mem::swap(&mut x1, &mut x2);
                mem::swap(&mut y1, &mut y2);
            }
            let s = Point{ x: x1, y: y1 };
            let e = Point{ x: x2, y: y2 };
            return Some(Seg{ s: s, e: e});
        }
        Err(_) => return None
    }
}
fn RoS(l: f64, m: f64, r: f64) -> bool
{
    return l.min(r) <= m && m <= r.max(l);
}
fn distance(a: &Point, b: &Point) -> f64
{
    return ((a.x-b.x)*(a.x-b.x)+(a.y-b.y)*(a.y-b.y)).sqrt();
}
fn best<'t>(a: &'t Point, b: &'t Point, c: &'t Point) -> &'t Point
{
    if distance(a, b) < distance(a, c)
    {
        return b;
    }
    else
    {
        return c;
    }
}
fn comp(a: &Seg, b: &Seg) -> Option<Point>
{
    let l1 = linify(a);
    let l2 = linify(b);
    let dn = l1.a * l2.b - l2.a * l1.b;
    if dn.abs() < EPS  {
        if (l1.c-l2.c).abs() < EPS {
            if RoS(b.s.x, a.s.x, b.e.x)
                && RoS(b.s.y, a.s.y, b.e.y) {
                return Some(a.s.clone());
            }
            if (a.s.x-a.e.x)*(a.s.x-b.s.x) >= 0. && (a.s.y-a.e.y)*(a.s.y-b.s.y) >= 0.
            {
                return Some(best(&a.s, &b.s, &b.e).clone());
            }
        }
    }
    else {
        let x = (l1.b * l2.c - l2.b * l1.c)/dn;
        let y = (l2.a * l1.c - l1.a * l2.c)/dn;
        if (a.s.x-a.e.x)*(a.s.x-x) >= 0. && (a.s.y-a.e.y)*(a.s.y-y) >= 0. && RoS(b.s.x, x, b.e.x) && RoS(b.s.y, y, b.e.y)
        {
            return Some(Point{ x: x, y: y });
        }
    }
    return None;
}
fn linify(f: &Seg) -> Line
{
    let a=f.s.y-f.e.y;
    let b=f.e.x-f.s.x;
    let c=f.s.x*f.e.y-f.e.x*f.s.y;
    let vec=(a*a+b*b).sqrt();
    return Line{ a: a/vec, b: b/vec, c: c/vec};
}
fn main()
{
    let luch = read_segment().unwrap();
    let mut colpoint: Option<Point> = None;
    loop {
        match read_segment() {
            Some(seg) => {
                match comp(&luch, &seg) {
                    Some(s) => {
                        match ans {
                            Some(cur) => colpoint = Some(best(&luch.s, &cur, &s).clone()),
                            None => colpoint = Some(s)
                        }
                    }
                    None => {}
                }
            }
            None => break
        }
    }
    match colpoint {
        Some(s) => {
            println!("{} {}", s.x, s.y);
        }
        None => {"Точек пересечения не существует"}
    }
}
