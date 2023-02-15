use jit::main::DataDesc;
use crate::riscv::common::{RiscvArgs, Xlen};
use crate::riscv::interpreter::main::{RiscvInt};
use crate::riscv::jit::main::RiscvJit;
#[derive(Debug,Copy, Clone,Eq, PartialEq)]
enum RiscvArithOp {
    Add,
    Sub,
    Shl,
    Xor,
    Or,
    And,
    Srl, // right logical
    Sra // right immediate

}

pub fn sign_ext_imm(val: u32) -> u64 {
    // For "I" decoded instructions.
    // decoder should take care of this up to 32 bit. For 64 bit, on our own.
    val as i32 as i64 as u64
}
pub fn xor(rj: &mut RiscvJit, args: &RiscvArgs) {
    let rs1 = rj.jit.get_perm_reg_desc(args.rs1 as usize).unwrap();
    let rs2 = rj.jit.get_perm_reg_desc(args.rs2 as usize).unwrap();
    let rd = rj.jit.get_perm_reg_desc(args.rd as usize).unwrap();
    rj.jit.xor(rd, rs1, rs2);
}
pub fn xori(rj: &mut RiscvJit, args: &RiscvArgs) {
    let rs1 = rj.jit.get_perm_reg_desc(args.rs1 as usize).unwrap();
    let imm = DataDesc::new_imm(sign_ext_imm(args.imm));
    let rd = rj.jit.get_perm_reg_desc(args.rd as usize).unwrap();
    rj.jit.xor(rd, rs1, imm);
}
pub fn or(rj: &mut RiscvJit, args: &RiscvArgs) {
    let rs1 = rj.jit.get_perm_reg_desc(args.rs1 as usize).unwrap();
    let rs2 = rj.jit.get_perm_reg_desc(args.rs2 as usize).unwrap();
    let rd = rj.jit.get_perm_reg_desc(args.rd as usize).unwrap();
    rj.jit.or(rd, rs1, rs2);
}
pub fn ori(rj: &mut RiscvJit, args: &RiscvArgs) {
    let rs1 = rj.jit.get_perm_reg_desc(args.rs1 as usize).unwrap();
    let imm = DataDesc::new_imm(sign_ext_imm(args.imm));
    let rd = rj.jit.get_perm_reg_desc(args.rd as usize).unwrap();
    rj.jit.or(rd, rs1, imm);
}

pub fn and(rj: &mut RiscvJit, args: &RiscvArgs) {
    let rs1 = rj.jit.get_perm_reg_desc(args.rs1 as usize).unwrap();
    let rs2 = rj.jit.get_perm_reg_desc(args.rs2 as usize).unwrap();
    let rd = rj.jit.get_perm_reg_desc(args.rd as usize).unwrap();
    rj.jit.and(rd, rs1, rs2);
}

pub fn andi(rj: &mut RiscvJit, args: &RiscvArgs) {
    let rs1 = rj.jit.get_perm_reg_desc(args.rs1 as usize).unwrap();
    let imm = DataDesc::new_imm(sign_ext_imm(args.imm));
    let rd = rj.jit.get_perm_reg_desc(args.rd as usize).unwrap();
    rj.jit.and(rd, rs1, imm);
}

pub fn add(rj: &mut RiscvJit, args: &RiscvArgs) {
    let rs1 = rj.jit.get_perm_reg_desc(args.rs1 as usize).unwrap();
    let rs2 = rj.jit.get_perm_reg_desc(args.rs2 as usize).unwrap();
    let rd = rj.jit.get_perm_reg_desc(args.rd as usize).unwrap();

    let tempdst = rj.jit.create_temp().unwrap();
    rj.jit.add(tempdst, rs1, rs2);
    rj.sign_ext(tempdst);
    rj.jit.mov(rd, tempdst);
    rj.jit.free_temp(tempdst).unwrap();

}
pub fn addi(rj: &mut RiscvJit, args: &RiscvArgs) {
    // because rs1 and rd are5 bits, should never go above 32 (uneless issue with decode)
    // overlfow doesnt matter
    let imm = DataDesc::new_imm(sign_ext_imm(args.imm));
    let rs1 = rj.jit.get_perm_reg_desc(args.rs1 as usize).unwrap();
    let rd = rj.jit.get_perm_reg_desc(args.rd as usize).unwrap();
    rj.jit.add(rd, rs1, imm);
    rj.sign_ext(rd);

}
pub fn addw(rj: &mut RiscvJit, args: &RiscvArgs) {
    let rs1 = rj.jit.get_perm_reg_desc(args.rs1 as usize).unwrap();
    let rs2 = rj.jit.get_perm_reg_desc(args.rs2 as usize).unwrap();
    let rd = rj.jit.get_perm_reg_desc(args.rd as usize).unwrap();
    rj.jit.add(rd, rs1, rs2);
    rj.jit.extsw(rd, rd);

}

pub fn sub(rj: &mut RiscvJit, args: &RiscvArgs) {
    let rs1 = rj.jit.get_perm_reg_desc(args.rs1 as usize).unwrap();
    let rs2 = rj.jit.get_perm_reg_desc(args.rs2 as usize).unwrap();
    let rd = rj.jit.get_perm_reg_desc(args.rd as usize).unwrap();
    rj.jit.sub(rd, rs1, rs2);
    rj.sign_ext(rd);

}
pub fn subw(rj: &mut RiscvJit, args: &RiscvArgs) {
    let rs1 = rj.jit.get_perm_reg_desc(args.rs1 as usize).unwrap();
    let rs2 = rj.jit.get_perm_reg_desc(args.rs2 as usize).unwrap();
    let rd = rj.jit.get_perm_reg_desc(args.rd as usize).unwrap();
    rj.jit.sub(rd, rs1, rs2);
    rj.jit.extsw(rd, rd);

}

pub fn mul(rj: &mut RiscvJit, args: &RiscvArgs) {
    let rs1 = rj.jit.get_perm_reg_desc(args.rs1 as usize).unwrap();
    let rs2 = rj.jit.get_perm_reg_desc(args.rs2 as usize).unwrap();
    let rd = rj.jit.get_perm_reg_desc(args.rd as usize).unwrap();
    rj.jit.mul64l(rd, rs1, rs2);
    rj.sign_ext(rd);

}

pub fn mulh(rj: &mut RiscvJit, args: &RiscvArgs) {
    let rs1 = rj.jit.get_perm_reg_desc(args.rs1 as usize).unwrap();
    let rs2 = rj.jit.get_perm_reg_desc(args.rs2 as usize).unwrap();
    let rd = rj.jit.get_perm_reg_desc(args.rd as usize).unwrap();
    if rj.cpu.xlen == Xlen::X32 {
        rj.jit.mul64l(rd, rs1, rs2);
        rj.jit.shr(rd, rd, DataDesc::new_imm(32));
        rj.sign_ext(rd);
    } else {
        rj.jit.smul64h(rd, rs1, rs2);
    }

}
pub fn mulhu(rj: &mut RiscvJit, args: &RiscvArgs) {
    let rs1 = rj.jit.get_perm_reg_desc(args.rs1 as usize).unwrap();
    let rs2 = rj.jit.get_perm_reg_desc(args.rs2 as usize).unwrap();
    let rd = rj.jit.get_perm_reg_desc(args.rd as usize).unwrap();
    if rj.cpu.xlen == Xlen::X32 {
        rj.jit.umul32h(rd, rs1, rs2);
        rj.sign_ext(rd);
    } else {
        rj.jit.umul64h(rd, rs1, rs2);
    }
}
/*
pub fn mulhsu(rj: &mut RiscvJit, args: &RiscvArgs) {
    let rs1 = rj.jit.get_perm_reg_desc(args.rs1 as usize).unwrap();
    let rs2 = rj.jit.get_perm_reg_desc(args.rs2 as usize).unwrap();
    let rd = rj.jit.get_perm_reg_desc(args.rd as usize).unwrap();
    if rj.cpu.xlen == Xlen::X32 {
        let temp1 = rj.jit.create_temp().unwrap();
        rj.jit.mov(temp1, rs2);
        rj.jit.extuw(temp1, temp1);
        rj.jit.mul64l(rd, rs1, temp1);
        rj.jit.shr(rd, rd, DataDesc::new_imm(32));
        rj.sign_ext(rd);
        rj.jit.free_temp(temp1);
    } else {
        rj.jit.umul64h(rd, rs1, rs2);
    }
    ri.regs[args.rd as usize] = match ri.xlen {
        Xlen::X32 => {
            ri.sign_ext(((ri.regs[args.rs1 as usize] as i64).wrapping_mul(ri.regs[args.rs2 as usize] as u32 as i64) >> 32) as u64)
        },
        Xlen::X64 => {
            ((ri.regs[args.rs1 as usize] as u128).wrapping_mul(ri.regs[args.rs2 as usize] as u64 as u128) >> 64) as i64 as u64
        }
    } as u64;

}

 */
pub fn rem(rj: &mut RiscvJit, args: &RiscvArgs) {
    let dividend = ri.sign_ext(ri.regs[args.rs1 as usize]) as i64;
    let divisor = ri.sign_ext(ri.regs[args.rs2 as usize]) as i64;
    if divisor == 0 {
        ri.regs[args.rd as usize] = dividend as u64;
    } else if dividend == ri.most_negative() && divisor == -1 {
        ri.regs[args.rd as usize] = 0;
    } else {
        ri.regs[args.rd as usize] = ri.sign_ext(dividend.wrapping_rem(divisor) as u64) as u64;
    }

}

pub fn remu(rj: &mut RiscvJit, args: &RiscvArgs) {
    let dividend = ri.cull_reg(ri.regs[args.rs1 as usize]);
    let divisor = ri.cull_reg(ri.regs[args.rs2 as usize]);
    ri.regs[args.rd as usize] = match divisor {
        0 => ri.sign_ext(dividend),
        _ => ri.sign_ext(dividend.wrapping_rem(divisor))
    } as u64;

}

pub fn mulw(rj: &mut RiscvJit, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = ri.sign_ext((ri.regs[args.rs1 as usize] as i32).wrapping_mul(ri.regs[args.rs2 as usize] as i32) as u64) as u64;

}
pub fn div(rj: &mut RiscvJit, args: &RiscvArgs) {
    let dividend = ri.regs[args.rs1 as usize] as i64;
    let divisor = ri.regs[args.rs2 as usize] as i64;
    if divisor == 0 {
        ri.regs[args.rd as usize] = -1 as i64 as u64;
    } else if dividend == std::i64::MIN && divisor == -1 {
        ri.regs[args.rd as usize] = dividend  as u64;
    } else {
        ri.regs[args.rd as usize] = dividend.wrapping_div(divisor) as u64;
    }
}

pub fn divw(rj: &mut RiscvJit, args: &RiscvArgs) {
    let dividend = ri.regs[args.rs1 as usize] as i32;
    let divisor = ri.regs[args.rs2 as usize] as i32;
    if divisor == 0 {
        ri.regs[args.rd as usize] = -1 as i64 as u64;
    } else if dividend == std::i32::MIN && divisor == -1 {
        ri.regs[args.rd as usize] = dividend as i32 as i64 as u64;
    } else {
        ri.regs[args.rd as usize] = dividend.wrapping_div(divisor) as i32 as i64 as u64
    }

}
pub fn divu(rj: &mut RiscvJit, args: &RiscvArgs) {
    let dividend = ri.cull_reg(ri.regs[args.rs1 as usize]);
    let divisor = ri.cull_reg(ri.regs[args.rs2 as usize]);
    if divisor == 0 {
        ri.regs[args.rd as usize] = -1 as i64 as u64;
    } else {
        ri.regs[args.rd as usize] = ri.sign_ext(dividend.wrapping_div(divisor));
    }

}
pub fn divuw(rj: &mut RiscvJit, args: &RiscvArgs) {
    let dividend = ri.cull_reg(ri.regs[args.rs1 as usize]) as u32;
    let divisor = ri.cull_reg(ri.regs[args.rs2 as usize]) as u32;
    if divisor == 0 {
        ri.regs[args.rd as usize] = -1 as i64 as u64;
    } else {
        ri.regs[args.rd as usize] = dividend.wrapping_div(divisor) as i32 as i64 as u64;
    }

}

pub fn remw(rj: &mut RiscvJit, args: &RiscvArgs) {
    let dividend = ri.regs[args.rs1 as usize] as i32;
    let divisor = ri.regs[args.rs2 as usize] as i32;
    if divisor == 0 {
        ri.regs[args.rd as usize] = dividend as i64 as u64;
    } else if dividend == std::i32::MIN && divisor == -1 {
        ri.regs[args.rd as usize] = 0;
    } else {
        ri.regs[args.rd as usize] = dividend.wrapping_rem(divisor) as i64 as u64;
    }

}
pub fn remuw(rj: &mut RiscvJit, args: &RiscvArgs) {
    let dividend = ri.regs[args.rs1 as usize] as u32;
    let divisor = ri.regs[args.rs2 as usize] as u32;
    if divisor == 0 {
        ri.regs[args.rd as usize] = dividend as i32 as i64 as u64;
    }  else {
        ri.regs[args.rd as usize] = dividend.wrapping_rem(divisor) as i32 as i64 as u64;
    }

}
pub fn slli(rj: &mut RiscvJit, args: &RiscvArgs) {
    let shiftby: u64 = match ri.xlen {
        Xlen::X32 => {
            args.shamt & 0x1f // lower 5 bits
        },
        Xlen::X64 => {
            args.shamt & 0x3f // lower 6 bits
        }
    } as u64;
    ri.regs[args.rd as usize] = ri.sign_ext(ri.regs[args.rs1 as usize] << shiftby);

}

pub fn srli(rj: &mut RiscvJit, args: &RiscvArgs) {
    let shiftby: u64 = match ri.xlen {
        Xlen::X32 => {
            args.shamt & 0x1f // lower 5 bits
        },
        Xlen::X64 => {
            args.shamt & 0x3f // lower 6 bits
        }
    } as u64;
    ri.regs[args.rd as usize] = ri.sign_ext(ri.cull_reg(ri.regs[args.rs1 as usize]) >> shiftby);

}

pub fn srai(rj: &mut RiscvJit, args: &RiscvArgs) {
    let shiftby: u32 = match ri.xlen {
        Xlen::X32 => {
            args.shamt & 0x1f // lower 5 bits
        },
        Xlen::X64 => {
            args.shamt & 0x3f // lower 6 bits
        }
    };
    ri.regs[args.rd as usize] = match ri.xlen {
        Xlen::X64 => {
            let mut val: i64 = ri.regs[args.rs1 as usize] as i64;
            val = val >> (shiftby as i64);
            val as u64

        },
        Xlen::X32 => {
            let mut val: i32 = ri.regs[args.rs1 as usize] as i32;
            val = val >> (shiftby as i32);
            (val as i64) as u64 // we want it to extend all the way, just in case
        }
    };

}




pub fn sll(rj: &mut RiscvJit, args: &RiscvArgs) {
    let shiftby: u64 = match ri.xlen {
        Xlen::X32 => {
            ri.regs[args.rs2 as usize] & 0x1f // lower 5 bits
        },
        Xlen::X64 => {
            ri.regs[args.rs2 as usize] & 0x3f // lower 6 bits
        }
    };
    ri.regs[args.rd as usize] = ri.sign_ext(ri.regs[args.rs1 as usize].wrapping_shl(shiftby as u32));


}

pub fn slt(rj: &mut RiscvJit, args: &RiscvArgs) {
    let condition: bool = match ri.xlen {
        Xlen::X32 => {
            (ri.regs[args.rs1 as usize] as i32) < (ri.regs[args.rs2 as usize] as i32)
        }
        Xlen::X64 => {
            (ri.regs[args.rs1 as usize] as i64) < (ri.regs[args.rs2 as usize] as i64)
        }
    };
    ri.regs[args.rd as usize] = if condition {
        1
    } else {
        0
    };

}

pub fn sltu(rj: &mut RiscvJit, args: &RiscvArgs) {
    let condition: bool = match ri.xlen {
        Xlen::X32 => {
            (ri.regs[args.rs1 as usize] as u32) < (ri.regs[args.rs2 as usize] as u32)
        }
        Xlen::X64 => {
            (ri.regs[args.rs1 as usize] as u64) < (ri.regs[args.rs2 as usize] as u64)
        }
    };
    ri.regs[args.rd as usize] = if condition {
        1
    } else {
        0
    };

}


pub fn srl(rj: &mut RiscvJit, args: &RiscvArgs) {
    let shiftby: u64 = match ri.xlen {
        Xlen::X32 => {
            ri.regs[args.rs2 as usize] & 0x1f // lower 5 bits
        },
        Xlen::X64 => {
            ri.regs[args.rs2 as usize] & 0x3f // lower 6 bits
        }
    };
    ri.regs[args.rd as usize] = ri.sign_ext(ri.cull_reg(ri.regs[args.rs1 as usize]).wrapping_shr(shiftby as u32));

}

pub fn sra(rj: &mut RiscvJit, args: &RiscvArgs) {
    let shiftby: u64 = match ri.xlen {
        Xlen::X32 => {
            ri.regs[args.rs2 as usize] & 0x1f // lower 5 bits
        },
        Xlen::X64 => {
            ri.regs[args.rs2 as usize] & 0x3f // lower 6 bits
        }
    };
    ri.regs[args.rd as usize] = match ri.xlen {
        Xlen::X64 => {
            let mut val: i64 = ri.regs[args.rs1 as usize] as i64;
            val = val >> (shiftby as i64);
            val as u64

        },
        Xlen::X32 => {
            let mut val: i32 = ri.regs[args.rs1 as usize] as i32;
            val = val >> (shiftby as i32);
            (val as i64) as u64 // we want it to extend all the way, just in case
        }
    };

}
pub fn addiw(rj: &mut RiscvJit, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = ri.regs[args.rs1 as usize].wrapping_add(sign_ext_imm(args.imm)) as i32 as i64 as u64;

}

pub fn slliw(rj: &mut RiscvJit, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = (ri.regs[args.rs1 as usize].wrapping_shl(args.shamt)) as i32 as u64;

}

pub fn srliw(rj: &mut RiscvJit, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = ((ri.regs[args.rs1 as usize] as u32) >> args.shamt) as i32 as i64 as u64;

}

pub fn sraiw(rj: &mut RiscvJit, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = ((ri.regs[args.rs1 as usize] as i32) >> args.shamt) as i64 as u64;

}
pub fn sllw(rj: &mut RiscvJit, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = (ri.regs[args.rs1 as usize] as u32).wrapping_shl(ri.regs[args.rs2 as usize] as u32) as i32 as i64 as u64;

}

pub fn srlw(rj: &mut RiscvJit, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = (ri.regs[args.rs1 as usize] as u32).wrapping_shr(ri.regs[args.rs2 as usize] as u32) as i32 as i64 as u64;

}

pub fn sraw(rj: &mut RiscvJit, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = (ri.regs[args.rs1 as usize] as i32).wrapping_shr(ri.regs[args.rs2 as usize] as u32) as i64 as u64;

}


pub fn lui(rj: &mut RiscvJit, args: &RiscvArgs) {
    // will overwite lower 12 with 0s, and dtree does this already, so just assign
    ri.regs[args.rd as usize] = sign_ext_imm(args.imm); // see riscv atlas

}
pub fn slti(rj: &mut RiscvJit, args: &RiscvArgs) {
    let condition: bool = match ri.xlen {
        Xlen::X32 => {
            (ri.regs[args.rs1 as usize] as i32) < (args.imm as i32)
        }
        Xlen::X64 => {
            (ri.regs[args.rs1 as usize] as i64) < (sign_ext_imm(args.imm) as i64)
        }
    };
    ri.regs[args.rd as usize] = if condition {
        1
    } else {
        0
    };

}

pub fn sltiu(rj: &mut RiscvJit, args: &RiscvArgs) {
    let condition: bool = match ri.xlen {
        Xlen::X32 => {
            (ri.regs[args.rs1 as usize] as u32) < (args.imm as u32)
        }
        Xlen::X64 => {
            (ri.regs[args.rs1 as usize] as u64) < (sign_ext_imm(args.imm) as u64)
        }
    };
    ri.regs[args.rd as usize] = if condition {
        1
    } else {
        0
    };

}
// technically part of crypto extension, but put it here because it fits better
pub fn gen_sh_add(rj: &mut RiscvJit, args: &RiscvArgs, amt: u64, is_uw: bool) {
    let mut rs1 = if is_uw {
        ri.regs[args.rs1 as usize] as u32 as u64
    } else {
        ri.regs[args.rs1 as usize]
    };
    if is_uw && ri.xlen != Xlen::X64 {
        panic!(); // todo: illegal instr
    }
    rs1 <<= amt;
    let newval = rs1 + ri.regs[args.rs2 as usize];
    ri.regs[args.rd as usize] = ri.sign_ext(newval);

}
pub fn sh1add(rj: &mut RiscvJit, args: &RiscvArgs) {
    gen_sh_add(ri, args, 1, false);

}
pub fn sh1add_uw(rj: &mut RiscvJit, args: &RiscvArgs) {
    gen_sh_add(ri, args, 1, true);

}

pub fn sh2add(rj: &mut RiscvJit, args: &RiscvArgs) {
    gen_sh_add(ri, args, 2, false);

}
pub fn sh2add_uw(rj: &mut RiscvJit, args: &RiscvArgs) {
    gen_sh_add(ri, args, 2, true);

}

pub fn sh3add(rj: &mut RiscvJit, args: &RiscvArgs) {
    gen_sh_add(ri, args, 3, false);

}
pub fn sh3add_uw(rj: &mut RiscvJit, args: &RiscvArgs) {
    gen_sh_add(ri, args, 3, true);

}

pub fn add_uw(rj: &mut RiscvJit, args: &RiscvArgs) {
    gen_sh_add(ri, args, 0, true);

}
pub fn xnor(rj: &mut RiscvJit, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = ri.regs[args.rs1 as usize] ^ (!ri.regs[args.rs2 as usize]);

}