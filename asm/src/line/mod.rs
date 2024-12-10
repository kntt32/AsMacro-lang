use super::ml_gen::*;
use super::*;
use crate::ml_gen::raw_encoder::{ModRmMode, RexMode};
use crate::registers::Register;
use line_parser::{get_reg64_str, get_rm64_ref_str};
use util::functions::stoi;
use util::functions::{get_inner_expr, match_str, MatchStr};
use util::svec::SVec;

mod line_parser;

#[derive(Clone, Copy, Debug)]
pub struct Line<'a> {
    label: Option<&'a str>,
    ops: Option<(Operator, SVec<2, &'a str>)>, // (operator, operands)
}

impl<'a> Line<'a> {
    /*
    pub fn opecode(&self) -> Option<SVec<3, u8>> {
        Some(self.ops?.0.encoding_rule.opecode)
    }

    pub fn rex_mode(&self) -> Option<RexMode> {
        Some(self.ops?.0.encoding_rule.rex)
    }

    pub fn modrm_mode(&self) -> Option<ModRmMode> {
        Some(
            match self.ops?.0.encoding_rule.modrm_mode
        )
    }*/
}

#[derive(Clone, Copy, Debug)]
pub struct RowLine<'a> {
    label: Option<&'a str>,
    mnemonic: Option<&'a str>,
    operands: SVec<2, &'a str>,
}

impl<'a> RowLine<'a> {
    pub fn new(
        label: Option<&'a str>,
        mnemonic: Option<&'a str>,
        operands: SVec<2, &'a str>,
    ) -> Self {
        RowLine {
            label: label,
            mnemonic: mnemonic,
            operands: operands,
        }
    }

    pub fn to_line(&self, operators_list: &[Operator]) -> Option<Line<'a>> {
        if self.mnemonic.is_some() {
            Some(Line {
                label: self.label,
                ops: Some((
                    operators_list[self.get_operation_index(operators)?],
                    self.operands,
                )),
            })
        } else {
            Some(Line {
                label: self.label,
                ops: None,
            })
        }
    }

    pub fn get_operation_index(self, operators_list: &[Operator]) -> Option<usize> {
        for i in 0..operators_list.len() {
            if self.mnemonic.is_some()
                && self.mnemonic? == operators_list[i].mnemonic
                && operators_list[i].operands.len() == self.operands.len()
            {
                let mut flag = true;
                for k in 0..operators_list[i].operands.len() {
                    if !operators_list[i].operands[k].is_match(self.operands[k]) {
                        flag = false;
                        break;
                    }
                }
                if flag {
                    return Some(i);
                }
            }
        }
        return None;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Operator {
    mnemonic: &'static str,
    operands: SVec<2, OperandType>,
    encoding_rule: Rule,
}

pub static operators: &[Operator] = &[
    Operator {
        mnemonic: "mov",
        operands: SVec::value([OperandType::Reg64, OperandType::Imm64], 2),
        encoding_rule: Rule {
            opecode: SVec::value([0xb8, 0, 0], 1),
            rex: RexRule::RexW,
            modrm: ModRmRule::None,
            imm: ImmRule::Id,
            add_reg: AddRegRule::Rd,
        },
    },
    Operator {
        mnemonic: "mov",
        operands: SVec::value([OperandType::Reg64, OperandType::Rm64], 2),
        encoding_rule: Rule {
            opecode: SVec::value([0x8b, 0, 0], 1),
            rex: RexRule::RexW,
            modrm: ModRmRule::R,
            imm: ImmRule::None,
            add_reg: AddRegRule::None,
        },
    }, //50+rd PUSH r64
    Operator {
        mnemonic: "push",
        operands: SVec::value([OperandType::Reg64, OperandType::None], 1),
        encoding_rule: Rule {
            opecode: SVec::value([0x50, 0, 0], 1),
            rex: RexRule::None,
            modrm: ModRmRule::None,
            imm: ImmRule::None,
            add_reg: AddRegRule::Rd,
        },
    }, //REX.W + 58+ rd POP r64
    Operator {
        mnemonic: "pop",
        operands: SVec::value([OperandType::Reg64, OperandType::None], 1),
        encoding_rule: Rule {
            opecode: SVec::value([0x58, 0, 0], 1),
            rex: RexRule::RexW,
            modrm: ModRmRule::None,
            imm: ImmRule::None,
            add_reg: AddRegRule::Rd,
        },
    }, //C3 RET
    Operator {
        mnemonic: "ret",
        operands: SVec::value([OperandType::None, OperandType::None], 0),
        encoding_rule: Rule {
            opecode: SVec::value([0xc3, 0, 0], 1),
            rex: RexRule::None,
            modrm: ModRmRule::None,
            imm: ImmRule::None,
            add_reg: AddRegRule::None,
        },
    },
];

#[derive(Clone, Copy, Default, Debug)]
enum OperandType {
    #[default]
    None,
    Imm64,
    Reg64,
    Rm64,
}

impl OperandType {
    fn is_match(self, expr: &str) -> bool {
        match self {
            OperandType::None => {
                if expr.is_empty() {
                    true
                } else {
                    false
                }
            }
            OperandType::Imm64 => stoi(expr).is_some(),
            OperandType::Reg64 => get_reg64_str(expr).is_some(),
            OperandType::Rm64 => get_reg64_str(expr).is_some() || get_rm64_ref_str(expr).is_some(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Rule {
    pub opecode: SVec<3, u8>,
    pub rex: RexRule,
    pub modrm: ModRmRule,
    pub imm: ImmRule,
    pub add_reg: AddRegRule,
}

pub type RexRule = RexMode;

#[derive(Clone, Copy, Debug)]
pub enum ModRmRule {
    None,
    R,
    Dight(u8),
}

#[derive(Clone, Copy, Debug)]
pub enum ImmRule {
    None,
    Ib,
    Iw,
    Id,
    Io,
}

#[derive(Clone, Copy, Debug)]
pub enum AddRegRule {
    None,
    Rb,
    Rw,
    Rd,
    Ro,
}
