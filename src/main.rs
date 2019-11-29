
fn elem_search(arr: &mut Vec<u32>) -> Option<u32> {
    let index : i32 = -1;
    
}


fn bubblesort(arr: &mut Vec<u32>)
{
    let mut temp = 0;
    //println!("temp => [{:?}]", temp);
    let  arr_len = arr.len();
    println!("\nLength of vec => [{:?}]",arr_len);
    for i in 0..arr_len
    {
        println!("i = [{:?}], ", i);
        let mut j = arr_len - 1;
        while j > i { 

            println!("j = {}", j);
            if arr[j] < arr[j-1]{

                temp = arr[j-1];
                arr[j-1] = arr[j];
                arr[j] = temp;
            }
        j = j - 1;
        }
    }
    println!("\nBubblesort => Array contents => {:?}", arr);
}


fn main() 
{
    let mut v  = vec![50, 30, 60, 90, 120, 5, 67, 63];
    println!("\nInitial array => {:?}",v);
    bubblesort(&mut v);
    println!("\nMain: => array => {:?}", v);

}
