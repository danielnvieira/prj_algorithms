use crate::generation_arrays::create_array_two_dimension_square;
pub fn exec() {
    let size = 4;
    let arr_bi = create_array_two_dimension_square(size);
    let mut biggest = 1;

    for x in 0..size {
        print!("[");
        for y in 0..size {
            print!(" {} ", arr_bi[x][y]);
        }
        print!("]");
        println!();
    }

    for x in 0..size {
        let mut size_arr = 0;
        for y in 0..size {
            if arr_bi[x][y] == 1 {
                size_arr += 1;
                if size_arr > 1 && size_arr > biggest {
                    if is_square(&arr_bi, get_pos(x, size_arr), get_pos(y, size_arr), x, y) {
                        biggest = size_arr;
                        size_arr = 0;
                    } else {
                        size_arr = 0;
                    }
                }
            } else {
                size_arr = 0;
            }
        }
    }
    if biggest <= 1 {
        println!("Não encontro array formado de números 1 ");
    } else {
        println!(
            "O maior array formado de números 1 encontrado é {}",
            biggest
        );
    }
}

fn get_pos(x: usize, size_arr: usize) -> Option<usize> {
    return x.checked_sub(size_arr - 1);
}

fn is_square(
    arr: &Vec<Vec<i32>>,
    x: Option<usize>,
    y: Option<usize>,
    x_max: usize,
    y_max: usize,
) -> bool {
    if x.is_none() || y.is_none() {
        return false;
    }

    let mut square = true;
    for i in x.unwrap()..=x_max {
        for j in y.unwrap()..=y_max {
            if arr[i][j] != 1 {
                square = false;
                return square;
            }
        }
    }
    // if square {
    //     println!("x: {} max: {}", x.unwrap(), x_max);
    //     println!("y: {} max: {}", y.unwrap(), y_max);
    // }
    return square;
}
