

pub fn strvec_to_stringvec(vec:&Vec<&str>) -> Vec<String> {
    let mut v =  Vec::new();
    for x in vec {
        v.push(x.to_string());
    }
    v
}