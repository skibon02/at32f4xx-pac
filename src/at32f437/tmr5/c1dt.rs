#[doc = "Register `C1DT` reader"]
pub type R = crate::R<C1DT_SPEC>;
#[doc = "Register `C1DT` writer"]
pub type W = crate::W<C1DT_SPEC>;
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
        f.debug_struct("C1DT").field("c1dt", &self.c1dt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel 1 data register"]
    #[inline(always)]
    pub fn c1dt(&mut self) -> C1DT_W<'_, C1DT_SPEC> {
        C1DT_W::new(self, 0)
    }
}
#[doc = "Channel 1 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1dt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1dt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1DT_SPEC;
impl crate::RegisterSpec for C1DT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1dt::R`](R) reader structure"]
impl crate::Readable for C1DT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c1dt::W`](W) writer structure"]
impl crate::Writable for C1DT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C1DT to value 0"]
impl crate::Resettable for C1DT_SPEC {}
