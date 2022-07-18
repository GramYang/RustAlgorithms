
fn cycle(arr:&mut [i32]){
    for cycle_start in 0..arr.len(){
        let mut item=arr[cycle_start];
        let mut pos=cycle_start;
        for i in arr.iter().skip(cycle_start+1){
            if *i<item{
                pos+=1;
            }
        }
        if pos==cycle_start{
            continue;
        }
        while item==arr[pos]{
            pos+=1;
        }
        std::mem::swap(&mut arr[pos],&mut item);
        while pos != cycle_start{
            pos=cycle_start;
            for i in arr.iter().skip(cycle_start+1){
                if *i<item{
                    pos+=1;
                }
            }
            while item==arr[pos]{
                pos+=1;
            }
            std::mem::swap(&mut arr[pos],&mut item);
        }
    }
}

#[allow(dead_code)]
pub fn c1(){
    let mut arr1 = [213, 542, 90, -23412, -32, 324, -34, 3324, 54];
    cycle(&mut arr1);
    println!("{:?}",arr1)
}