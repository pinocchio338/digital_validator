// automatically generated by the FlatBuffers compiler, do not modify

// @generated

use crate::common_generated::*;
use core::cmp::Ordering;
use core::mem;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

pub enum CompiledInstructionOffset {}
#[derive(Copy, Clone, PartialEq, Eq)]

pub struct CompiledInstruction<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for CompiledInstruction<'a> {
    type Inner = CompiledInstruction<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table::new(buf, loc),
        }
    }
}

impl<'a> CompiledInstruction<'a> {
    pub const VT_PROGRAM_ID_INDEX: flatbuffers::VOffsetT = 4;
    pub const VT_ACCOUNTS: flatbuffers::VOffsetT = 6;
    pub const VT_DATA: flatbuffers::VOffsetT = 8;

    #[inline]
    pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        CompiledInstruction { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args CompiledInstructionArgs<'args>,
    ) -> flatbuffers::WIPOffset<CompiledInstruction<'bldr>> {
        let mut builder = CompiledInstructionBuilder::new(_fbb);
        if let Some(x) = args.data {
            builder.add_data(x);
        }
        if let Some(x) = args.accounts {
            builder.add_accounts(x);
        }
        builder.add_program_id_index(args.program_id_index);
        builder.finish()
    }

    #[inline]
    pub fn program_id_index(&self) -> u8 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u8>(CompiledInstruction::VT_PROGRAM_ID_INDEX, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn accounts(&self) -> Option<flatbuffers::Vector<'a, u8>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(
                    CompiledInstruction::VT_ACCOUNTS,
                    None,
                )
        }
    }
    #[inline]
    pub fn data(&self) -> Option<flatbuffers::Vector<'a, u8>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(
                    CompiledInstruction::VT_DATA,
                    None,
                )
        }
    }
}

impl flatbuffers::Verifiable for CompiledInstruction<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<u8>("program_id_index", Self::VT_PROGRAM_ID_INDEX, false)?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>(
                "accounts",
                Self::VT_ACCOUNTS,
                false,
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>(
                "data",
                Self::VT_DATA,
                false,
            )?
            .finish();
        Ok(())
    }
}
pub struct CompiledInstructionArgs<'a> {
    pub program_id_index: u8,
    pub accounts: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
    pub data: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
}
impl<'a> Default for CompiledInstructionArgs<'a> {
    #[inline]
    fn default() -> Self {
        CompiledInstructionArgs {
            program_id_index: 0,
            accounts: None,
            data: None,
        }
    }
}

pub struct CompiledInstructionBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> CompiledInstructionBuilder<'a, 'b> {
    #[inline]
    pub fn add_program_id_index(&mut self, program_id_index: u8) {
        self.fbb_.push_slot::<u8>(
            CompiledInstruction::VT_PROGRAM_ID_INDEX,
            program_id_index,
            0,
        );
    }
    #[inline]
    pub fn add_accounts(&mut self, accounts: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
        self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
            CompiledInstruction::VT_ACCOUNTS,
            accounts,
        );
    }
    #[inline]
    pub fn add_data(&mut self, data: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(CompiledInstruction::VT_DATA, data);
    }
    #[inline]
    pub fn new(
        _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> CompiledInstructionBuilder<'a, 'b> {
        let start = _fbb.start_table();
        CompiledInstructionBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<CompiledInstruction<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl core::fmt::Debug for CompiledInstruction<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("CompiledInstruction");
        ds.field("program_id_index", &self.program_id_index());
        ds.field("accounts", &self.accounts());
        ds.field("data", &self.data());
        ds.finish()
    }
}
#[inline]
/// Verifies that a buffer of bytes contains a `CompiledInstruction`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_compiled_instruction_unchecked`.
pub fn root_as_compiled_instruction(
    buf: &[u8],
) -> Result<CompiledInstruction, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root::<CompiledInstruction>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `CompiledInstruction` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_compiled_instruction_unchecked`.
pub fn size_prefixed_root_as_compiled_instruction(
    buf: &[u8],
) -> Result<CompiledInstruction, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root::<CompiledInstruction>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `CompiledInstruction` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_compiled_instruction_unchecked`.
pub fn root_as_compiled_instruction_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<CompiledInstruction<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root_with_opts::<CompiledInstruction<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `CompiledInstruction` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_compiled_instruction_unchecked`.
pub fn size_prefixed_root_as_compiled_instruction_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<CompiledInstruction<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root_with_opts::<CompiledInstruction<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a CompiledInstruction and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `CompiledInstruction`.
pub unsafe fn root_as_compiled_instruction_unchecked(buf: &[u8]) -> CompiledInstruction {
    flatbuffers::root_unchecked::<CompiledInstruction>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed CompiledInstruction and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `CompiledInstruction`.
pub unsafe fn size_prefixed_root_as_compiled_instruction_unchecked(
    buf: &[u8],
) -> CompiledInstruction {
    flatbuffers::size_prefixed_root_unchecked::<CompiledInstruction>(buf)
}
#[inline]
pub fn finish_compiled_instruction_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<CompiledInstruction<'a>>,
) {
    fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_compiled_instruction_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<CompiledInstruction<'a>>,
) {
    fbb.finish_size_prefixed(root, None);
}
