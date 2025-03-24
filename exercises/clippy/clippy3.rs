#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // 使用 `if let` 或 `match` 处理 `None` 情况
        println!("my_option is None");
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_vec = vec![1, 2, 3, 4, 5];
    my_vec.clear(); // 直接修改原 Vec
    println!("This Vec is empty, see? {:?}", my_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 使用 `std::mem::swap` 交换两个变量的值
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}