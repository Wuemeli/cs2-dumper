/*
 * Created using https://github.com/a2x/cs2-dumper
 * Fri, 3 Nov 2023 09:09:53 +0000
 */

#![allow(non_snake_case, non_upper_case_globals)]

pub mod CAnimScriptBase {
    pub const m_bIsValid: usize = 0x8; // bool
}

pub mod EmptyTestScript { // CAnimScriptBase
    pub const m_hTest: usize = 0x10; // CAnimScriptParam<float>
}