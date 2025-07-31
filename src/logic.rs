use i3ipc::{I3EventListener, Subscription, event::{Event, inner::{WindowChange::{self, *} } } };
use crate::{runner};
use std::{process::Child, thread, time::Duration};
use std::sync::RwLock;

pub static POLYBAR_PID : RwLock<u32> = RwLock::new(0);
static EVENT_LIST : RwLock<Vec<u32>> = RwLock::new(Vec::new());

pub fn run() {
    let bar : Child = runner::launch_polybar(String::from("title"));

    runner::bar_command(bar.id(), String::from("hide"));

    let mut rx : I3EventListener = I3EventListener::connect().unwrap();

    rx.subscribe(&[Subscription::Window]).unwrap();

    for event in rx.listen() {
        update_polybar_pid(&bar);

        thread::spawn(move || {
            event_handler(event.unwrap());
        });

        thread::spawn(move ||{
            let pid = POLYBAR_PID.read().unwrap();    
        
            visibility_handler(*pid);
        });
    }

}

fn update_polybar_pid(bar : &Child) {
    let mut lock = POLYBAR_PID.write().unwrap();
    *lock = bar.id();
    drop(lock);
}



fn event_handler(event: Event) {
    match event {
        Event::WindowEvent(inner_window_event) => {
            let info : WindowChange = inner_window_event.change;
            match info {
                New | Close | Focus | FullscreenMode =>  {
                    let mut lock = EVENT_LIST.write().unwrap();
                    (*lock).push(0);
                    drop(lock);

                    thread::sleep(Duration::from_secs(2));

                    let mut lock = EVENT_LIST.write().unwrap();
                    (*lock).pop();
                    drop(lock);

                },
                _ => ()
            }
        },
        _ => ()

    }
}

pub fn visibility_handler(bar: u32) {
    if (*EVENT_LIST.read().unwrap()).len() > 0 {
        runner::bar_command(bar, String::from("show"));
        return;
    }
    
    runner::bar_command(bar, String::from("hide"));
}
