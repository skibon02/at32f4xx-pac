#[doc = "Register `CONTR` reader"]
pub type R = crate::R<CONTR_SPEC>;
#[doc = "Register `CONTR` writer"]
pub type W = crate::W<CONTR_SPEC>;
#[doc = "Flash continue read enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCONTR_EN_A {
    #[doc = "0: Flash continue read disabled"]
    Disabled = 0,
    #[doc = "1: Flash continue read enabled. Faster reads, but more power consumption"]
    Enabled = 1,
}
impl From<FCONTR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FCONTR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCONTR_EN` reader - Flash continue read enable"]
pub type FCONTR_EN_R = crate::BitReader<FCONTR_EN_A>;
impl FCONTR_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FCONTR_EN_A {
        match self.bits {
            false => FCONTR_EN_A::Disabled,
            true => FCONTR_EN_A::Enabled,
        }
    }
    #[doc = "Flash continue read disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FCONTR_EN_A::Disabled
    }
    #[doc = "Flash continue read enabled. Faster reads, but more power consumption"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FCONTR_EN_A::Enabled
    }
}
#[doc = "Field `FCONTR_EN` writer - Flash continue read enable"]
pub type FCONTR_EN_W<'a, REG> = crate::BitWriter<'a, REG, FCONTR_EN_A>;
impl<'a, REG> FCONTR_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash continue read disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FCONTR_EN_A::Disabled)
    }
    #[doc = "Flash continue read enabled. Faster reads, but more power consumption"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FCONTR_EN_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 31 - Flash continue read enable"]
    #[inline(always)]
    pub fn fcontr_en(&self) -> FCONTR_EN_R {
        FCONTR_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONTR")
            .field("fcontr_en", &self.fcontr_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - Flash continue read enable"]
    #[inline(always)]
    pub fn fcontr_en(&mut self) -> FCONTR_EN_W<'_, CONTR_SPEC> {
        FCONTR_EN_W::new(self, 31)
    }
}
#[doc = "Flash continue read register\n\nYou can [`read`](crate::Reg::read) this register and get [`contr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`contr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONTR_SPEC;
impl crate::RegisterSpec for CONTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`contr::R`](R) reader structure"]
impl crate::Readable for CONTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`contr::W`](W) writer structure"]
impl crate::Writable for CONTR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONTR to value 0x80"]
impl crate::Resettable for CONTR_SPEC {
    const RESET_VALUE: u32 = 0x80;
}
