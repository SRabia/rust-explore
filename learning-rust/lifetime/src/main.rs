// annotate a lifetime for x, y to be the same lifetime for
// return which is the shortest lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len(){y}else{y}
}
// hold a ref "part" instance of this struct cannot outlive
// the ref it hold
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let s1 = "aa";
    let s2 = "ajfia";

    println!("the longest {}", longest(s1, s2));

    let novel = String::from("call mem ididiididid, didi, ya yay yaya");
    let first_sentence = novel.split('.').next().expect("couldn not find a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    };

}
