#[repr(C)]
#[derive(Debug)]
#[doc = "Channel %s mux"]
pub struct ChannelMux {
    muxctrl: MUXCTRL,
}
impl ChannelMux {
    #[doc = "0x00 - DMA channel mux control register"]
    #[inline(always)]
    pub const fn muxctrl(&self) -> &MUXCTRL {
        &self.muxctrl
    }
}
#[doc = "MUXCTRL (rw) register accessor: DMA channel mux control register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxctrl`] module"]
pub type MUXCTRL = crate::Reg<muxctrl::MUXCTRL_SPEC>;
#[doc = "DMA channel mux control register"]
pub mod muxctrl;
