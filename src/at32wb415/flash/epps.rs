#[doc = "Register `EPPS` reader"]
pub type R = crate::R<EPPS_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Erase/program protection status register\n\nYou can [`read`](crate::Reg::read) this register and get [`epps::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPPS_SPEC;
impl crate::RegisterSpec for EPPS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epps::R`](R) reader structure"]
impl crate::Readable for EPPS_SPEC {}
#[doc = "`reset()` method sets EPPS to value 0xffff_ffff"]
impl crate::Resettable for EPPS_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
