#[doc = "Register `MACA0H` reader"]
pub type R = crate::R<MACA0H_SPEC>;
#[doc = "Register `MACA0H` writer"]
pub type W = crate::W<MACA0H_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ethernet MAC address 0 high register. This register changed to 16bits wide, because the only available another field is bit 31 - AE, but it is restricted to be always 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`maca0h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca0h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACA0H_SPEC;
impl crate::RegisterSpec for MACA0H_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`maca0h::R`](R) reader structure"]
impl crate::Readable for MACA0H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maca0h::W`](W) writer structure"]
impl crate::Writable for MACA0H_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets MACA0H to value 0xffff"]
impl crate::Resettable for MACA0H_SPEC {
    const RESET_VALUE: u16 = 0xffff;
}
