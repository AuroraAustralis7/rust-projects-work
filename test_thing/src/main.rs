fn main() {
    let mut v = vec![0, 1, 2];
    give_and_take(&mut v, 4);
}


fn give_and_take(v: &mut Vec<i32>, n: i32) -> i32 {
    v.push(n);
    v.remove(0)
}