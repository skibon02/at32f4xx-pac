#[doc = "Register `CLR` writer"]
pub type W = crate::W<CLR_SPEC>;
#[doc = "Clear 0~7 bit address match flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRCW_A {
    #[doc = "1: Write 1 to clear flag"]
    Clear = 1,
}
impl From<ADDRCW_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRCW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRC` writer - Clear 0~7 bit address match flag"]
pub type ADDRC_W<'a, REG> = crate::BitWriter1C<'a, REG, ADDRCW_A>;
impl<'a, REG> ADDRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write 1 to clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ADDRCW_A::Clear)
    }
}
#[doc = "Field `ACKFAILC` writer - Clear acknowledge failure flag"]
pub use ADDRC_W as ACKFAILC_W;
#[doc = "Field `STOPC` writer - Clear stop condition generation complete flag"]
pub use ADDRC_W as STOPC_W;
#[doc = "Field `BUSERRC` writer - Clear bus error flag"]
pub use ADDRC_W as BUSERRC_W;
#[doc = "Field `ARLOSTC` writer - Clear arbitration lost flag"]
pub use ADDRC_W as ARLOSTC_W;
#[doc = "Field `OUFC` writer - Clear overload / underload flag"]
pub use ADDRC_W as OUFC_W;
#[doc = "Field `PECERRC` writer - Clear PEC receive error flag"]
pub use ADDRC_W as PECERRC_W;
#[doc = "Field `TMOUTC` writer - Clear SMBus timeout flag"]
pub use ADDRC_W as TMOUTC_W;
#[doc = "Field `ALERTC` writer - Clear SMBus alert flag"]
pub use ADDRC_W as ALERTC_W;
impl core::fmt::Debug for crate::generic::Reg<CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 3 - Clear 0~7 bit address match flag"]
    #[inline(always)]
    pub fn addrc(&mut self) -> ADDRC_W<'_, CLR_SPEC> {
        ADDRC_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear acknowledge failure flag"]
    #[inline(always)]
    pub fn ackfailc(&mut self) -> ACKFAILC_W<'_, CLR_SPEC> {
        ACKFAILC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear stop condition generation complete flag"]
    #[inline(always)]
    pub fn stopc(&mut self) -> STOPC_W<'_, CLR_SPEC> {
        STOPC_W::new(self, 5)
    }
    #[doc = "Bit 8 - Clear bus error flag"]
    #[inline(always)]
    pub fn buserrc(&mut self) -> BUSERRC_W<'_, CLR_SPEC> {
        BUSERRC_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear arbitration lost flag"]
    #[inline(always)]
    pub fn arlostc(&mut self) -> ARLOSTC_W<'_, CLR_SPEC> {
        ARLOSTC_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear overload / underload flag"]
    #[inline(always)]
    pub fn oufc(&mut self) -> OUFC_W<'_, CLR_SPEC> {
        OUFC_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear PEC receive error flag"]
    #[inline(always)]
    pub fn pecerrc(&mut self) -> PECERRC_W<'_, CLR_SPEC> {
        PECERRC_W::new(self, 11)
    }
    #[doc = "Bit 12 - Clear SMBus timeout flag"]
    #[inline(always)]
    pub fn tmoutc(&mut self) -> TMOUTC_W<'_, CLR_SPEC> {
        TMOUTC_W::new(self, 12)
    }
    #[doc = "Bit 13 - Clear SMBus alert flag"]
    #[inline(always)]
    pub fn alertc(&mut self) -> ALERTC_W<'_, CLR_SPEC> {
        ALERTC_W::new(self, 13)
    }
}
#[doc = "Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLR_SPEC;
impl crate::RegisterSpec for CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clr::W`](W) writer structure"]
impl crate::Writable for CLR_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f38;
}
#[doc = "`reset()` method sets CLR to value 0"]
impl crate::Resettable for CLR_SPEC {}
