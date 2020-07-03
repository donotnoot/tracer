mod tuple;
mod matrix;

fn main() {
    let p = tuple::point(1.0, 3.0, 4.0);
    let v = tuple::vector(0.9, 0.3, 0.1);

    println!("{} {}", p, v);

    println!("{}", &p + &v);
    println!("{}", &p - &v);
    println!("{}", p == v);
    println!("{}", p == v);
    println!("{}", -&p)
}
