use i3ipc::{I3EventListener, Subscription, event::{Event, inner::{WindowChange::{self, *} } } };
use polybartender::runner;
use std::{thread, time::Duration};

fn main() {
    let mut rx : I3EventListener = I3EventListener::connect().unwrap();

    rx.subscribe(&[Subscription::Window]).unwrap();
    for event in rx.listen() {
        let w_event = event.unwrap();

        let bar : u32 = runner::launch_polybar(String::from("title"));
        
        match w_event {
            Event::WindowEvent(inner_window_event) => {
                let info : WindowChange = inner_window_event.change;
                match info {
                    New | Close | Focus | FullscreenMode =>  {
                        runner::bar_command(bar, String::from("show"));
                        thread::sleep(Duration::from_secs(2));
                        runner::bar_command(bar, String::from("hide"));
                    },
                    _ => ()
                }
            },
            _ => ()

        }
    }
}

