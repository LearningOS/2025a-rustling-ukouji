/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: PartialOrd+Copy>(array: &mut [T]){
    // A merge sort
    if array.len() == 1 {
        return;
    }
    if array.len() == 2 {
        if array[0] > array[1] {
            array.swap(0, 1);
        }
        return
    }
	let mid = array.len() / 2;
    sort(&mut array[..mid]);
    sort(&mut array[mid..]);

    // Merge the two sub lists
    let mut left = 0;
    let mut right = mid;

    let mut sorted = vec![array[0]; array.len()];
    for i in 0..array.len() {
        if right < array.len() && (left == mid || array[left] > array[right]) {
            sorted[i] = array[right];
            right += 1;
        } else {
            sorted[i] = array[left];
            left += 1;
        }
    }
    for i in 0..array.len() {
        array[i] = sorted[i];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}