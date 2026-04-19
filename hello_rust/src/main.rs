fn main() {
    // hello world
    println!("Hello, world!");
    // 变量
    let (x, y, z) = (1, 2, 3);
    println!("x = {}", x);
    println!("Interproduct = {}", interproduct(x, y, z));
    println!("fib = {}", fib(3));
    for i in 1..z {
        println!("i: {i}");
    }
    let mut tmp = 1;
    loop{
        println!("tmp = {}", tmp);
        if tmp >= 5 {
            break;
        }
        tmp += 1;
    }
    let a = 10;
    println!("before: {a}");
    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }

    println!("after: {a}");

}
fn interproduct(a:i32, b:i32, c:i32) -> i32 {
    a * b + b * c + c * a
}

fn fib(n: u32) -> u32 {
    if n <= 2 {
        // The base case.
        1
    } else {
        // The recursive case.
        fib(n - 1) + fib(n - 2)
    }
}