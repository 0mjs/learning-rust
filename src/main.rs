fn main() {
    let arr = [1, 2, 3, 4, 5];
    let mut even_arr: Vec<i32> = Vec::new();
    let is_even = |num: i32| num % 2 == 0;

    for item in arr.iter() {
        if is_even(*item) {
            even_arr.push(*item);
            println!("{} was even", item);
        } else {
            println!("{} was odd", item);
        }
    }

    println!("{:?}", even_arr);
}
