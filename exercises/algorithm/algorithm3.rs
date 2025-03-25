/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort(array: &mut [u32]){
    let mut temp=0;
    for i in 0..array.len(){
       let mut  min=array[i];
       temp=i;
        for j in i..array.len(){
            if array[j]<min{
                temp=j;
                min=array[j];
            }
        }
        array[temp]=array[i];
        array[i]=min;
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