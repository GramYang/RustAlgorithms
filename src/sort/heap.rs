fn heap<T:Ord>(arr:&mut [T]){
    if arr.len()<=1{
        return;
    }
    heapify(arr);
    for end in (1..arr.len()).rev(){
        arr.swap(0, end);
        move_down(&mut arr[..end],0);
    }
}

fn heapify<T:Ord>(arr:&mut [T]){
    let last_parent=(arr.len()-2)/2;
    for i in (0..=last_parent).rev(){
        move_down(arr,i);
    }
}

fn move_down<T:Ord>(arr:&mut [T],mut root:usize){
    let last = arr.len() - 1;
    loop {
        let left = 2 * root + 1;
        if left > last {
            break;
        }
        let right = left + 1;
        let max = if right <= last && arr[right] > arr[left] {
            right
        } else {
            left
        };

        if arr[max] > arr[root] {
            arr.swap(root, max);
        }
        root = max;
    }
}

#[allow(dead_code)]
pub fn h1(){
    let mut arr = vec![3, 4, 2, 1, 7];
    heap(&mut arr);
    println!("{:?}",arr)
}