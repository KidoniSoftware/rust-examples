#[cfg(test)]
mod tests {
    #[test]
    fn iterator_examples_filtering() {
        let data = vec![0, 1, 2, 3, 4, 5];

        let even: Vec<_> = data.iter().filter(|x| (*x % 2) == 0).map(|i| *i).collect();
        assert_eq!(even, vec![0, 2, 4]);

        let even: Vec<_> = data
            .iter()
            .filter_map(|i| if i % 2 == 0 { Some(*i) } else { None })
            .collect();

        assert_eq!(even, vec![0, 2, 4]);
    }

    #[test]
    fn iterator_examples_folding() {
        let data = vec![0, 1, 2, 3, 4, 5];

        let sum = data.iter().fold(0, |sum, value| sum + value);
        assert_eq!(sum, 15);
    }

    #[test]
    fn iterator_examples_flatten() {
        let data = [vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let result = data.iter().flatten().map(|x| *x).collect::<Vec<_>>();
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let result = data
            .iter()
            .flat_map(|x| x.iter().map(|y| *y).collect::<Vec<i32>>())
            .collect::<Vec<_>>();
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let result = data.into_iter().flatten().collect::<Vec<_>>();
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
