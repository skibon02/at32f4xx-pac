#[doc = "Register `EXINTC4` reader"]
pub type R = crate::R<EXINTC4_SPEC>;
#[doc = "Register `EXINTC4` writer"]
pub type W = crate::W<EXINTC4_SPEC>;
#[doc = "Field `EXINT(12-15)` reader - External interrupt line %s"]
pub use super::exintc2::EXINT_R;
#[doc = "External interrupt line %s"]
pub use super::exintc2::EXINT_SOURCE_PORT_NO_H_A;
#[doc = "Field `EXINT(12-15)` writer - External interrupt line %s"]
pub use super::exintc2::EXINT_W;
impl R {
    #[doc = "External interrupt line (12-15)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `EXINT12` field.</div>"]
    #[inline(always)]
    pub fn exint(&self, n: u8) -> EXINT_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        EXINT_R::new(((self.bits >> (n * 4)) & 0x0f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "External interrupt line (12-15)"]
    #[inline(always)]
    pub fn exint_iter(&self) -> impl Iterator<Item = EXINT_R> + '_ {
        (0..4).map(move |n| EXINT_R::new(((self.bits >> (n * 4)) & 0x0f) as u8))
    }
    #[doc = "Bits 0:3 - External interrupt line 12"]
    #[inline(always)]
    pub fn exint12(&self) -> EXINT_R {
        EXINT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - External interrupt line 13"]
    #[inline(always)]
    pub fn exint13(&self) -> EXINT_R {
        EXINT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - External interrupt line 14"]
    #[inline(always)]
    pub fn exint14(&self) -> EXINT_R {
        EXINT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - External interrupt line 15"]
    #[inline(always)]
    pub fn exint15(&self) -> EXINT_R {
        EXINT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXINTC4")
            .field("exint12", &self.exint12())
            .field("exint13", &self.exint13())
            .field("exint14", &self.exint14())
            .field("exint15", &self.exint15())
            .finish()
    }
}
impl W {
    #[doc = "External interrupt line (12-15)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `EXINT12` field.</div>"]
    #[inline(always)]
    pub fn exint(&mut self, n: u8) -> EXINT_W<'_, EXINTC4_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        EXINT_W::new(self, n * 4)
    }
    #[doc = "Bits 0:3 - External interrupt line 12"]
    #[inline(always)]
    pub fn exint12(&mut self) -> EXINT_W<'_, EXINTC4_SPEC> {
        EXINT_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - External interrupt line 13"]
    #[inline(always)]
    pub fn exint13(&mut self) -> EXINT_W<'_, EXINTC4_SPEC> {
        EXINT_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - External interrupt line 14"]
    #[inline(always)]
    pub fn exint14(&mut self) -> EXINT_W<'_, EXINTC4_SPEC> {
        EXINT_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - External interrupt line 15"]
    #[inline(always)]
    pub fn exint15(&mut self) -> EXINT_W<'_, EXINTC4_SPEC> {
        EXINT_W::new(self, 12)
    }
}
#[doc = "external interrupt configuration register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`exintc4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exintc4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXINTC4_SPEC;
impl crate::RegisterSpec for EXINTC4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exintc4::R`](R) reader structure"]
impl crate::Readable for EXINTC4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exintc4::W`](W) writer structure"]
impl crate::Writable for EXINTC4_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXINTC4 to value 0"]
impl crate::Resettable for EXINTC4_SPEC {}
