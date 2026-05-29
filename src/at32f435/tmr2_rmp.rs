#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rmp: RMP,
}
impl RegisterBlock {
    #[doc = "0x00 - TMR2 remap"]
    #[inline(always)]
    pub const fn rmp(&self) -> &RMP {
        &self.rmp
    }
}
#[doc = "RMP (rw) register accessor: TMR2 remap\n\nYou can [`read`](crate::Reg::read) this register and get [`rmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmp`] module"]
pub type RMP = crate::Reg<rmp::RMP_SPEC>;
#[doc = "TMR2 remap"]
pub mod rmp;
