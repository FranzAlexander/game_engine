use game_platform::windows::*;

fn main() {
    // let test_vector = Vector2::new(2, 3);
    // println!("{:?}", test_vector);
    //windows::create_message_window();
   let mut win = Window::new().unwrap();

    win.create_window().unwrap();
}
