// stringmod.rs
//! strings utils

///concat
pub fn concat_4(one: &str, two: &str, three: &str, four: &str) -> String {
    let mut ret = String::new();
    ret.push_str(one);
    ret.push_str(two);
    ret.push_str(three);
    ret.push_str(four);
    //return
    ret
}
