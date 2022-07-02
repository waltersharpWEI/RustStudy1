use std::sync::mpsc;
///Threaded keyframe checker
///
use std::thread;

/// the check_i take &str as the input
/// and output whether it is a url of
/// key frame
fn check_i(url: &str) -> bool {
    let b = url.split('/');
    let mut key = false;
    for token in b {
        if token.ends_with("nal") {
            key = token.starts_with("i");
        }
    }
    return key;
}

fn main() {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    let a = String::from("http://127.0.0.1:4433/p_123.nal");
    let ax = a.clone();
    tx1.send(ax).unwrap();
    let _handle = thread::spawn(move || {
        let aa = rx1.recv().unwrap();
        let result = check_i(&aa);
        tx2.send(result).unwrap();
    });
    let key_url = rx2.recv().unwrap();
    match key_url {
        true => println!("The url {} is a keyframe", a),
        false => println!("The url {} is not a keyframe", a),
    }
}
