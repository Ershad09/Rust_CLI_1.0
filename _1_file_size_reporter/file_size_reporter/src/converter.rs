pub fn convert_size(bytes: u64) -> (f64, f64, f64) {
    let kb = bytes as f64 / 1024.0;
    let mb = kb / 1024.0;
    let gb = mb / 1024.0;

    (kb, mb, gb)
}