
fn gnome<T>(arr: &[T])->Vec<T>
where
    T:PartialEq+PartialOrd+Clone,
{
    let mut arr = arr.to_vec();
    let mut i: usize = 1;
    let mut j: usize = 2;

    while i < arr.len() {
        if arr[i - 1] < arr[i] {
            i = j;
            j = i + 1;
        } else {
            arr.swap(i - 1, i);
            i -= 1;
            if i == 0 {
                i = j;
                j += 1;
            }
        }
    }
    arr
}

#[allow(dead_code)]
pub fn g1(){
    println!("{:?}",gnome(&vec![6, 5, -8, 3, 2, 3]))
}