fn bucket(arr: &[usize])->Vec<usize>{
    if arr.is_empty(){
        return vec![];
    }
    let max=*arr.iter().max().unwrap();
    let len = arr.len();
    let mut buckets=vec![vec![];len+1];
    for x in arr{
        buckets[len**x/max].push(*x);
    }
    for bucket in buckets.iter_mut(){
        super::insert::insert(bucket);
    }
    let mut result=vec![];
    for bucket in buckets{
        for x in bucket{
            result.push(x)
        }
    }
    result
}

#[allow(dead_code)]
pub fn b1(){
    let arr: [usize; 4] = [35, 53, 1, 0];
    let res = bucket(&arr);
    println!("{:?}",res)
}