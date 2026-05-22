#[doc = "Register `DMATPD` reader"]
pub type R = crate::R<DMATPD_SPEC>;
#[doc = "Register `DMATPD` writer"]
pub type W = crate::W<DMATPD_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ethernet DMA transmit poll demand register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatpd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatpd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMATPD_SPEC;
impl crate::RegisterSpec for DMATPD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatpd::R`](R) reader structure"]
impl crate::Readable for DMATPD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmatpd::W`](W) writer structure"]
impl crate::Writable for DMATPD_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DMATPD to value 0"]
impl crate::Resettable for DMATPD_SPEC {}
