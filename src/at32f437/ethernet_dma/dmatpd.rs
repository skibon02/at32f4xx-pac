#[doc = "Register `DMATPD` writer"]
pub type W = crate::W<DMATPD_SPEC>;
#[doc = "Transmit poll demand\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum TPDW_A {
    #[doc = "0: When these bits are written with any value, the DMA reads the current descriptor pointed to by the EMAC_DMACTD. If the descriptor is not available (owned by host), the transmission suspends, and the bit 2 (TU) is set in the status register. If the descriptor is available, the transmission resumes."]
    Poll = 0,
}
impl From<TPDW_A> for u32 {
    #[inline(always)]
    fn from(variant: TPDW_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TPDW_A {
    type Ux = u32;
}
impl crate::IsEnum for TPDW_A {}
#[doc = "Field `TPD` writer - Transmit poll demand"]
pub type TPD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, TPDW_A>;
impl<'a, REG> TPD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "When these bits are written with any value, the DMA reads the current descriptor pointed to by the EMAC_DMACTD. If the descriptor is not available (owned by host), the transmission suspends, and the bit 2 (TU) is set in the status register. If the descriptor is available, the transmission resumes."]
    #[inline(always)]
    pub fn poll(self) -> &'a mut crate::W<REG> {
        self.variant(TPDW_A::Poll)
    }
}
impl core::fmt::Debug for crate::generic::Reg<DMATPD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit poll demand"]
    #[inline(always)]
    pub fn tpd(&mut self) -> TPD_W<'_, DMATPD_SPEC> {
        TPD_W::new(self, 0)
    }
}
#[doc = "Ethernet DMA transmit poll demand register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatpd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMATPD_SPEC;
impl crate::RegisterSpec for DMATPD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dmatpd::W`](W) writer structure"]
impl crate::Writable for DMATPD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMATPD to value 0"]
impl crate::Resettable for DMATPD_SPEC {}
