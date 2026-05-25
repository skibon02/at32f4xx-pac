#[doc = "Register `EXINTC3` reader"]
pub type R = crate::R<EXINTC3_SPEC>;
#[doc = "Register `EXINTC3` writer"]
pub type W = crate::W<EXINTC3_SPEC>;
#[doc = "Field `EXINT(8-11)` reader - External interrupt line %s"]
pub use super::exintc2::EXINT_R;
#[doc = "External interrupt line %s"]
pub use super::exintc2::EXINT_SOURCE_PORT_NO_H_A;
#[doc = "Field `EXINT(8-11)` writer - External interrupt line %s"]
pub use super::exintc2::EXINT_W;
impl R {
    #[doc = "External interrupt line (8-11)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `EXINT8` field.</div>"]
    #[inline(always)]
    pub fn exint(&self, n: u8) -> EXINT_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        EXINT_R::new(((self.bits >> (n * 4)) & 0x0f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "External interrupt line (8-11)"]
    #[inline(always)]
    pub fn exint_iter(&self) -> impl Iterator<Item = EXINT_R> + '_ {
        (0..4).map(move |n| EXINT_R::new(((self.bits >> (n * 4)) & 0x0f) as u8))
    }
    #[doc = "Bits 0:3 - External interrupt line 8"]
    #[inline(always)]
    pub fn exint8(&self) -> EXINT_R {
        EXINT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - External interrupt line 9"]
    #[inline(always)]
    pub fn exint9(&self) -> EXINT_R {
        EXINT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - External interrupt line 10"]
    #[inline(always)]
    pub fn exint10(&self) -> EXINT_R {
        EXINT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - External interrupt line 11"]
    #[inline(always)]
    pub fn exint11(&self) -> EXINT_R {
        EXINT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXINTC3")
            .field("exint8", &self.exint8())
            .field("exint9", &self.exint9())
            .field("exint10", &self.exint10())
            .field("exint11", &self.exint11())
            .finish()
    }
}
impl W {
    #[doc = "External interrupt line (8-11)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `EXINT8` field.</div>"]
    #[inline(always)]
    pub fn exint(&mut self, n: u8) -> EXINT_W<'_, EXINTC3_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        EXINT_W::new(self, n * 4)
    }
    #[doc = "Bits 0:3 - External interrupt line 8"]
    #[inline(always)]
    pub fn exint8(&mut self) -> EXINT_W<'_, EXINTC3_SPEC> {
        EXINT_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - External interrupt line 9"]
    #[inline(always)]
    pub fn exint9(&mut self) -> EXINT_W<'_, EXINTC3_SPEC> {
        EXINT_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - External interrupt line 10"]
    #[inline(always)]
    pub fn exint10(&mut self) -> EXINT_W<'_, EXINTC3_SPEC> {
        EXINT_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - External interrupt line 11"]
    #[inline(always)]
    pub fn exint11(&mut self) -> EXINT_W<'_, EXINTC3_SPEC> {
        EXINT_W::new(self, 12)
    }
}
#[doc = "external interrupt configuration register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`exintc3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exintc3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXINTC3_SPEC;
impl crate::RegisterSpec for EXINTC3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exintc3::R`](R) reader structure"]
impl crate::Readable for EXINTC3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exintc3::W`](W) writer structure"]
impl crate::Writable for EXINTC3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXINTC3 to value 0"]
impl crate::Resettable for EXINTC3_SPEC {}
