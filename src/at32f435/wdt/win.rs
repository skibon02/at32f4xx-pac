#[doc = "Register `WIN` reader"]
pub type R = crate::R<WIN_SPEC>;
#[doc = "Register `WIN` writer"]
pub type W = crate::W<WIN_SPEC>;
#[doc = "Field `WIN` reader - Window value"]
pub type WIN_R = crate::FieldReader<u16>;
#[doc = "Field `WIN` writer - Window value"]
pub type WIN_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:11 - Window value"]
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIN").field("win", &self.win()).finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Window value"]
    #[inline(always)]
    pub fn win(&mut self) -> WIN_W<'_, WIN_SPEC> {
        WIN_W::new(self, 0)
    }
}
#[doc = "Window register\n\nYou can [`read`](crate::Reg::read) this register and get [`win::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`win::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIN_SPEC;
impl crate::RegisterSpec for WIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win::R`](R) reader structure"]
impl crate::Readable for WIN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`win::W`](W) writer structure"]
impl crate::Writable for WIN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WIN to value 0x0fff"]
impl crate::Resettable for WIN_SPEC {
    const RESET_VALUE: u32 = 0x0fff;
}
