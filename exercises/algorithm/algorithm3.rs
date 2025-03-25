/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
// I AM NOT DONE

fn sort(array: &mut [i32]) {
    if array.is_empty() {
        return;
    }

    // 找到数组中的最小值和最大值
    let min = *array.iter().min().unwrap();
    let max = *array.iter().max().unwrap();

    // 创建计数数组
    let mut count = vec![0; (max - min + 1) as usize];

    // 计算每个元素的出现次数
    for &num in array.iter() {
        count[(num - min) as usize] += 1;
    }

    // 将排序后的结果写回原数组
    let mut index = 0;
    for (num, &cnt) in count.iter().enumerate() {
        for _ in 0..cnt {
            array[index] = num as i32 + min;
            index += 1;
        }
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