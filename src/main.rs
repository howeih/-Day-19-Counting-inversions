fn find_inversions(items: Vec<i32>, left: usize, end: usize) -> (Vec<i32>, i32) {
    let mut inv_num = 0;
    let count = (end - left) / 2;
    let mid = left + count;
    if left >= mid {
        return (items, 0);
    }
    let (items, inv) = find_inversions(items, left, mid);
    inv_num += inv;
    let (mut items, inv) = find_inversions(items, mid, end);
    inv_num += inv;
    let mut i = left;
    let mut j = mid;

    let mut merge_result = Vec::<i32>::new();
    while i < mid && j < end {
        if items[i] <= items[j] {
            merge_result.push(items[i]);
            i += 1;
        } else {
            merge_result.push(items[j]);
            j += 1;
            inv_num += mid as i32 - i as i32;
        }
    }
    for l in i..mid {
        merge_result.push(items[l]);
    }
    for r in j..end {
        merge_result.push(items[r]);
    }
    let mut merge_i = 0;
    for i in left..end {
        items[i] = merge_result[merge_i];
        merge_i += 1;
    }
    (items, inv_num)
}

fn main() {
    let items = vec![29, 25, 17, 15, 6, 8, 11, 15, 7, 5];
    let item_len = items.len();
    let (items, inv) = find_inversions(items, 0, item_len);
    println!("{:?}, number of inversions:{}", items, inv);
}
