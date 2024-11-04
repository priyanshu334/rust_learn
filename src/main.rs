//boworing 
fn main(){
    let s1 :String = String::from("hello");

    do_something(s2:&s1);

}
fn do_something(s2:&String){
   println!("{}",s2);
}