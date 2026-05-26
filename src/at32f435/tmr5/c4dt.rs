#[doc = "Register `C4DT` reader"]
pub type R = crate::R<C4DT_SPEC>;
#[doc = "Register `C4DT` writer"]
pub type W = crate::W<C4DT_SPEC>;
#[doc = "Field `C4DT` reader - Channel 4 data register"]
pub type C4DT_R = crate::FieldReader<u32>;
#[doc = "Field `C4DT` writer - Channel 4 data register"]
pub type C4DT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel 4 data register"]
    #[inline(always)]
    pub fn c4dt(&self) -> C4DT_R {
        C4DT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C4DT").field("c4dt", &self.c4dt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel 4 data register"]
    #[inline(always)]
    pub fn c4dt(&mut self) -> C4DT_W<'_, C4DT_SPEC> {
        C4DT_W::new(self, 0)
    }
}
#[doc = "Channel 4 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`c4dt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4dt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C4DT_SPEC;
impl crate::RegisterSpec for C4DT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c4dt::R`](R) reader structure"]
impl crate::Readable for C4DT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c4dt::W`](W) writer structure"]
impl crate::Writable for C4DT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C4DT to value 0"]
impl crate::Resettable for C4DT_SPEC {}
