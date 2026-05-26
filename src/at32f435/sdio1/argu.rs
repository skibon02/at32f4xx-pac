#[doc = "Register `ARGU` reader"]
pub type R = crate::R<ARGU_SPEC>;
#[doc = "Register `ARGU` writer"]
pub type W = crate::W<ARGU_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Bits 31:0 = : Command argument\n\nYou can [`read`](crate::Reg::read) this register and get [`argu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`argu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARGU_SPEC;
impl crate::RegisterSpec for ARGU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`argu::R`](R) reader structure"]
impl crate::Readable for ARGU_SPEC {}
#[doc = "`write(|w| ..)` method takes [`argu::W`](W) writer structure"]
impl crate::Writable for ARGU_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ARGU to value 0"]
impl crate::Resettable for ARGU_SPEC {}
