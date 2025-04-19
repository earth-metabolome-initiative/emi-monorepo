use algebra::prelude::SquareCSR2D;
use graph::prelude::{GenericGraph, SinkNodes};
use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|csr: GenericGraph<u8, SquareCSR2D<u16, u8>>| {
            let _sink_nodes = csr.sink_nodes();
        });
    }
}
