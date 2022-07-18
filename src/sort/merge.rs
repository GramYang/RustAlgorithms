
fn merge1<T:Ord+Copy>(arr: &mut [T],mid :usize){
    let left_half=arr[..mid].to_vec();
    let right_half=arr[mid..].to_vec();
    let mut l=0;
    let mut r=0;
    for v in arr{
        if r==right_half.len() || (l<left_half.len() && left_half[l]<right_half[r]){
            *v=left_half[l];
            l+=1;
        }else{
            *v=right_half[r];
            r+=1;
        }
    }
}

fn merge<T:Ord+Copy>(arr: &mut [T]){
    if arr.len()>1{
        let mid=arr.len()/2;
        merge(&mut arr[..mid]);
        merge(&mut arr[mid..]);
        merge1(arr,mid);
    }
}

#[allow(dead_code)]
pub fn m1(){
    let mut v=vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
    merge(&mut v);
    println!("{:?}",v);
}