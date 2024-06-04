pub fn quick_sort(arr: &mut [i32]) {
    qsort(arr, 0, arr.len())
}

fn qsort(arr: &mut [i32], start: usize, end: usize) {
    assert!(end >= start);
    assert!(arr.len() >= (end - start));

    if start < end {
        let partition_point = partition(arr, start, end);
        qsort(arr, start, partition_point);
        qsort(arr, partition_point + 1, end);
    }
}

fn partition(arr: &mut [i32], start: usize, end: usize) -> usize {
    let mut pivot = start;
    let mut left = start + 1;
    let mut right = end;

    while left < right {
        if arr[left] <= arr[pivot] {
            swap(arr, left, pivot);
            pivot = left;
            left += 1;
        } else {
            swap(arr, left, right - 1);
            right -= 1;
        }
    }

    pivot
}

fn swap(arr: &mut [i32], from: usize, to: usize) {
    let tmp = arr[from];
    arr[from] = arr[to];
    arr[to] = tmp;
}

#[cfg(test)]
mod tests {
    use random::Source;

    use super::*;

    #[test]
    fn sort() {
        let mut source = random::default(42);
        let mut input = source.iter().take(1000).collect::<Vec<i32>>();
        let mut expected = input.clone();
        let input = input.as_mut_slice();
        let expected = expected.as_mut_slice();
        expected.sort();

        quick_sort(input);

        assert!(expected.eq(&input));
    }
}
