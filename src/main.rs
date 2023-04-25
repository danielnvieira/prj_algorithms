use rand::Rng;
fn main() {
    let size=4;
    let arr_bi= create_vector(size);
    let mut biggest=0;
    
    for x in 0..size {
        let mut size_arr =0;
        for y in 0..size {
            if arr_bi[x][y]==1 {
                if biggest==0 {
                    biggest=1;
                }
                size_arr+=1;
                if size_arr>1 && size_arr>biggest {
                    if is_square(&arr_bi, 
                    get_pos(x,size_arr),
                    get_pos(y,size_arr),  
                    x,
                    y){
                        biggest=size_arr
                    }else{
                        size_arr=0;
                    }
                }
            }else{
                size_arr=0;
            }
        }
        
    }
    println!("{}",biggest);
}

fn get_pos(x:usize,size_arr:usize)->Option<usize>{
    return x.checked_sub(size_arr - 1);
}

fn create_vector(size:usize) -> Vec<Vec<i32>> {
    let mut rng =rand::thread_rng();
    let mut arr_bi=vec![vec![0;size];size];
    for x in 0..size {
        for y in 0..size {
            arr_bi[x][y] = rng.gen_range(0..2);
        }
    }
    println!("{:?}",arr_bi);
    return arr_bi;
}

fn is_square(arr:&Vec<Vec<i32>> ,x:Option<usize>,y:Option<usize>,x_max:usize,y_max:usize)->bool{
    if x.is_none() || y.is_none() {
        return false;
    }
    let mut square=true;
    for i in x.unwrap()..x_max {
        if square{
            for j in y.unwrap()..y_max {
                if arr[i][j]!=1{
                    square=false;
                    break;
                }
            }
        }
        
    }
    return square;
}
