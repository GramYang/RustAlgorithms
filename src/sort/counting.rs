use std::ops::AddAssign;


fn counting(arr:&mut [u32],maxval:usize){
    let mut occurences:Vec<usize>=vec![0;maxval+1];
    for &data in arr.iter(){
        occurences[data as usize]+=1;
    }
    let mut i=0;
    for(data,&number) in occurences.iter().enumerate(){
        for _ in 0..number{
            arr[i] =data as u32;
            i+=1;
        }
    }
}

fn generic_counting<T:Into<u64>+From<u8>+AddAssign+Copy>(arr:&mut [T],maxval:usize){
    let mut occurences:Vec<usize>=vec![0;maxval+1];
    for &data in arr.iter(){
        occurences[data.into() as usize]+=1;
    }
    let mut i=0;
    let mut data=T::from(0);
    for &number in occurences.iter(){
        for _ in 0..number{
            arr[i]=data;
            i+=1;
        }
        data+=T::from(1);
    }
}

#[allow(dead_code)]
pub fn c1(){
    let mut v1=vec![6, 5, 4, 3, 2, 1];
    counting(&mut v1, 6);
    println!("{:?}",v1);
    let mut v2:Vec<u8>=vec![100, 30, 60, 10, 20, 120, 1];
    generic_counting(&mut v2, 120);
    println!("{:?}",v2);
}