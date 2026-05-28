#[repr(C)]
#[derive(Debug)]
#[doc = "Cluster GeneratorChannelMux%s, containing MUXG?CTRL"]
pub struct GeneratorChannelMux {
    muxgctrl: MUXGCTRL,
}
impl GeneratorChannelMux {
    #[doc = "0x00 - DMA generator channel mux control register"]
    #[inline(always)]
    pub const fn muxgctrl(&self) -> &MUXGCTRL {
        &self.muxgctrl
    }
}
#[doc = "MUXGCTRL (rw) register accessor: DMA generator channel mux control register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxgctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxgctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxgctrl`] module"]
pub type MUXGCTRL = crate::Reg<muxgctrl::MUXGCTRL_SPEC>;
#[doc = "DMA generator channel mux control register"]
pub mod muxgctrl;
