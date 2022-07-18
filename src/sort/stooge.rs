
fn stooge<T:Ord>(arr:&mut [T],start:usize,end:usize){
    if arr[start]>arr[end]{
        arr.swap(start,end);
    }
    if start+1>=end{
        return;
    }
    let k=(end-start+1)/3;
    stooge(arr,start,end-k);
    stooge(arr,start+k,end);
    stooge(arr,start,end-k);
}

fn stooge1<T:Ord>(arr:&mut [T]){
    let len=arr.len();
    if len==0{
        return;
    }
    stooge(arr, 0, len-1);
}

#[allow(dead_code)]
pub fn s1(){
    let mut vec = vec![3, 5, 6, 3, 1, 4];
    stooge1(&mut vec);
    println!("{:?}",vec)
}