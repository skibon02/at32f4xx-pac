#[doc = "Register `SCAL` reader"]
pub type R = crate::R<SCAL_SPEC>;
#[doc = "Register `SCAL` writer"]
pub type W = crate::W<SCAL_SPEC>;
#[doc = "Field `DEC` reader - Decrease ERTC clock"]
pub type DEC_R = crate::FieldReader<u16>;
#[doc = "Field `DEC` writer - Decrease ERTC clock"]
pub type DEC_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "16 second calibration period\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAL16_A {
    #[doc = "0: No effect"]
    None = 0,
    #[doc = "1: 16-second calibration"]
    Period16seconds = 1,
}
impl From<CAL16_A> for bool {
    #[inline(always)]
    fn from(variant: CAL16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAL16` reader - 16 second calibration period"]
pub type CAL16_R = crate::BitReader<CAL16_A>;
impl CAL16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAL16_A {
        match self.bits {
            false => CAL16_A::None,
            true => CAL16_A::Period16seconds,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CAL16_A::None
    }
    #[doc = "16-second calibration"]
    #[inline(always)]
    pub fn is_period16seconds(&self) -> bool {
        *self == CAL16_A::Period16seconds
    }
}
#[doc = "Field `CAL16` writer - 16 second calibration period"]
pub type CAL16_W<'a, REG> = crate::BitWriter<'a, REG, CAL16_A>;
impl<'a, REG> CAL16_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(CAL16_A::None)
    }
    #[doc = "16-second calibration"]
    #[inline(always)]
    pub fn period16seconds(self) -> &'a mut crate::W<REG> {
        self.variant(CAL16_A::Period16seconds)
    }
}
#[doc = "8-second calibration period\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAL8_A {
    #[doc = "0: No effect"]
    None = 0,
    #[doc = "1: 8-second calibration"]
    Period8seconds = 1,
}
impl From<CAL8_A> for bool {
    #[inline(always)]
    fn from(variant: CAL8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAL8` reader - 8-second calibration period"]
pub type CAL8_R = crate::BitReader<CAL8_A>;
impl CAL8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAL8_A {
        match self.bits {
            false => CAL8_A::None,
            true => CAL8_A::Period8seconds,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CAL8_A::None
    }
    #[doc = "8-second calibration"]
    #[inline(always)]
    pub fn is_period8seconds(&self) -> bool {
        *self == CAL8_A::Period8seconds
    }
}
#[doc = "Field `CAL8` writer - 8-second calibration period"]
pub type CAL8_W<'a, REG> = crate::BitWriter<'a, REG, CAL8_A>;
impl<'a, REG> CAL8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(CAL8_A::None)
    }
    #[doc = "8-second calibration"]
    #[inline(always)]
    pub fn period8seconds(self) -> &'a mut crate::W<REG> {
        self.variant(CAL8_A::Period8seconds)
    }
}
#[doc = "Add ERTC clock\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADD_A {
    #[doc = "0: No effect"]
    None = 0,
    #[doc = "1: One ERTC_CLK is inserted every 2^11 ERTC_CLK cycles"]
    OneClkEvery2_11cycles = 1,
}
impl From<ADD_A> for bool {
    #[inline(always)]
    fn from(variant: ADD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADD` reader - Add ERTC clock"]
pub type ADD_R = crate::BitReader<ADD_A>;
impl ADD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADD_A {
        match self.bits {
            false => ADD_A::None,
            true => ADD_A::OneClkEvery2_11cycles,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ADD_A::None
    }
    #[doc = "One ERTC_CLK is inserted every 2^11 ERTC_CLK cycles"]
    #[inline(always)]
    pub fn is_one_clk_every2_11cycles(&self) -> bool {
        *self == ADD_A::OneClkEvery2_11cycles
    }
}
#[doc = "Field `ADD` writer - Add ERTC clock"]
pub type ADD_W<'a, REG> = crate::BitWriter<'a, REG, ADD_A>;
impl<'a, REG> ADD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(ADD_A::None)
    }
    #[doc = "One ERTC_CLK is inserted every 2^11 ERTC_CLK cycles"]
    #[inline(always)]
    pub fn one_clk_every2_11cycles(self) -> &'a mut crate::W<REG> {
        self.variant(ADD_A::OneClkEvery2_11cycles)
    }
}
impl R {
    #[doc = "Bits 0:8 - Decrease ERTC clock"]
    #[inline(always)]
    pub fn dec(&self) -> DEC_R {
        DEC_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 13 - 16 second calibration period"]
    #[inline(always)]
    pub fn cal16(&self) -> CAL16_R {
        CAL16_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 8-second calibration period"]
    #[inline(always)]
    pub fn cal8(&self) -> CAL8_R {
        CAL8_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Add ERTC clock"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCAL")
            .field("add", &self.add())
            .field("cal8", &self.cal8())
            .field("cal16", &self.cal16())
            .field("dec", &self.dec())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - Decrease ERTC clock"]
    #[inline(always)]
    pub fn dec(&mut self) -> DEC_W<'_, SCAL_SPEC> {
        DEC_W::new(self, 0)
    }
    #[doc = "Bit 13 - 16 second calibration period"]
    #[inline(always)]
    pub fn cal16(&mut self) -> CAL16_W<'_, SCAL_SPEC> {
        CAL16_W::new(self, 13)
    }
    #[doc = "Bit 14 - 8-second calibration period"]
    #[inline(always)]
    pub fn cal8(&mut self) -> CAL8_W<'_, SCAL_SPEC> {
        CAL8_W::new(self, 14)
    }
    #[doc = "Bit 15 - Add ERTC clock"]
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W<'_, SCAL_SPEC> {
        ADD_W::new(self, 15)
    }
}
#[doc = "calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`scal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCAL_SPEC;
impl crate::RegisterSpec for SCAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scal::R`](R) reader structure"]
impl crate::Readable for SCAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scal::W`](W) writer structure"]
impl crate::Writable for SCAL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCAL to value 0"]
impl crate::Resettable for SCAL_SPEC {}
