use rust_hdl_core::clock::Clock;
use rust_hdl_core::direction::{In, Out};
use rust_hdl_core::logic::Logic;
use rust_hdl_core::signal::Signal;
use rust_hdl_core::synth::Synth;
use rust_hdl_macros::LogicBlock;
use rust_hdl_core::ast::Verilog;

#[derive(Clone, Debug, LogicBlock)]
pub struct DFF<T: Synth> {
    pub d: Signal<In, T>,
    pub q: Signal<Out, T>,
    pub clk: Signal<In, Clock>,
}

impl<T: Synth> Default for DFF<T> {
    fn default() -> DFF<T> {
        Self::new(T::default())
    }
}

impl<T: Synth> DFF<T> {
    pub fn new(init: T) -> DFF<T> {
        Self {
            d: Signal::default(),
            q: Signal::new_with_default(init), // This should be marked as a register, since we write to it on a clock edge
            clk: Signal::default(),
        }
    }
}

impl<T: Synth> Logic for DFF<T> {
    fn update(&mut self) {
        if self.clk.pos_edge() {
            self.q.next = self.d.val()
        }
    }
    fn hdl(&self) -> Verilog {
        Verilog::Custom("always @(posedge clk) q <= d;".to_owned())
    }
    fn connect(&mut self) {
        self.q.connect();
    }
}
