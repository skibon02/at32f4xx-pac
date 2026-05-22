#[doc = "Register `DMARPD` reader"]
pub type R = crate::R<DMARPD_SPEC>;
#[doc = "Register `DMARPD` writer"]
pub type W = crate::W<DMARPD_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "EHERNET DMA receive poll demand register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmarpd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmarpd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMARPD_SPEC;
impl crate::RegisterSpec for DMARPD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmarpd::R`](R) reader structure"]
impl crate::Readable for DMARPD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmarpd::W`](W) writer structure"]
impl crate::Writable for DMARPD_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DMARPD to value 0"]
impl crate::Resettable for DMARPD_SPEC {}
