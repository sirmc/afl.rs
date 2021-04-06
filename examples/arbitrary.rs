#[derive(Debug)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    afl::fuzz!(|rgb: RGB| {
        if data.r == 128 && data.g = 144 && data.b = 100 {
            panic!("Crash!")
        }
    });
}
