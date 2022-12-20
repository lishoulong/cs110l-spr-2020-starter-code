/* The following exercises were borrowed from Will Crichton's CS 242 Rust lab. */

use std::collections::HashSet;

fn main() {
    println!("Hi! Try running \"cargo test\" to run tests.");
}

fn add_n(v: Vec<i32>, n: i32) -> Vec<i32> {
    let mut retv = Vec::new();
    for i in v.iter() {
        retv.push(i + n);
    }
    retv
}

fn test_n() {
    let v = vec!["1", "2", "3"];
    for x in v {
        println!("{}", x);
    }
    println!("{:?}", v);
    // can borrow
    // for x in &v {
    //     println!("{}", x);
    // }
    // println!("{:?}", v);

    let str = String::from("hello");
    let str2 = str;
    println!("hi {}", str2);
}

fn add_n_inplace(v: &mut Vec<i32>, n: i32) {
    for i in 0..v.len() {
        v[i] = v[i] + n;
    }
}

fn dedup(v: &mut Vec<i32>) {
    let mut dict = HashSet::new();
    let mut newV = Vec::new();
    for i in 0..v.len() {
        if dict.contains(&v[i]) {
            continue;
        } else {
            // v.remove(i);
            dict.insert(v[i]);
            newV.push(v[i]);
        }
    }
    v.clear();
    for i in 0..newV.len() {
        v.push(newV[i]);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_n() {
        assert_eq!(add_n(vec![1], 2), vec![3]);
    }

    #[test]
    fn test_add_n_inplace() {
        let mut v = vec![1];
        add_n_inplace(&mut v, 2);
        assert_eq!(v, vec![3]);
    }

    #[test]
    fn test_dedup() {
        let mut v = vec![3, 1, 0, 1, 4, 4];
        dedup(&mut v);
        assert_eq!(v, vec![3, 1, 0, 4]);
    }
}
