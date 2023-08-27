

pub fn sort(v: &mut [i32]){
    for _ in 0..v.len() -1{
        for i in 1..v.len(){
            if v[i] < v[i -1]{
                v.swap(i -1, i);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_sort1() {
        let mut v = vec![7,8,96,1];
        let expect = vec![1, 7, 8, 96];
        sort(&mut v);
        assert_eq!(v, expect);
    }

    #[test]
    fn test_sort2() {
        let mut v = vec![8, 9, 7, 0];
        let expect = vec![0, 7, 8, 9];
        sort(&mut v);
        assert_eq!(v, expect);
    }
}
