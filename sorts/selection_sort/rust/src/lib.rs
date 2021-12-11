pub fn selection(vecs: &mut Vec<i32>) -> &mut Vec<i32> {
    if vecs.len() <= 1 {
        return vecs;
    }
    for i in 0..vecs.len()-1 {
        let mut min_index = i;
        for j in i+1..vecs.len() {
            if vecs[j] < vecs[min_index] {
                min_index = j;
            }
        }
        vecs.swap(i,min_index);
    }
    vecs
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn selection_test() {
        let mut vecs:Vec<i32> = vec![90,100,8,7,17,29,10];
        let mut wanted:Vec<i32> = vec![7,8,10,17,29,90,100];
        assert_eq!(&mut wanted, selection(&mut vecs));
    }
}
