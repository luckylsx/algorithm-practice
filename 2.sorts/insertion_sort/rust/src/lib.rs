pub fn insertion(vecs: &mut Vec<i32>) -> &mut Vec<i32> {
    if vecs.len() <= 1 {
        return vecs;
    }
    for i in 1..vecs.len() {
        let mut j = i-1;
        while j >= (0 as usize) {
            if vecs[j] > vecs[j+1] {
                vecs.swap(j, j+1);
            }
            if j == 0 {
                break;
            }
            j-=1;
        }
    }
    vecs
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insertion_test() {
        let mut vecs:Vec<i32> = vec![90,100,8,7,17,29,10];
        let mut wanted:Vec<i32> = vec![7,8,10,17,29,90,100];
        assert_eq!(insertion(&mut vecs), &mut wanted);
    }
}
