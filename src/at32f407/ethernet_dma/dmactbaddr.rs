#[doc = "Register `DMACTBADDR` reader"]
pub type R = crate::R<DMACTBADDR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Ethernet DMA current host transmit buffer address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactbaddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACTBADDR_SPEC;
impl crate::RegisterSpec for DMACTBADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactbaddr::R`](R) reader structure"]
impl crate::Readable for DMACTBADDR_SPEC {}
#[doc = "`reset()` method sets DMACTBADDR to value 0"]
impl crate::Resettable for DMACTBADDR_SPEC {}
