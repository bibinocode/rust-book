fn main() {
    // for 循环
    let array = [1, 2, 3];
    for i in array {
        print!("{}", i); // 123
    }

    // range 区间循环 or 0..10
    for c in 0u32..10 {
        print!("{}", c); // 0123456789
    }

    // if 可以当表达式
    let value = if true { "good" } else { "bad" };
    println!("{}", value); // good

    // while 循环
    let mut count = 0;

    while 1 == 1 {
        println!("while 循环");
        if count == 5 {
            break;
        };
        count += 1;
    }

    // loop 无限循环 一般用来处理线程
    loop {
        print!("loop 无限循环");
    }
}
