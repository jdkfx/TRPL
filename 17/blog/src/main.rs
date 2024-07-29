extern crate blog;
use blog::Post;

fn main() {
  let mut post = Post::new();
  post.add_text("今日はお昼にサラダを食べた");
  let post = post.request_review();
  let post = post.approve();
  assert_eq!("今日はお昼にサラダを食べた", post.content());
}