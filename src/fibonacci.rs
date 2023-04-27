pub fn exec() {
    let mut arr: Vec<i32> = Vec::new();
    for x in 0..30 {
        if arr.is_empty() {
            arr.push(0);
        } else if arr.len() <= 2 {
            arr.push(1);
        } else {
            println!("{:?}", arr);
            println!("{}", arr[x-1]);
            println!("{}", arr[x-2]);
            arr.push(arr[x - 1] + arr[x - 2]);
        }

    }
    println!("{:?}", arr);
}
