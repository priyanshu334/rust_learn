fn main(){
    let mut s1 :String = String::from("harik");
    let mut s2 = s1;
    s1 =s2.clone();
    println!("{}",s1);
    println!("{}",s2);
}