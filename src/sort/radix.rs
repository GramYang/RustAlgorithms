fn radix(arr:&mut [u64]){
    let max:usize=match arr.iter().max(){
        Some(&x)=>x as usize,
        None=>return,//这里的return直接退出了radix
    };
    let radix = arr.len().next_power_of_two();
    let mut place = 1;
    while place <= max {
        let digit_of = |x| x as usize / place % radix;
        let mut counter = vec![0; radix];
        for &x in arr.iter() {
            counter[digit_of(x)] += 1;
        }
        for i in 1..radix {
            counter[i] += counter[i - 1];
        }
        for &x in arr.to_owned().iter().rev() {
            counter[digit_of(x)] -= 1;
            arr[counter[digit_of(x)]] = x;
        }
        place *= radix;
    }
}

#[allow(dead_code)]
pub fn r1(){
    let mut v=vec![201, 127, 64, 37, 24, 4, 1];
    radix(&mut v);
    println!("{:?}",v)
}