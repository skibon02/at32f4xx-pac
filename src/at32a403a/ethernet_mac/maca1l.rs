#[doc = "Register `MACA1L` reader"]
pub type R = crate::R<MACA1L_SPEC>;
#[doc = "Register `MACA1L` writer"]
pub type W = crate::W<MACA1L_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ethernet MAC address1 low register\n\nYou can [`read`](crate::Reg::read) this register and get [`maca1l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca1l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACA1L_SPEC;
impl crate::RegisterSpec for MACA1L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca1l::R`](R) reader structure"]
impl crate::Readable for MACA1L_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maca1l::W`](W) writer structure"]
impl crate::Writable for MACA1L_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets MACA1L to value 0xffff_ffff"]
impl crate::Resettable for MACA1L_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
