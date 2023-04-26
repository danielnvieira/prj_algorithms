use rand::Rng;
pub fn create_random_array_one_dimension(size: usize) -> Vec<i32> {
    let mut random = rand::thread_rng();
    let mut array = Vec::new();
    for _x in 0..size {
        array.push(random.gen_range(1..101));
    }
    println!("{:?}", array);
    return array;
}

pub fn create_array_two_dimension_square(size: usize) -> Vec<Vec<i32>> {
    let mut rng = rand::thread_rng();
    let mut arr_bi = vec![vec![0; size]; size];
    for x in 0..size {
        for y in 0..size {
            arr_bi[x][y] = rng.gen_range(0..2);
        }
    }
    println!("{:?}", arr_bi);
    return arr_bi;
}
