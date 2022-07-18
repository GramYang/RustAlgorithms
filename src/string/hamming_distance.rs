
fn hamming_distance(string1:&str,string2:&str)->usize{
    let mut distance=0;
    let mut string1=string1.chars();
    let mut string2=string2.chars();
    loop{
        match(string1.next(),string2.next()){
            (Some(char1),Some(char2)) if char1 !=char2=> distance+=1,
            (Some(char1), Some(char2)) if char1 == char2 => continue,
            (None, Some(_)) | (Some(_), None) => panic!("Strings must have the same length"),
            (None, None) => break,
            _ => unreachable!(),
        }
    }
    distance
}

#[allow(dead_code)]
pub fn h1(){
    let result=hamming_distance("karolin", "kathrin");
    println!("{}",result)
}