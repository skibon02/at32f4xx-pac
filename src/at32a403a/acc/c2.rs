#[doc = "Register `C2` reader"]
pub type R = crate::R<C2_SPEC>;
#[doc = "Register `C2` writer"]
pub type W = crate::W<C2_SPEC>;
#[doc = "Field `C2` reader - Compare 2"]
pub type C2_R = crate::FieldReader<u16>;
#[doc = "Field `C2` writer - Compare 2"]
pub type C2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Compare 2"]
    #[inline(always)]
    pub fn c2(&self) -> C2_R {
        C2_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2").field("c2", &self.c2()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare 2"]
    #[inline(always)]
    pub fn c2(&mut self) -> C2_W<'_, C2_SPEC> {
        C2_W::new(self, 0)
    }
}
#[doc = "compare value 2\n\nYou can [`read`](crate::Reg::read) this register and get [`c2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2_SPEC;
impl crate::RegisterSpec for C2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2::R`](R) reader structure"]
impl crate::Readable for C2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c2::W`](W) writer structure"]
impl crate::Writable for C2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C2 to value 0x1f40"]
impl crate::Resettable for C2_SPEC {
    const RESET_VALUE: u32 = 0x1f40;
}
