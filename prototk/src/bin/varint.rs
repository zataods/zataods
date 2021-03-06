use prototk::Packable;

fn main() {
    for argument in std::env::args().skip(1) {
        let x = match argument.parse::<u64>() {
            Ok(x) => x,
            Err(e) => {
                println!("don't know how to parse {}: {}", argument, e);
                continue;
            },
        };
        let v: prototk::v64 = x.into();
        let mut pirate = [0u8; 10];
        let pirate: &mut [u8] = &mut pirate;
        let pa = prototk::stack_pack(v);
        pa.into_slice(pirate);
        println!("{:?}", &pirate[..pa.pack_sz()]);
    }
}
