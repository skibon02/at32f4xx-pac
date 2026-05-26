#[doc = "Register `DTTMR` reader"]
pub type R = crate::R<DTTMR_SPEC>;
#[doc = "Register `DTTMR` writer"]
pub type W = crate::W<DTTMR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Bits 31:0 = TIMEOUT: Data timeout period\n\nYou can [`read`](crate::Reg::read) this register and get [`dttmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dttmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTTMR_SPEC;
impl crate::RegisterSpec for DTTMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dttmr::R`](R) reader structure"]
impl crate::Readable for DTTMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dttmr::W`](W) writer structure"]
impl crate::Writable for DTTMR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DTTMR to value 0"]
impl crate::Resettable for DTTMR_SPEC {}
