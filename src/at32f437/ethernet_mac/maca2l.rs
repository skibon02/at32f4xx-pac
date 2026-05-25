#[doc = "Register `MACA2L` reader"]
pub type R = crate::R<MACA2L_SPEC>;
#[doc = "Register `MACA2L` writer"]
pub type W = crate::W<MACA2L_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ethernet MAC address 2 low register\n\nYou can [`read`](crate::Reg::read) this register and get [`maca2l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca2l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACA2L_SPEC;
impl crate::RegisterSpec for MACA2L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca2l::R`](R) reader structure"]
impl crate::Readable for MACA2L_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maca2l::W`](W) writer structure"]
impl crate::Writable for MACA2L_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets MACA2L to value 0xffff_ffff"]
impl crate::Resettable for MACA2L_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
