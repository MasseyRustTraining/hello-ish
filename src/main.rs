fn print_and_sum(a: &mut [f32]) -> f32 {
    let mut sum = 0.0;
    a[0] = 3.0;
    for x in a {
        sum += *x;
    }
    println!("{}", sum);
    sum
}

fn main() {
    let mut x = 0u32;
    x += 1u16 as u32;

    let x = (
        0.0f32,
        x,
    );
    let _x = (x.1,);

    let mut y = [0.0; 100_000];
    let x: [f32; 3] = [0.05, 0.10, 1.15];
    #[allow(clippy::manual_memcpy)]
    for i in 0..3 {
        y[i] = x[i];
    }
    //y[100_001] = 1.0;

    // println!("Hello, world! {:?}", x[0]);
    let s = print_and_sum(&mut y);
    println!("got {}", s);
}
