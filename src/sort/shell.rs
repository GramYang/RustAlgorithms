
fn shell<T:Ord+Copy>(values:&mut [T]){
    fn insertion<T: Ord + Copy>(values: &mut [T], start: usize, gap: usize) {
        for i in ((start + gap)..values.len()).step_by(gap) {
            let val_current = values[i];
            let mut pos = i;
            while pos >= gap && values[pos - gap] > val_current {
                values[pos] = values[pos - gap];
                pos -= gap;
            }
            values[pos] = val_current;
        }
    }

    let mut count_sublist = values.len() / 2;
    while count_sublist > 0 {
        for pos_start in 0..count_sublist {
            insertion(values, pos_start, count_sublist);
        }
        count_sublist /= 2;
    }
}

#[allow(dead_code)]
pub fn s1(){
    let mut vec = vec![6, 5, 4, 3, 2, 1];
    shell(&mut vec);
    println!("{:?}",vec)
}