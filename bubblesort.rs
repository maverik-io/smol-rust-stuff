fn print_array(array: &[i32]) {
    print!("[ ");
    for i in array {
        print!("{i} ");
    }
    println!("]");
}

fn bubblesort(array: &mut [i32]) {
    let n = array.len();
    let mut i = 0;
    while i < (n - 1) {
        let mut j = 0;
        while j < (n - i - 1) {
            if array[j] > array[j + 1] {
                let temp = array[j].clone();
                array[j] = array[j + 1].clone();
                array[j + 1] = temp;
            }
            j += 1;
        }
        i += 1;
        print!("Pass {i}         : ");
        print_array(&array);
    }
}

fn main() {
    let mut array = [90, 89, 78, 67, 56, 45, 34, 23, 12];
    print!("Unsorted Array : ");
    print_array(&array);
    println!("");

    bubblesort(&mut array);

    print!("\nSorted Array   : ");
    print_array(&array);
}
