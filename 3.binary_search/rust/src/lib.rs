pub fn binary_search(vecs: &mut Vec<i32>, target: i32) -> i32 {
    if vecs.len() == 0 {
        return -1;
    }
    let (mut low, mut high) = (0, vecs.len()-1);
    while low <= high {
        let middle = (low + high) >> 1;
        if vecs[middle] > target {
            high = middle-1;
        } else if vecs[middle] < target {
            low = middle+1;
        } else {
            return middle as i32;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn binary_search_test() {
        let mut vecs1 = vec![7, 8, 10, 17, 29, 90, 100];
        let target = 7;
        assert_eq!(0, binary_search(&mut vecs1, target));

        let mut vecs2 = vec![7, 8, 10, 17, 29, 90, 100];
        let target = 100;
        assert_eq!(6, binary_search(&mut vecs2, target));

        let mut vecs3 = vec![7, 8, 10, 17, 29, 90, 100];
        let target = 1000;
        assert_eq!(-1, binary_search(&mut vecs3, target));
    }
}
