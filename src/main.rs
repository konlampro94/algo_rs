

fn selection_sort<T: PartialOrd + Copy>(arr: &mut Vec<T>) -> &Vec<T>
{
    
    let mut _temp = arr[0];
    let mut _min = 0;
    
    for i in 0..arr.len() 
    {
        _min = i;
        for j in i+1..arr.len()
        {
            if arr[j] < arr[_min] 
            {
                _min = j;
            }
        }
        _temp = arr[i];
        arr[i] = arr[_min];
        arr[_min] = _temp;
    }
    arr
}

/// 
mod ins_sort;

/// Returns the index of the position in the array
fn elem_search<T: PartialEq + Copy>(arr: &Vec<T>, elem: T) -> Option<usize> 
{
    for (i, &pair) in arr.iter().enumerate() 
    {
        if pair == elem 
        {
            return Some(i)
        } 
    }
    None
}


/// performs the classic Bubblesort algorithm
fn bubblesort<T: PartialOrd + Copy>(arr: &mut Vec<T>) ->  &Vec<T>
{

    let mut _temp = arr[0];
    let  arr_len = arr.len();
    for i in 0..arr_len
    {
        let mut j = arr_len - 1;
        while j > i 
        { 
            if arr[j] < arr[j-1]
            {
                _temp = arr[j-1];
                arr[j-1] = arr[j];
                arr[j] = _temp;
            }
        j = j - 1;
        }
    }
    arr
}


fn main() 
{
    let mut v  = vec![50, 30, 60, 90, 120, 5, 67, 63];
    
    println!("\nInitial Array contents => {:?}",v);
    
    bubblesort(&mut v);

    println!("\nMain: => Array contents after bubblesort => {:?}", v);

    let mut v  = vec![50, 30, 60, 90, 120, 5, 67, 63];

    selection_sort(&mut v);

    println!("\nMain: => Array contents after selectionsort => {:?}", v);

    match elem_search(&v, 67) {
        Some(num) => println!("Element [{}] exists!!", num),
        None  => println!("Element does't exist in the array!"),
    }
    ins_sort::insertion_sort();
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bubble_search_works() 
    {
        let test_vec : Vec<i32> =  vec![1, 2, 3, 4, 5]; 
        assert_eq!(bubblesort(&mut vec![5, 4, 3, 2, 1]), &test_vec);
        assert_ne!(bubblesort(&mut vec![5, 3, 3, 2, 1]), &test_vec);
        let vec_f32 : Vec<f32> = vec![1.4, 3.2, 5.7, 7.8];
        assert_eq!(bubblesort(&mut vec![1.4, 5.7, 3.2, 7.8]), &vec_f32);

    }
    #[test]
    fn selection_sort_works()
    {
        let test_vec : Vec<i32> =  vec![1, 2, 3, 4, 5];
        assert_eq!(selection_sort(&mut vec![5, 4, 3, 2, 1]), &test_vec);
        let vec_f32 : Vec<f32> = vec![1.4, 3.2, 5.7, 7.8];
        assert_eq!(selection_sort(&mut vec![1.4, 5.7, 3.2, 7.8]), &vec_f32);
    }

    #[test]
    fn elem_search_works() 
    {
        let test_vec : Vec<i32> =  vec![1, 2, 3, 4, 5];
        assert_eq!(elem_search(&test_vec, 3), Some(2));
        assert_eq!(elem_search(&test_vec, 39), None);
    }
}