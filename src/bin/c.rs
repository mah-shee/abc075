#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n:usize,
        mut s:[usize; n],
    }
    s.sort();
    let mut ans = s.iter().sum::<usize>();
    if ans % 10 == 0 {
        for i in s.iter() {
            if i % 10 != 0 {
                ans -= i;
                break;
            }
        }
    }
    if ans % 10 == 0 {
        println!("{}", 0);
    } else {
        println!("{}", ans);
    }
}
