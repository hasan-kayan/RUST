use usb_enumeration::{Event, Observer};
use crate::device::Device;
use std::sync::mpsc::Receiver;
use std::thread;

pub fn start_usb_monitor() -> Receiver<DeviceEvent> {
    let (tx, rx) = std::sync::mpsc::channel();
    thread::spawn(move || {
        let observer = Observer::new().subscribe();
        for event in observer.rx_event.iter() {
            match event {
                Event::Connect(dev) => {
                    let device = Device {
                        id: dev.id,
                        vendor_id: dev.vendor_id,
                        product_id: dev.product_id,
                        description: dev.description,
                    };
                    tx.send(DeviceEvent::Connected(device)).unwrap();
                }
                Event::Disconnect(dev) => {
                    let device = Device {
                        id: dev.id,
                        vendor_id: dev.vendor_id,
                        product_id: dev.product_id,
                        description: dev.description,
                    };
                    tx.send(DeviceEvent::Disconnected(device)).unwrap();
                }
                _ => {}
            }
        }
    });
    rx
}

pub enum DeviceEvent {
    Connected(Device),
    Disconnected(Device),
}
