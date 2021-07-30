fn sumintegeru32(t: &[u32]) -> Option<u32> {
    if t.len() > 0 {
        let mut total:u32 = 0;
        for item in t.iter() {
            match total.checked_add(*item) {
                Some(s) => total = s,
                None => return None,
            } 
        }
        Some(total)
    }
    else {
        None
    }
}

fn main() {
    let a = vec![1, 2, 3];
    println!("sum is {:?}",sumintegeru32(&a));
}