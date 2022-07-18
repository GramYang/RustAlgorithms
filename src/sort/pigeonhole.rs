fn pigeonhole(array: &mut [i32]){
    if let (Some(min), Some(max)) = (array.iter().min(), array.iter().max()) {
        let holes_range: usize = (max - min + 1) as usize;
        let mut holes = vec![0; holes_range];
        let mut holes_repeat = vec![0; holes_range];
        for i in array.iter() {
            let index = *i - min;
            holes[index as usize] = *i;
            holes_repeat[index as usize] += 1;
        }
        let mut index = 0;
        for i in 0..holes_range {
            while holes_repeat[i] > 0 {
                array[index] = holes[i];
                index += 1;
                holes_repeat[i] -= 1;
            }
        }
    }
}

#[allow(dead_code)]
pub fn p1(){
    let mut arr1 = [3, 3, 3, 1, 2, 6, 5, 5, 5, 4, 1, 6, 3];
    pigeonhole(&mut arr1);
    println!("{:?}",arr1)
}