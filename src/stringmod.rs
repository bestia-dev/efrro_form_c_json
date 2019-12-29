// stringmod.rs
//! strings utils

///concat
pub fn concat_4(one: &str, two: &str, three: &str, four: &str) -> String {
    let mut ret = String::with_capacity(50);
    ret.push_str(one);
    ret.push_str(two);
    ret.push_str(three);
    ret.push_str(four);
    //return
    ret
}
///concat
pub fn concat_5(one: &str, two: &str, three: &str, four: &str, five: &str) -> String {
    let mut ret = String::with_capacity(50);
    ret.push_str(one);
    ret.push_str(two);
    ret.push_str(three);
    ret.push_str(four);
    ret.push_str(five);
    //return
    ret
}
