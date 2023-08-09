use design_patterns::strings;

fn main() {
    strings::coercion();
    let n = String::from("hello");
    let p = strings::P { n: "hello" };
    let f = strings::F { n };
    p.g();
    f.g();
    println!("{}", p.n);
    //now a move error
    // println!("{n}");
}
