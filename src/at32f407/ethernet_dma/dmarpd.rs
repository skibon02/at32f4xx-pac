#[doc = "Register `DMARPD` writer"]
pub type W = crate::W<DMARPD_SPEC>;
#[doc = "Receive poll demand\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum RPDW_A {
    #[doc = "0: When these bits are written with any value, the DMA reads the current descriptor pointed to by the EMAC_DMACRD. If the descriptor is not available (owned by host), the reception suspends, and the bit 7 (RU) is set in the status register. If the descriptor is available, the reception resumes."]
    Poll = 0,
}
impl From<RPDW_A> for u32 {
    #[inline(always)]
    fn from(variant: RPDW_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RPDW_A {
    type Ux = u32;
}
impl crate::IsEnum for RPDW_A {}
#[doc = "Field `RPD` writer - Receive poll demand"]
pub type RPD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, RPDW_A>;
impl<'a, REG> RPD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "When these bits are written with any value, the DMA reads the current descriptor pointed to by the EMAC_DMACRD. If the descriptor is not available (owned by host), the reception suspends, and the bit 7 (RU) is set in the status register. If the descriptor is available, the reception resumes."]
    #[inline(always)]
    pub fn poll(self) -> &'a mut crate::W<REG> {
        self.variant(RPDW_A::Poll)
    }
}
impl core::fmt::Debug for crate::generic::Reg<DMARPD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive poll demand"]
    #[inline(always)]
    pub fn rpd(&mut self) -> RPD_W<'_, DMARPD_SPEC> {
        RPD_W::new(self, 0)
    }
}
#[doc = "Ethernet DMA receive poll demand register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmarpd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMARPD_SPEC;
impl crate::RegisterSpec for DMARPD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dmarpd::W`](W) writer structure"]
impl crate::Writable for DMARPD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMARPD to value 0"]
impl crate::Resettable for DMARPD_SPEC {}
