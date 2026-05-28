#[doc = "Register `MACHTH` reader"]
pub type R = crate::R<MACHTH_SPEC>;
#[doc = "Register `MACHTH` writer"]
pub type W = crate::W<MACHTH_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ethernet MAC hash table high register\n\nYou can [`read`](crate::Reg::read) this register and get [`machth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`machth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACHTH_SPEC;
impl crate::RegisterSpec for MACHTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`machth::R`](R) reader structure"]
impl crate::Readable for MACHTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`machth::W`](W) writer structure"]
impl crate::Writable for MACHTH_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets MACHTH to value 0"]
impl crate::Resettable for MACHTH_SPEC {}
