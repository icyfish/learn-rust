fn main() {
    let s1 = gives_ownership(); // gives_ownership 将其返回值 move 给 s1 

    println!("{}", s1);

    // let s2 = String::from("hello"); // s2 进入作用域

    // let s3 = takes_and_gives_back(s2); // s2 被 move 到 takes_and_gives_back, 同时函数返回值被 move 给 s3

    // println!("{}", s3);


} // s3 在这里跳出作用域, 然后被 drop, s2 被 move, 因此无事发生. s1 跳出作用域然后被 drop 

fn gives_ownership() -> String {
    // 调用 gives_ownership 会 move 返回值

    let some_string = String::from("yours"); // some_string 进入作用域

    some_string // some_string 被返回 
}

// 该函数接受一个 String, 然后返回它 
// fn takes_and_gives_back(a_string: String) -> String {
//     // a_string 进入作用域

//     a_string // a_string 被返回并被 move 出
// }