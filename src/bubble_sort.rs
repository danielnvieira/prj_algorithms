use crate::generation_arrays::create_random_array_one_dimension;

pub fn exec() {
    let size: usize = 10;
    let mut array = create_random_array_one_dimension(size);

    for x in 0..size {
        for y in 0..size {
            if array[y] > array[x] {
                let temp = array[y];
                array[y] = array[x];
                array[x] = temp;
            }
        }
    }
    println!("{:?}", array);
}
