#[doc = "Register `DMACRD` reader"]
pub type R = crate::R<DMACRD_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Ethernet DMA current host receive descriptor register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacrd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACRD_SPEC;
impl crate::RegisterSpec for DMACRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacrd::R`](R) reader structure"]
impl crate::Readable for DMACRD_SPEC {}
#[doc = "`reset()` method sets DMACRD to value 0"]
impl crate::Resettable for DMACRD_SPEC {}
