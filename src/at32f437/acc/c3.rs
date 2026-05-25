#[doc = "Register `C3` reader"]
pub type R = crate::R<C3_SPEC>;
#[doc = "Register `C3` writer"]
pub type W = crate::W<C3_SPEC>;
#[doc = "Field `C3` reader - Compare 3"]
pub type C3_R = crate::FieldReader<u16>;
#[doc = "Field `C3` writer - Compare 3"]
pub type C3_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Compare 3"]
    #[inline(always)]
    pub fn c3(&self) -> C3_R {
        C3_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C3").field("c3", &self.c3()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare 3"]
    #[inline(always)]
    pub fn c3(&mut self) -> C3_W<'_, C3_SPEC> {
        C3_W::new(self, 0)
    }
}
#[doc = "compare value 3\n\nYou can [`read`](crate::Reg::read) this register and get [`c3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C3_SPEC;
impl crate::RegisterSpec for C3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c3::R`](R) reader structure"]
impl crate::Readable for C3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c3::W`](W) writer structure"]
impl crate::Writable for C3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C3 to value 0x1f54"]
impl crate::Resettable for C3_SPEC {
    const RESET_VALUE: u32 = 0x1f54;
}
