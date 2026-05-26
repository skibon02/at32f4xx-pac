#[repr(C)]
#[derive(Debug)]
#[doc = "Short/long card status response part %s"]
pub struct Rsp {
    rsp: RSP,
}
impl Rsp {
    #[doc = "0x00 - Short/long card status response"]
    #[inline(always)]
    pub const fn rsp(&self) -> &RSP {
        &self.rsp
    }
}
#[doc = "RSP (r) register accessor: Short/long card status response\n\nYou can [`read`](crate::Reg::read) this register and get [`rsp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsp`] module"]
pub type RSP = crate::Reg<rsp::RSP_SPEC>;
#[doc = "Short/long card status response"]
pub mod rsp;
