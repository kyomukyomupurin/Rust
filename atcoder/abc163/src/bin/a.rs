fn main() {
    proconio::input! {
        r: f64
    }
    let ans = 2.0 * r * std::f64::consts::PI;
    println!("{}", ans);
}
