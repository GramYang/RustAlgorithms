
fn pancake<T>(arr:&mut [T])->Vec<T>
where
    T:PartialEq+Ord+PartialOrd+Clone,
{
    let len = arr.len();
    if len < 2 {
        arr.to_vec();
    }
    for i in (0..len).rev() {
        let max_index = arr
            .iter()
            .take(i + 1)
            .enumerate()
            .max_by_key(|&(_, elem)| elem)
            .map(|(idx, _)| idx)
            .unwrap();
        if max_index != i {
            arr[0..max_index + 1].reverse();
            arr[0..i + 1].reverse();
        }
    }
    arr.to_vec()
}

#[allow(dead_code)]
pub fn p1(){
    println!("{:?}",pancake(&mut vec![6, 5, -8, 3, 2, 3]))
}