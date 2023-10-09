use blog_2::Post;
fn main() {
    let mut post = Post::new();

    post.add_text("Hello");

    let post = post.request_review();

    let post = post.approve();
    assert_eq!("Hello", post.content());
}
