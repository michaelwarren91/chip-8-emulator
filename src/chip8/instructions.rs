use std::fmt;

pub enum Instruction {
    Unknown {
        opcode: u16,
    },
    SystemCall {
        address: u16,
    },
    ClearScreen,
    Return,
    Jump {
        address: u16,
    },
    Call {
        address: u16,
    },
    SkipIfRegisterEqualsValue {
        register: u8,
        value: u8,
    },
    SkipIfRegisterNotEqualsValue {
        register: u8,
        value: u8,
    },
    SkipIfRegistersEqual {
        register_a: u8,
        register_b: u8,
    },
    LoadByte {
        register: u8,
        value: u8,
    },
    AddByte {
        register: u8,
        value: u8,
    },
    CopyRegister {
        destination_register: u8,
        source_register: u8,
    },
    Or {
        destination_register: u8,
        source_register: u8,
    },
    And {
        destination_register: u8,
        source_register: u8,
    },
    Xor {
        destination_register: u8,
        source_register: u8,
    },
    AddRegisters {
        destination_register: u8,
        source_register: u8,
    },
    SubtractRegisters {
        destination_register: u8,
        source_register: u8,
    },
    ShiftRight {
        destination_register: u8,
    },
    SubtractFromRegister {
        destination_register: u8,
        source_register: u8,
    },
    ShiftLeft {
        destination_register: u8,
    },
    SkipIfRegistersNotEqual {
        register_a: u8,
        register_b: u8,
    },
    LoadIndexRegister {
        address: u16,
    },
    JumpWithOffset {
        address: u16,
    },
    RandomByte {
        destination_register: u8,
        mask: u8,
    },
    DrawSprite {
        x_register: u8,
        y_register: u8,
        height: u8,
    },
    SkipIfKeyPressed {
        register: u8,
    },
    SkipIfKeyNotPressed {
        register: u8,
    },
    ReadDelayTimer {
        destination_register: u8,
    },
    WaitForKeyPress {
        destination_register: u8,
    },
    SetDelayTimer {
        source_register: u8,
    },
    SetSoundTimer {
        source_register: u8,
    },
    AddToIndexRegister {
        source_register: u8,
    },
    LoadFontAddress {
        register: u8,
    },
    StoreBcd {
        source_register: u8,
    },
    StoreRegisters {
        final_register: u8,
    },
    LoadRegisters {
        final_register: u8,
    },
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown { opcode } => write!(f, "UNKNOWN opcode={opcode}"),
            Self::SystemCall { address } => write!(f, "SYS 0x{:X}", address),
            Self::ClearScreen => write!(f, "CLS"),
            Self::Return => write!(f, "RET"),
            Self::Jump { address } => write!(f, "JP 0x{:X}", address),
            Self::Call { address } => write!(f, "CALL 0x{:X}", address),
            Self::SkipIfRegisterEqualsValue { register, value } => {
                write!(f, "SE V{:X}, 0x{:X}", register, value)
            }
            Self::SkipIfRegisterNotEqualsValue { register, value } => {
                write!(f, "SNE V{:X}, 0x{:X}", register, value)
            }
            Self::SkipIfRegistersEqual {
                register_a,
                register_b,
            } => write!(f, "SE V{:X}, V:{:X}", register_a, register_b),
            Self::LoadByte { register, value } => write!(f, "LD V{:X}, 0x{:X}", register, value),
            Self::AddByte { register, value } => write!(f, "ADD V{:X}, 0x{:X}", register, value),
            Self::CopyRegister {
                source_register,
                destination_register,
            } => write!(f, "LD V{:X}, V{:X}", source_register, destination_register),
            Self::Or {
                source_register,
                destination_register,
            } => write!(f, "OR V{:X}, V{:X}", source_register, destination_register),
            Self::And {
                source_register,
                destination_register,
            } => write!(f, "AND V{:X}, V{:X}", source_register, destination_register),
            Self::Xor {
                source_register,
                destination_register,
            } => write!(f, "XOR V{:X}, V{:X}", source_register, destination_register),
            Self::AddRegisters {
                source_register,
                destination_register,
            } => write!(f, "ADD V{:X}, V{:X}", source_register, destination_register),
            Self::SubtractRegisters {
                source_register,
                destination_register,
            } => write!(f, "SUB V{:X}, V{:X}", source_register, destination_register),
            Self::ShiftRight {
                destination_register,
            } => write!(f, "SHR V{:X}", destination_register),
            Self::SubtractFromRegister {
                source_register,
                destination_register,
            } => write!(
                f,
                "SUBN V{:X}, V{:X}",
                source_register, destination_register
            ),
            Self::ShiftLeft {
                destination_register,
            } => write!(f, "SHL V{:X}", destination_register),
            Self::SkipIfRegistersNotEqual {
                register_a,
                register_b,
            } => write!(f, "SNE V{:X}, V{:X}", register_a, register_b),
            Self::LoadIndexRegister { address } => write!(f, "LD I, 0x{:X}", address),
            Self::JumpWithOffset { address } => write!(f, "JP V0, 0x{:X}", address),
            Self::RandomByte {
                destination_register,
                mask,
            } => write!(f, "RND V{:X}, 0x{:X}", destination_register, mask),
            Self::DrawSprite {
                x_register,
                y_register,
                height,
            } => write!(
                f,
                "DRW V{:X}, V{:X}, 0x{:X}",
                x_register, y_register, height
            ),
            Self::SkipIfKeyPressed { register } => write!(f, "SKP V{:X}", register),
            Self::SkipIfKeyNotPressed { register } => write!(f, "SKNP V{:X}", register),
            Self::ReadDelayTimer {
                destination_register,
            } => write!(f, "LD V{:X}, DT", destination_register),
            Self::WaitForKeyPress {
                destination_register,
            } => write!(f, "LD V{:X}, K", destination_register),
            Self::SetDelayTimer { source_register } => write!(f, "LD DT, V{:X}", source_register),
            Self::SetSoundTimer { source_register } => write!(f, "LD ST, V{:X}", source_register),
            Self::AddToIndexRegister { source_register } => {
                write!(f, "ADD I, V{:X}", source_register)
            }
            Self::LoadFontAddress { register } => write!(f, "LD F, V{:X}", register),
            Self::StoreBcd { source_register } => write!(f, "LD B, V{:X}", source_register),
            Self::StoreRegisters { final_register } => write!(f, "LD [I], V{:X}", final_register),
            Self::LoadRegisters { final_register } => write!(f, "LD V{:X}, [I]", final_register),
        }
    }
}
