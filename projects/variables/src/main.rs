fn main() {
    let uns_32n: u32 = 32;
    let sig_32n: i32 = -32;
    println!("unsigned 32 bit number is {uns_32n}, signed 32 bit number is {sig_32n}");
    let float_32f: f32 = 4.323232323;
    let float_64f: f64 = -9.3289389283928938928938; 
    println!("32 bit float is {float_32f}, 64 bit float is {float_64f}");
    let a = [1, 2, 3, 4, 5];
    let months: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sept", "Oct", "Nov", "Dec"];
    let a_from_index: [i32; 5] = [a[0], a[1], a[2], a[3], a[4]];
    let five_threes = [3; 5];
    println!("{a:?}, {months:?}, {a_from_index:?}, {five_threes:?}");
}
