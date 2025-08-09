fn main() {
   let mut s = String::from("Hello");
   s.push_str(" World!");

   let s2 = s.clone();

   println!("{s2}");
}
