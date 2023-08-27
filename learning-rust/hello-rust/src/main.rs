
use std::fs::File;

fn nth_word(s:&str, nb_word_slice:u32) ->&str
{
    let bytes: &[u8] =s.as_bytes();
    let mut count =0;
    let mut prvidx=0;
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{

            if nb_word_slice == count{
                return &s[prvidx..i];
            }
            count += 1;
            prvidx = i;
        }
    }
    return &s[..];
}


fn main() {

    let s:String = String::from("helll in your");
    println!("{}", nth_word(&s, 30));

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    assert_eq!(slice, &[2,3]);

    let g = File::open("heloo.txt");

}



