#[doc = "Register `MACMIIDT` reader"]
pub type R = crate::R<MACMIIDT_SPEC>;
#[doc = "Register `MACMIIDT` writer"]
pub type W = crate::W<MACMIIDT_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ethernet MAC MII data register\n\nYou can [`read`](crate::Reg::read) this register and get [`macmiidt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macmiidt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACMIIDT_SPEC;
impl crate::RegisterSpec for MACMIIDT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`macmiidt::R`](R) reader structure"]
impl crate::Readable for MACMIIDT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macmiidt::W`](W) writer structure"]
impl crate::Writable for MACMIIDT_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets MACMIIDT to value 0"]
impl crate::Resettable for MACMIIDT_SPEC {}
