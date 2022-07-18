
fn odd_even<T:Ord>(arr: &mut [T]){
    let len = arr.len();
    if len == 0 {
        return;
    }
    let mut sorted = false;
    while !sorted {
        sorted = true;
        for i in (1..len - 1).step_by(2) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
        for i in (0..len - 1).step_by(2) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
    }
}


#[allow(dead_code)]
pub fn o1(){
    let mut v=vec![3, 5, 1, 2, 4, 6];
    odd_even(&mut v);
    println!("{:?}",v)
}