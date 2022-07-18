fn comb<T:Ord>(arr:&mut [T]){
    let mut gap=arr.len();
    let shrink=1.3;
    let mut sorted=false;
    while !sorted {
        gap = (gap as f32 / shrink).floor() as usize;
        if gap <= 1 {
            gap = 1;
            sorted = true;
        }
        for i in 0..arr.len() - gap {
            let j = i + gap;
            if arr[i] > arr[j] {
                arr.swap(i, j);
                sorted = false;
            }
        }
    }
}

#[allow(dead_code)]
pub fn c1(){
    let mut vec1 = vec![6, 5, 4, 3, 2, 1];
    comb(&mut vec1);
    println!("{:?}",vec1)
}