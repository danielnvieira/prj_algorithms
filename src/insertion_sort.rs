use crate::generation_arrays::create_random_array_one_dimension;
pub fn exec() {
    let size: usize = 10;
    let mut array = create_random_array_one_dimension(size);

    for x in 0..size {
        let mut y = x;
        while y > 0 && array[y] < array[y - 1] {
            let temp = array[y];
            array[y] = array[y - 1];
            array[y - 1] = temp;
            y -= 1;
        }
    }
    println!("{:?}", array);
}
