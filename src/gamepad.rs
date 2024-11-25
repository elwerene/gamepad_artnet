use crossbeam_channel::Sender;
use gilrs::{Button, Event, Gilrs};
use log::debug;
use std::time::Duration;

pub fn listen(artnet_sender: Sender<Vec<u8>>) {
    let mut gilrs = Gilrs::new().expect("Could not initialize gilrs");
    let mut data = vec![0u8; 512];
    loop {
        match gilrs.next_event() {
            Some(Event {
                id, event, time, ..
            }) => {
                let gamepad_id: usize = id.into();
                // why +8 ?
                let gamepad_start = 8 + gamepad_id * 40;

                debug!("{:?} New event from {}: {:?}", time, id, event);
                match event {
                    gilrs::EventType::ButtonChanged(button, value, _) => {
                        let artnet_value = (value * 255.0).clamp(0.0, 255.0) as u8;

                        data[gamepad_start
                            + match button {
                                Button::South => 0,
                                Button::East => 1,
                                Button::North => 2,
                                Button::West => 3,
                                Button::C => 4,
                                Button::Z => 5,
                                Button::LeftTrigger => 6,
                                Button::LeftTrigger2 => 7,
                                Button::RightTrigger => 8,
                                Button::RightTrigger2 => 9,
                                Button::Select => 10,
                                Button::Start => 11,
                                Button::Mode => 12,
                                Button::LeftThumb => 13,
                                Button::RightThumb => 14,
                                Button::DPadUp => 15,
                                Button::DPadDown => 16,
                                Button::DPadLeft => 17,
                                Button::DPadRight => 18,
                                Button::Unknown => 19,
                            }] = artnet_value
                    }
                    gilrs::EventType::AxisChanged(axis, value, _) => {
                        // * 300.0 as some gamepads do not reach 1.0
                        let value = (value * 300.0).clamp(-255.0, 255.0) as i16;
                        match axis {
                            gilrs::Axis::LeftStickX => {
                                data[gamepad_start + 20] = if value < -200 { 1 } else { 0 };
                                data[gamepad_start + 21] = if value > 200 { 1 } else { 0 };
                            }
                            gilrs::Axis::LeftStickY => {
                                data[gamepad_start + 22] = if value < -200 { 1 } else { 0 };
                                data[gamepad_start + 23] = if value > 200 { 1 } else { 0 };
                            }
                            gilrs::Axis::LeftZ => {
                                data[gamepad_start + 24] =
                                    if value < 0 { (-value) as u8 } else { 0 };
                                data[gamepad_start + 25] = if value < 0 { 0 } else { value as u8 };
                            }
                            gilrs::Axis::RightStickX => {
                                data[gamepad_start + 26] = if value < -200 { 1 } else { 0 };
                                data[gamepad_start + 27] = if value > 200 { 1 } else { 0 };
                            }
                            gilrs::Axis::RightStickY => {
                                data[gamepad_start + 28] = if value < -200 { 1 } else { 0 };
                                data[gamepad_start + 29] = if value > 200 { 1 } else { 0 };
                            }
                            gilrs::Axis::RightZ => {
                                data[gamepad_start + 30] =
                                    if value < 0 { (-value) as u8 } else { 0 };
                                data[gamepad_start + 31] = if value < 0 { 0 } else { value as u8 };
                            }
                            gilrs::Axis::DPadX => {
                                data[gamepad_start + 32] = if value < -200 { 1 } else { 0 };
                                data[gamepad_start + 33] = if value > 200 { 1 } else { 0 };
                            }
                            gilrs::Axis::DPadY => {
                                data[gamepad_start + 34] = if value < -200 { 1 } else { 0 };
                                data[gamepad_start + 35] = if value > 200 { 1 } else { 0 };
                            }
                            _ => {}
                        };
                    }
                    _ => continue,
                }

                artnet_sender
                    .send(data.clone())
                    .expect("Could not send artnet data to artnet thread");
            }
            None => std::thread::sleep(Duration::from_nanos(1)),
        }
    }
}
