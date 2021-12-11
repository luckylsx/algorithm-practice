pub fn bubble(vecs: &mut Vec<i32>) -> &mut Vec<i32> {
    if vecs.len() <= 1 {
        return vecs;
    }
    let mut e = vecs.len()-1;
    while e > 0 {
        for j in 0..e {
            if vecs[j] > vecs[j+1] {
                vecs.swap(j, j+1)
            }
        }
        e-=1
    }
    vecs
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bubble_test() {
        let mut vecs:Vec<i32> = vec![90,100,8,7,17,29,10];
        let mut wanted:Vec<i32> = vec![7,8,10,17,29,90,100];
        assert_eq!(&mut wanted, bubble(&mut vecs));
    }
}
