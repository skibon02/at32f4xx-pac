#[doc = "Register `C2DT` reader"]
pub type R = crate::R<C2DT_SPEC>;
#[doc = "Register `C2DT` writer"]
pub type W = crate::W<C2DT_SPEC>;
#[doc = "Field `C2DT` reader - Channel 2 data register"]
pub type C2DT_R = crate::FieldReader<u32>;
#[doc = "Field `C2DT` writer - Channel 2 data register"]
pub type C2DT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel 2 data register"]
    #[inline(always)]
    pub fn c2dt(&self) -> C2DT_R {
        C2DT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2DT").field("c2dt", &self.c2dt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel 2 data register"]
    #[inline(always)]
    pub fn c2dt(&mut self) -> C2DT_W<'_, C2DT_SPEC> {
        C2DT_W::new(self, 0)
    }
}
#[doc = "Channel 2 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`c2dt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2dt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2DT_SPEC;
impl crate::RegisterSpec for C2DT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2dt::R`](R) reader structure"]
impl crate::Readable for C2DT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c2dt::W`](W) writer structure"]
impl crate::Writable for C2DT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C2DT to value 0"]
impl crate::Resettable for C2DT_SPEC {}
