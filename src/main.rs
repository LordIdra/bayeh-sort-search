fn linear_search<T>(items: &mut[T], target: &T) -> Option<usize>
where T: Eq {
    for index in 0..items.len() {
        if items[index] == *target {
            return Some(index);
        }
    }

    None
}

fn binary_search<T>(items: &mut[T], target: &T) -> Option<usize>
where T: Eq + Ord {
    let mut low = 0;
    let mut high = items.len()-1;
    let mut mid;

    while low <= high {
        mid = (high+low) / 2;

        if items[mid] < *target {
            low = mid + 1;

        } else if items[mid] > *target {
            high = mid - 1;

        } else {
            return Some(mid);
        }
    }

    None
}

fn bubble_sort<T>(items: &mut [T])
where T: Eq + Ord {
    let mut swap: bool = true;
    while swap {
        swap = false;
        for i in 0..(items.len()-1) {
            if items[i] > items[i+1] {
                (*items).swap(i, i+1);
                swap = true;
            }
        }
    }
}

fn insertion_sort<T>(items: &mut [T])
where T: Eq + Ord {
    for i in 0..items.len() {
        let key = &items[i];
        let mut j = i;
        while (j >= 0) && ()
    }
}

#[cfg(test)]
mod tests {
    use crate::{bubble_sort, insertion_sort};


    #[test]
    fn test_linear_search() {
        use crate::linear_search;

        let mut items: Vec<i32> = vec![2, 6, 8, -46, 0, 12, 58, 480];

        assert!(linear_search(&mut items, &13).is_none());
        assert!(linear_search(&mut items, &6).is_some());

        assert_eq!(linear_search(&mut items, &12).unwrap(), 5);
        assert_eq!(linear_search(&mut items, &-46).unwrap(), 3);
    }

    #[test]
    fn test_binary_search() {
        use crate::binary_search;

        let mut items: Vec<i32> = vec![-82, -6, 0, 2, 6, 8, 12, 58, 480];

        assert!(binary_search(&mut items, &13).is_none());
        assert!(binary_search(&mut items, &6).is_some());

        assert_eq!(binary_search(&mut items, &12).unwrap(), 6);
        assert_eq!(binary_search(&mut items, &0).unwrap(), 2);
        assert_eq!(binary_search(&mut items, &-82).unwrap(), 0);
        assert_eq!(binary_search(&mut items, &480).unwrap(), 8);
    }
    
    #[test]
    fn test_bubble_sort() {
        let mut items: Vec<i32> = vec![5, -6, 45, 2, -8, 823, -12, 0, 480];
        let sorted_items: Vec<i32> = vec![-12, -8, -6, 0, 2, 5, 45, 480, 823];

        bubble_sort(&mut items);

        assert!(items == sorted_items);
    }

    #[test]
    fn test_insertion_sort() {
        let mut items: Vec<i32> = vec![5, -6, 45, 2, -8, 823, -12, 0, 480];
        let sorted_items: Vec<i32> = vec![-12, -8, -6, 0, 2, 5, 45, 480, 823];

        insertion_sort(&mut items);

        assert!(items == sorted_items);
    }
}

fn main() {}
