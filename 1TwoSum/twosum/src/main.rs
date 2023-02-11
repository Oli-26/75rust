fn main() {
    let two_sum = vec![1, 5, 202, 342, 1, 7, 9, 432, 42, 4, 5, 22];
    let to_match = 3644;

    // N log2(N)
    let sorted_vec = merge_sort(two_sum);

    // N
    let mut trimmed_vec : Vec<i32> = sorted_vec.into_iter().filter(| n | *n < to_match).collect::<Vec<i32>>();
    println!("{:?}", trimmed_vec);

    let mut even : Vec<(usize, i32)> = vec![];
    let mut odd : Vec<(usize, i32)> = vec![];

    // N
    let mut index : usize = 0;
    for n in &trimmed_vec {
        if n % 2 == 0{
            even.push((index, *n));
        }else{
            odd.push((index, *n));
        }
        index += 1;
    }

    // N Log2(N)
    if to_match % 2 == 0{
        for (i1, n1) in even.iter() {
            let i2 = find(to_match - n1, &even);
            match i2 {
                Some(ind) => {
                    println!("{}, {}", *i1, ind);
                    return;
                }, 
                            
                None => {},
            }
            
        }
    }else{
        for (i1, n1) in odd.iter() {
            let i2 = find(to_match - n1, &even);
            match i2 {
                Some(ind) => {
                    println!("{}, {}", *i1, ind);
                    return;
                },
                None => {},
            }
            return;
        }
    }

    println!("No pair found");
}


fn find(n : i32, vec : &Vec<(usize, i32)>) -> Option<usize> {
    let mut index1 = 0;
    let mut index2 = vec.len();
    let mut pivot_point = (index1 + index2) / 2;
    loop {
        if vec[pivot_point].1 == n{
            return Some(vec[pivot_point].0);
        }
        if vec[pivot_point].1 < n {
            index1 = pivot_point;    
        }else if vec[pivot_point].1 > n {
            index2 = pivot_point;
        }

        if pivot_point == (index1 + index2) / 2 {
            return None;
        }
        pivot_point = (index1 + index2) / 2;   
    }
}

fn merge_sort(vec : Vec<i32>) -> Vec<i32> {
    if vec.len() == 1 {
        return vec;
    }else{
        return merge(merge_sort(vec[..vec.len()/2].to_vec()), merge_sort(vec[vec.len()/2..].to_vec()));
    }
}

fn merge(vec1 : Vec<i32>, vec2 : Vec<i32>) -> Vec<i32> {
    let mut i1 : usize = 0;
    let mut i2 : usize = 0;
    let mut merged_vec = vec![];
    while (i1 + i2) <= vec1.len() + vec2.len(){
        if i1 >= vec1.len(){
            merged_vec.append(&mut vec2[i2..].to_vec());
            return merged_vec;
        }
        if i2 >= vec2.len(){
            merged_vec.append(&mut vec1[i1..].to_vec());
            return merged_vec;
        }

        if vec1[i1] <= vec2[i2]{
            merged_vec.push(vec1[i1]);
            i1 += 1;
        }else{
            
            merged_vec.push(vec2[i2]);
            i2 += 1;
        }
    }
    merged_vec
}