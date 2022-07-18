fn exchange(arr:&mut [i32]){
    let length=arr.len();
    for n1 in 0..length{
        for n2 in (n1+1)..length{
            if arr[n2]<arr[n1]{
                arr.swap(n1, n2);
            }
        }
    }
}

#[allow(dead_code)]
pub fn e1(){
    let mut a1 = [213, 542, 90, -23412, -32, 324, -34, 3324, 54];
    exchange(&mut a1);
    println!("{:?}",a1);
}