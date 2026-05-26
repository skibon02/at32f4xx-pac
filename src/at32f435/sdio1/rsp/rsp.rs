#[doc = "Register `RSP` reader"]
pub type R = crate::R<RSP_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Short/long card status response\n\nYou can [`read`](crate::Reg::read) this register and get [`rsp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSP_SPEC;
impl crate::RegisterSpec for RSP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsp::R`](R) reader structure"]
impl crate::Readable for RSP_SPEC {}
#[doc = "`reset()` method sets RSP to value 0"]
impl crate::Resettable for RSP_SPEC {}
