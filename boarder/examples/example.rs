use boarder::{start_usb_monitor, DeviceEvent};

fn main() {
    let rx = start_usb_monitor();
    for event in rx.iter() {
        match event {
            DeviceEvent::Connected(device) => {
                println!("Device connected: {:?}", device);
            }
            DeviceEvent::Disconnected(device) => {
                println!("Device disconnected: {:?}", device);
            }
        }
    }
}
