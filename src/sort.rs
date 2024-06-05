///
///
/// # Arguments
///
/// * `arr`: array to sort
///
/// returns: ()
///
/// ***array is sorted in place***
///
/// # Examples
///
/// ```
///     use random::Source;
///
///     fn test() {
///         let mut source = random::default(42);
///         let mut input = source.iter().take(1000).collect::<Vec<i32>>();
///         let mut expected = input.clone();
///         let expected = expected.as_mut_slice();
///         expected.sort();
///
///         let input = input.as_mut_slice();
///         rust_examples::sort::quick_sort(input);
///
///         assert!(expected.eq(&input));
///     }
/// ```
pub fn quick_sort<T>(arr: &mut [T])
where
    T: PartialOrd + Copy,
{
    qsort(arr, 0, arr.len())
}

fn qsort<T>(arr: &mut [T], start: usize, end: usize)
where
    T: PartialOrd + Copy,
{
    assert!(end >= start);
    assert!(arr.len() >= (end - start));

    if start < end {
        let partition_point = partition(arr, start, end);
        qsort(arr, start, partition_point);
        qsort(arr, partition_point + 1, end);
    }
}

fn partition<T>(arr: &mut [T], start: usize, end: usize) -> usize
where
    T: PartialOrd + Copy,
{
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

#[allow(clippy::manual_swap)]
fn swap<T>(arr: &mut [T], from: usize, to: usize)
where
    T: PartialOrd + Copy,
{
    let tmp = arr[from];
    arr[from] = arr[to];
    arr[to] = tmp;
}

#[cfg(test)]
mod tests {
    use random::Source;

    use super::*;

    #[test]
    fn sort_i32() {
        let mut source = random::default(42);
        let mut input = source.iter().take(1000).collect::<Vec<i32>>();

        do_sort(&mut input);
    }

    #[test]
    fn sort_i8() {
        let mut source = random::default(42);
        let mut input = source.iter().take(1000).collect::<Vec<i8>>();

        do_sort(&mut input);
    }

    #[test]
    fn sort_u8() {
        let mut source = random::default(42);
        let mut input = source.iter().take(1000).collect::<Vec<u8>>();

        do_sort(&mut input);
    }

    #[test]
    fn sort_char() {
        let mut source = random::default(42);
        let mut input = source
            .iter()
            .take(1000)
            .map(|x: u8| x as char)
            .collect::<Vec<char>>();

        do_sort(&mut input);
    }

    fn do_sort<T>(input: &mut Vec<T>)
    where
        T: Clone + Ord + Copy,
    {
        let mut expected = input.clone();
        let expected = expected.as_mut_slice();
        expected.sort();

        let input = input.as_mut_slice();
        quick_sort(input);

        assert!(expected.eq(&input));
    }
}
