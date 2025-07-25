use i3ipc::{I3EventListener,
Subscription,
event::{Event, inner::WindowChange}};

use polybartender;

fn main() {
    println!("{}", polybartender::runner::launch_polybar(vec![String::from("dummyb"), String::from("titleb")])); 

    let mut rx : I3EventListener = I3EventListener::connect().unwrap();

    rx.subscribe(&[Subscription::Window]).unwrap();
    for event in rx.listen() {
        let w_event = event.unwrap();
        
        match w_event {
            Event::WindowEvent(inner_window_event) => {
                let info : WindowChange = inner_window_event.change;
                let mut response : String = String::from("No changes happened");
                match info {
                    WindowChange::New => response = String::from("New window opened"),
                    WindowChange::Close => response = String::from("Window Closed"),
                    WindowChange::Focus => response = String::from("Window has gained focus"),
                    WindowChange::FullscreenMode => response = String::from("An window has become fullscreen"),
                    _ => ()
                }
                println!("{response}");
            },
            _ => ()

        }
    }
}

