fn partition<T:PartialOrd>(arr:&mut [T],lo:isize,hi:isize)->isize{
    let pivot = hi as usize;
    let mut i = lo - 1;
    let mut j = hi;
    loop {
        i += 1;
        while arr[i as usize] < arr[pivot] {
            i += 1;
        }
        j -= 1;
        while j >= 0 && arr[j as usize] > arr[pivot] {
            j -= 1;
        }
        if i >= j {
            break;
        } else {
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap(i as usize, pivot as usize);
    i
}

fn quick<T: Ord>(arr: &mut [T], lo: isize, hi: isize) {
    if lo < hi {
        let p = partition(arr, lo, hi);
        quick(arr, lo, p - 1);
        quick(arr, p + 1, hi);
    }
}

#[allow(dead_code)]
pub fn q1(){
    let mut arr=vec![5,4,3,2,1];
    let len=arr.len();
    quick(&mut arr,0,(len-1)as isize);
    println!("{:?}",arr)
}