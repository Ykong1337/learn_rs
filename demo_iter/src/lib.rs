mod tests {
    #[test]
    fn test_iter() {
        let v1 = vec![1, 2, 3, 4];
        let mut v1_iter = v1.iter();

        let count: i32 = v1_iter.sum();
        assert_eq!(count, 10);

        let vec: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(vec, vec![2, 3, 4, 5]);
    }
}