use app::init::fetch_data;

fn main() {
  let contents = fetch_data().unwrap();
  println!("{:?}", contents);
}
