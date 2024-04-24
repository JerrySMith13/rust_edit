use device_query::{self, DeviceQuery, Keycode};

fn main() {
    loop{
        let keys_pressed: Vec<Keycode> = device_query::DeviceState::new().get_keys();
        for key in keys_pressed {
            println!("{:?}", key);
            if key == Keycode::Escape {
                break;
            }
        }
    }
}
