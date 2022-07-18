fn bubble<T:Ord>(arr: &mut [T]){
    let mut sorted=false;
    let mut n=arr.len();
    while !sorted{
        sorted=true;
        for i in 0..n-1{
            if arr[i]>arr[i+1]{
                arr.swap(i, i+1);
                sorted=false;
            }
        }
        n-=1;
    }
}

#[allow(dead_code)]
pub fn b1(){
    let mut vec1=vec![6,5,4,3,2,1];
    bubble(&mut vec1);
    println!("{:?}",vec1);
}