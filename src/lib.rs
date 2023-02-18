pub fn combinations<T>(data: &[T], r: usize) -> Vec<Vec<&T>> {
    fn combinations_helper<'a, T>(start: usize, data: &'a [T], r: usize, comb: &mut Vec<&'a T>, result: &mut Vec<Vec<&'a T>>) {
        if r == 0 {
            result.push(comb.clone());
        } else if start < data.len() {
            comb.push(&data[start]);
            combinations_helper(start + 1, data, r - 1, comb, result);
            comb.pop();
            combinations_helper(start + 1, data, r, comb, result);
        }
    }

    let mut result = vec![];
    let mut comb = vec![];
    combinations_helper(0, data, r, &mut comb, &mut result);
    result
}