#[doc = "Register `MACHTL` reader"]
pub type R = crate::R<MACHTL_SPEC>;
#[doc = "Register `MACHTL` writer"]
pub type W = crate::W<MACHTL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ethernet MAC hash table low register\n\nYou can [`read`](crate::Reg::read) this register and get [`machtl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`machtl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACHTL_SPEC;
impl crate::RegisterSpec for MACHTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`machtl::R`](R) reader structure"]
impl crate::Readable for MACHTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`machtl::W`](W) writer structure"]
impl crate::Writable for MACHTL_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets MACHTL to value 0"]
impl crate::Resettable for MACHTL_SPEC {}
