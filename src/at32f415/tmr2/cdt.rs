#[doc = "Register `C%sDT` reader"]
pub type R = crate::R<CDT_SPEC>;
#[doc = "Register `C%sDT` writer"]
pub type W = crate::W<CDT_SPEC>;
#[doc = "Field `C1DT` reader - Channel 1 data register"]
pub type C1DT_R = crate::FieldReader<u32>;
#[doc = "Field `C1DT` writer - Channel 1 data register"]
pub type C1DT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel 1 data register"]
    #[inline(always)]
    pub fn c1dt(&self) -> C1DT_R {
        C1DT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CDT").field("c1dt", &self.c1dt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel 1 data register"]
    #[inline(always)]
    pub fn c1dt(&mut self) -> C1DT_W<'_, CDT_SPEC> {
        C1DT_W::new(self, 0)
    }
}
#[doc = "Channel %s data register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDT_SPEC;
impl crate::RegisterSpec for CDT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdt::R`](R) reader structure"]
impl crate::Readable for CDT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cdt::W`](W) writer structure"]
impl crate::Writable for CDT_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets C%sDT to value 0"]
impl crate::Resettable for CDT_SPEC {}
