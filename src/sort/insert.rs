pub fn insert<T>(arr:&mut [T])
where 
    T: Ord + Copy,
{
    for i in 1..arr.len(){
        let cur=arr[i];
        let mut j=i-1;
        while arr[j]>cur{
            arr.swap(j+1,j);
            if j==0{
                break;
            }
            j-=1;
        }
    }
}

#[allow(dead_code)]
pub fn i1(){
    let mut arr=vec!["d","a","c","e","b"];
    insert(&mut arr);
    println!("{:?}",arr)
}