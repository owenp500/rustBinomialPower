fn main() {
    let n = 10;
    
    for k in 0..n {
        let choose = fact(n) / (fact(n-k) * fact(k));
        
        print!("{}(x^{} y^{}) + ",choose,n-k,k);
   } 
}
fn fact(f:usize) -> usize{
    if f > 1 {
        return f * fact(f-1);
    }
    else {
        return 1;
    }
}