#![windows_subsystem="windows"]

extern crate native_windows_gui as nwg;

use sha2::{Sha256, Digest};
use std::env;
use std::fs::File;
use std::io;
use std::rc::Rc;


fn gen_hash(path: String) -> String {
    let mut sha256 = Sha256::new();

    let mut file = File::open(path).expect("Error");

    io::copy(&mut file, &mut sha256).expect("Error");

    let hash: String = format!("{:X}", sha256.finalize());

    return hash;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let hash = gen_hash(args[0].to_string());

    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let mut window = Default::default();
    let mut text_box = Default::default();
    let mut hash_box = Default::default();

    nwg::Window::builder()
        .size((700, 40))
        .center(true)
        .title("SHA-256")
        .build(&mut window)
        .unwrap();

    nwg::Label::builder()
        .text("SHA-256:")
        .position((10,10))
        .parent(&window)
        .build(&mut text_box)
        .unwrap();

    nwg::TextInput::builder()
        .text(&*hash)
        .size((600, 20))
        .position((85,10))
        .parent(&window)
        .build(&mut hash_box)
        .unwrap();

    let window = Rc::new(window);
    let events_window = window.clone();

    let handler = nwg::full_bind_event_handler(&window.handle, move |evt, _evt_data, handle| {
        use nwg::Event as E;

        match evt {
            E::OnWindowClose =>
                if &handle == &events_window as &nwg::Window {
                    nwg::stop_thread_dispatch();
                },
            _ => {}
        }
    });

    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&handler);
    std::process::exit(0);

}
