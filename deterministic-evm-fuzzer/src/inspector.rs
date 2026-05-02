use revm::{
    interpreter::{Interpreter, InstructionResult},
    Inspector, EvmContext, Database,
};
use alloy_primitives::U256;
use ahash::AHasher;
use std::hash::Hasher;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TraceStep {
    pub pc: usize,
    pub op: u8,
    pub stack_hash: u64,
}

pub struct DeterministicInspector {
    pub traces: Vec<TraceStep>,
    pub max_steps: usize,
}

impl DeterministicInspector {
    pub fn new(max_steps: usize) -> Self {
        Self {
            traces: Vec::with_capacity(1000),
            max_steps,
        }
    }
}

impl<DB: Database> Inspector<DB> for DeterministicInspector {
    #[inline]
    fn step(&mut self, interp: &mut Interpreter, _context: &mut EvmContext<DB>) {
        if self.traces.len() >= self.max_steps {
            return;
        }

        // Hash top 16 elements of the stack for deterministic state tracking
        let mut hasher = AHasher::default();
        let stack = interp.stack();
        let len = stack.len();
        let start = if len > 16 { len - 16 } else { 0 };
        
        for i in start..len {
            let val = stack.peek(i).unwrap();
            hasher.write(&val.to_be_bytes::<32>());
        }

        let step = TraceStep {
            pc: interp.program_counter(),
            op: interp.current_opcode(),
            stack_hash: hasher.finish(),
        };

        self.traces.push(step);
    }
}
