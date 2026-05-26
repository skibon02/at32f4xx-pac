#[doc = "Register `C3DT` reader"]
pub type R = crate::R<C3DT_SPEC>;
#[doc = "Register `C3DT` writer"]
pub type W = crate::W<C3DT_SPEC>;
#[doc = "Field `C3DT` reader - Channel 3 data register"]
pub type C3DT_R = crate::FieldReader<u32>;
#[doc = "Field `C3DT` writer - Channel 3 data register"]
pub type C3DT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel 3 data register"]
    #[inline(always)]
    pub fn c3dt(&self) -> C3DT_R {
        C3DT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C3DT").field("c3dt", &self.c3dt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel 3 data register"]
    #[inline(always)]
    pub fn c3dt(&mut self) -> C3DT_W<'_, C3DT_SPEC> {
        C3DT_W::new(self, 0)
    }
}
#[doc = "Channel 3 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`c3dt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3dt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C3DT_SPEC;
impl crate::RegisterSpec for C3DT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c3dt::R`](R) reader structure"]
impl crate::Readable for C3DT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c3dt::W`](W) writer structure"]
impl crate::Writable for C3DT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C3DT to value 0"]
impl crate::Resettable for C3DT_SPEC {}
