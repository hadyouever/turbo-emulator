use jit::main::DataDesc;
use crate::common::TLSJitVar;
use crate::riscv::common::Xlen;
use crate::riscv::interpreter::main::RiscvInt;

pub struct RiscvJit {
    pub cpu: RiscvInt,
    pub jit: TLSJitVar,
}
impl RiscvJit {
    pub fn sign_ext(&mut self, tempreg: DataDesc) {
        match self.cpu.xlen {
            Xlen::X32 => {
                self.jit.extsw(tempreg, tempreg);
            }
            Xlen::X64 => {

            }
        }
    }
}