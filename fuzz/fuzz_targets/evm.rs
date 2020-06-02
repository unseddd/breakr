#![no_main]
use libfuzzer_sys::fuzz_target;

use std::{fs::File, io::BufReader, rc::Rc};

use evm::{Capture, Machine};

const DISP_PAT: [u8; 2] = [0x80, 0x6d];

fn find_dispatch(msg: &[u8]) -> Option<usize> {
    msg.windows(DISP_PAT.len()).position(|window| window == DISP_PAT)
}

fn get_signatures(msg: &[u8]) -> Vec<Vec<u8>> {
    let mut ret: Vec<Vec<u8>> = vec![];
    let mut m = msg.to_vec();

    while !m.is_empty() {
        match find_dispatch(&msg) {
            Some(n) => {
                let start = n + DISP_PAT.len();
                let end = start + 4;

                ret.push(m[start .. end].to_vec());
                m = m.drain(..end).collect();
            },
            None => m = m.drain(..).collect(),
        }
    }

    ret
}

fuzz_target!(|data: &[u8]| {
    let contract_file = File::open("fuzz/contracts/fuzz.bin")
        .expect("error opening contract file");

    let contract = BufReader::new(contract_file);

    let buf = contract.buffer();
    let sigs = get_signatures(&buf);

    for s in sigs.iter() {
        let mut code = s.clone();
        code.extend_from_slice(&data);

        let mut vm = Machine::new(Rc::new(buf.to_vec()), Rc::new(code), 1024, 10000);

        if let Capture::Exit(n) = vm.run() {
            if n != evm::ExitSucceed::Returned.into() {
                panic!("Error: sig: {:?}, code: {:?}, reason: {:?}", &s, &data, &n)
            }
        }
    }
});
