
fn main() {
    
    for i in 0..100 {
        let nums = gen_nums();
        let sorted_nums = quick_sort(nums);
        print!("{}\n",i);
    }
}

fn gen_nums() -> Vec<i32> {
    let mut nums :Vec<i32> = Vec::new();
    for i in 0..100000 {
        nums.push(rand::random::<i32>());
    }
    nums
}

fn quick_sort(mut nums :Vec<i32>) -> Vec<i32> {

    if nums.len() <=1 {
        return nums;
    }

    let pivot = nums.swap_remove(0);
    let mut larger_nums :Vec<i32> = Vec::new();
    let mut smaller_nums :Vec<i32> = Vec::new();

    for i in nums {
        if i <= pivot {
            smaller_nums.push(i);
        } else {
            larger_nums.push(i)
        }
    }

    let mut sorted_smaller = quick_sort(smaller_nums);
    let mut sorted_larger =  quick_sort(larger_nums);
    
    sorted_smaller.push(pivot);
    sorted_smaller.append(&mut sorted_larger);
    sorted_smaller
}
