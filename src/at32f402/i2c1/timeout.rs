#[doc = "Register `TIMEOUT` reader"]
pub type R = crate::R<TIMEOUT_SPEC>;
#[doc = "Register `TIMEOUT` writer"]
pub type W = crate::W<TIMEOUT_SPEC>;
#[doc = "Field `TOTIME` reader - Clock timeout. For low level: (TOTIME + 1) x 2048 x TI2C_CLK. For high level: (TOTIME + 1) x 4 x TI2C_CLK."]
pub type TOTIME_R = crate::FieldReader<u16>;
#[doc = "Field `TOTIME` writer - Clock timeout. For low level: (TOTIME + 1) x 2048 x TI2C_CLK. For high level: (TOTIME + 1) x 4 x TI2C_CLK."]
pub type TOTIME_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
#[doc = "Clock timeout detection mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOMODE_A {
    #[doc = "0: Clock low level detection timeout"]
    LowLevel = 0,
    #[doc = "1: Clock high level detection timeout"]
    HighLevel = 1,
}
impl From<TOMODE_A> for bool {
    #[inline(always)]
    fn from(variant: TOMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOMODE` reader - Clock timeout detection mode"]
pub type TOMODE_R = crate::BitReader<TOMODE_A>;
impl TOMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TOMODE_A {
        match self.bits {
            false => TOMODE_A::LowLevel,
            true => TOMODE_A::HighLevel,
        }
    }
    #[doc = "Clock low level detection timeout"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == TOMODE_A::LowLevel
    }
    #[doc = "Clock high level detection timeout"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == TOMODE_A::HighLevel
    }
}
#[doc = "Field `TOMODE` writer - Clock timeout detection mode"]
pub type TOMODE_W<'a, REG> = crate::BitWriter<'a, REG, TOMODE_A>;
impl<'a, REG> TOMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock low level detection timeout"]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut crate::W<REG> {
        self.variant(TOMODE_A::LowLevel)
    }
    #[doc = "Clock high level detection timeout"]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut crate::W<REG> {
        self.variant(TOMODE_A::HighLevel)
    }
}
#[doc = "Detect clock low/high timeout enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOEN_A {
    #[doc = "0: Clock timeout detection disabled"]
    Disabled = 0,
    #[doc = "1: Clock timeout detection enabled"]
    Enabled = 1,
}
impl From<TOEN_A> for bool {
    #[inline(always)]
    fn from(variant: TOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOEN` reader - Detect clock low/high timeout enable"]
pub type TOEN_R = crate::BitReader<TOEN_A>;
impl TOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TOEN_A {
        match self.bits {
            false => TOEN_A::Disabled,
            true => TOEN_A::Enabled,
        }
    }
    #[doc = "Clock timeout detection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TOEN_A::Disabled
    }
    #[doc = "Clock timeout detection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TOEN_A::Enabled
    }
}
#[doc = "Field `TOEN` writer - Detect clock low/high timeout enable"]
pub type TOEN_W<'a, REG> = crate::BitWriter<'a, REG, TOEN_A>;
impl<'a, REG> TOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock timeout detection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TOEN_A::Disabled)
    }
    #[doc = "Clock timeout detection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TOEN_A::Enabled)
    }
}
#[doc = "Field `EXTTIME` reader - Cumulative clock low extend timeout value. (EXTTIME + 1) x 2048 x TI2C_CLK."]
pub type EXTTIME_R = crate::FieldReader<u16>;
#[doc = "Field `EXTTIME` writer - Cumulative clock low extend timeout value. (EXTTIME + 1) x 2048 x TI2C_CLK."]
pub type EXTTIME_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
#[doc = "Cumulative clock low extend timeout enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTEN_A {
    #[doc = "0: Cumulative clock low extend timeout disabled"]
    Disabled = 0,
    #[doc = "1: Cumulative clock low extend timeout enabled"]
    Enabled = 1,
}
impl From<EXTEN_A> for bool {
    #[inline(always)]
    fn from(variant: EXTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTEN` reader - Cumulative clock low extend timeout enable"]
pub type EXTEN_R = crate::BitReader<EXTEN_A>;
impl EXTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTEN_A {
        match self.bits {
            false => EXTEN_A::Disabled,
            true => EXTEN_A::Enabled,
        }
    }
    #[doc = "Cumulative clock low extend timeout disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTEN_A::Disabled
    }
    #[doc = "Cumulative clock low extend timeout enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EXTEN_A::Enabled
    }
}
#[doc = "Field `EXTEN` writer - Cumulative clock low extend timeout enable"]
pub type EXTEN_W<'a, REG> = crate::BitWriter<'a, REG, EXTEN_A>;
impl<'a, REG> EXTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cumulative clock low extend timeout disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN_A::Disabled)
    }
    #[doc = "Cumulative clock low extend timeout enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN_A::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:11 - Clock timeout. For low level: (TOTIME + 1) x 2048 x TI2C_CLK. For high level: (TOTIME + 1) x 4 x TI2C_CLK."]
    #[inline(always)]
    pub fn totime(&self) -> TOTIME_R {
        TOTIME_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Clock timeout detection mode"]
    #[inline(always)]
    pub fn tomode(&self) -> TOMODE_R {
        TOMODE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Detect clock low/high timeout enable"]
    #[inline(always)]
    pub fn toen(&self) -> TOEN_R {
        TOEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:27 - Cumulative clock low extend timeout value. (EXTTIME + 1) x 2048 x TI2C_CLK."]
    #[inline(always)]
    pub fn exttime(&self) -> EXTTIME_R {
        EXTTIME_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Cumulative clock low extend timeout enable"]
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMEOUT")
            .field("totime", &self.totime())
            .field("tomode", &self.tomode())
            .field("toen", &self.toen())
            .field("exttime", &self.exttime())
            .field("exten", &self.exten())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Clock timeout. For low level: (TOTIME + 1) x 2048 x TI2C_CLK. For high level: (TOTIME + 1) x 4 x TI2C_CLK."]
    #[inline(always)]
    pub fn totime(&mut self) -> TOTIME_W<'_, TIMEOUT_SPEC> {
        TOTIME_W::new(self, 0)
    }
    #[doc = "Bit 12 - Clock timeout detection mode"]
    #[inline(always)]
    pub fn tomode(&mut self) -> TOMODE_W<'_, TIMEOUT_SPEC> {
        TOMODE_W::new(self, 12)
    }
    #[doc = "Bit 15 - Detect clock low/high timeout enable"]
    #[inline(always)]
    pub fn toen(&mut self) -> TOEN_W<'_, TIMEOUT_SPEC> {
        TOEN_W::new(self, 15)
    }
    #[doc = "Bits 16:27 - Cumulative clock low extend timeout value. (EXTTIME + 1) x 2048 x TI2C_CLK."]
    #[inline(always)]
    pub fn exttime(&mut self) -> EXTTIME_W<'_, TIMEOUT_SPEC> {
        EXTTIME_W::new(self, 16)
    }
    #[doc = "Bit 31 - Cumulative clock low extend timeout enable"]
    #[inline(always)]
    pub fn exten(&mut self) -> EXTEN_W<'_, TIMEOUT_SPEC> {
        EXTEN_W::new(self, 31)
    }
}
#[doc = "Timeout register\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMEOUT_SPEC;
impl crate::RegisterSpec for TIMEOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timeout::R`](R) reader structure"]
impl crate::Readable for TIMEOUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timeout::W`](W) writer structure"]
impl crate::Writable for TIMEOUT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMEOUT to value 0"]
impl crate::Resettable for TIMEOUT_SPEC {}
