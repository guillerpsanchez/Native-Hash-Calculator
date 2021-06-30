#![windows_subsystem="windows"]

extern crate native_windows_gui as nwg;

use sha2::{Sha256, Digest};
use blake3;
use std::env;
use std::fs::File;
use std::io;
use std::rc::Rc;
use hex;

fn gen_hash_sha256(path: String) -> String {
    let mut sha256 = Sha256::new();

    let mut file = File::open(path).expect("Error");

    io::copy(&mut file, &mut sha256).expect("Error");

    let hash: String = format!("{:x}", sha256.finalize());

    return hash;
}

fn gen_hash_blake3(path: String) -> std::string::String {
    let mut blake3 = blake3::Hasher::new();

    let mut file = File::open(path).expect("Error");

    io::copy(&mut file, &mut blake3).expect("Error");

    let hash = format!("{}", hex::encode(blake3.finalize()).to_string());

    return hash;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let hash_sha256 = gen_hash_sha256(args[1].to_string());
    let hash_blake3 = gen_hash_blake3(args[1].to_string());
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Courier New").expect("Failed to set default font");

    let mut window = Default::default();
    let mut text_box_sha256 = Default::default();
    let mut hash_box_sha256 = Default::default();
    let mut text_box_blake3 = Default::default();
    let mut hash_box_blake3 = Default::default();

    nwg::Window::builder()
        .size((750, 70))
        .center(true)
        .title("Native-Hash-Calculator")
        .build(&mut window)
        .unwrap();

    nwg::Label::builder()
        .text("SHA-256:")
        .position((10,10))
        .parent(&window)
        .build(&mut text_box_sha256)
        .unwrap();


    nwg::TextInput::builder()
        .text(&*hash_sha256)
        .size((650, 20))
        .position((95,10))
        .parent(&window)
        .build(&mut hash_box_sha256)
        .unwrap();

    nwg::Label::builder()
        .text("Blake3:")
        .position((10,40))
        .parent(&window)
        .build(&mut text_box_blake3)
        .unwrap();


    nwg::TextInput::builder()
        .text(&*hash_blake3)
        .size((650, 20))
        .position((95,40))
        .parent(&window)
        .build(&mut hash_box_blake3)
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
