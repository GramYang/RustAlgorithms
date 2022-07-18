
fn selection<T:Ord>(arr: &mut [T]){
    let len=arr.len();
    for left in 0..len{
        let mut smallest=left;
        for right in (left+1)..len{
            if arr[right]<arr[smallest]{
                smallest=right;
            }
        }
        arr.swap(smallest, left);
    }
}

#[allow(dead_code)]
pub fn s1(){
    let mut v=vec!["d", "a", "c", "b"];
    selection(&mut v);
    println!("{:?}",v)
}