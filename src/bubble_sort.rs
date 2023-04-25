use rand::Rng;

pub fn exec(){
    let size:usize =10;
    let mut array = create_random_array(size);
    println!();
    
    for x in 0..size{
        for y in 0..size{
            if array[y]>array[x]{
                let temp = array[y];
                array[y]=array[x];
                array[x]=temp;
            }
        }
    }
    print!("Desordenado {:?}",array);
    println!();
}

fn create_random_array(size:usize)-> Vec<i32> {
    let mut random = rand::thread_rng();
    let mut array=Vec::new();
    for _x in 0..size {
        array.push(random.gen_range(1..101));
    }
    print!("Ordenado    {:?}",array);
    return array;
}