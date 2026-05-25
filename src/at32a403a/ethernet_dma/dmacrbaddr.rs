#[doc = "Register `DMACRBADDR` reader"]
pub type R = crate::R<DMACRBADDR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Ethernet DMA current host receive buffer address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacrbaddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACRBADDR_SPEC;
impl crate::RegisterSpec for DMACRBADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacrbaddr::R`](R) reader structure"]
impl crate::Readable for DMACRBADDR_SPEC {}
#[doc = "`reset()` method sets DMACRBADDR to value 0"]
impl crate::Resettable for DMACRBADDR_SPEC {}
