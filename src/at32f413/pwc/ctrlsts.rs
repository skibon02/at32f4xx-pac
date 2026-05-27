#[doc = "Register `CTRLSTS` reader"]
pub type R = crate::R<CTRLSTS_SPEC>;
#[doc = "Register `CTRLSTS` writer"]
pub type W = crate::W<CTRLSTS_SPEC>;
#[doc = "Standby wake-up event flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWEF_A {
    #[doc = "0: No wakeup event occurred"]
    NoWakeup = 0,
    #[doc = "1: Wakeup event occurred"]
    Wakeup = 1,
}
impl From<SWEF_A> for bool {
    #[inline(always)]
    fn from(variant: SWEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWEF` reader - Standby wake-up event flag"]
pub type SWEF_R = crate::BitReader<SWEF_A>;
impl SWEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWEF_A {
        match self.bits {
            false => SWEF_A::NoWakeup,
            true => SWEF_A::Wakeup,
        }
    }
    #[doc = "No wakeup event occurred"]
    #[inline(always)]
    pub fn is_no_wakeup(&self) -> bool {
        *self == SWEF_A::NoWakeup
    }
    #[doc = "Wakeup event occurred"]
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == SWEF_A::Wakeup
    }
}
#[doc = "Standby mode entry flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEF_A {
    #[doc = "0: Device is not in standby mode"]
    NoStandby = 0,
    #[doc = "1: Device is in standby mode"]
    Standby = 1,
}
impl From<SEF_A> for bool {
    #[inline(always)]
    fn from(variant: SEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEF` reader - Standby mode entry flag"]
pub type SEF_R = crate::BitReader<SEF_A>;
impl SEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SEF_A {
        match self.bits {
            false => SEF_A::NoStandby,
            true => SEF_A::Standby,
        }
    }
    #[doc = "Device is not in standby mode"]
    #[inline(always)]
    pub fn is_no_standby(&self) -> bool {
        *self == SEF_A::NoStandby
    }
    #[doc = "Device is in standby mode"]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == SEF_A::Standby
    }
}
#[doc = "Power voltage monitoring output flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVMOF_A {
    #[doc = "0: Power voltage is higher than the threshold"]
    Higher = 0,
    #[doc = "1: Power voltage is lower than the threshold"]
    Lower = 1,
}
impl From<PVMOF_A> for bool {
    #[inline(always)]
    fn from(variant: PVMOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVMOF` reader - Power voltage monitoring output flag"]
pub type PVMOF_R = crate::BitReader<PVMOF_A>;
impl PVMOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PVMOF_A {
        match self.bits {
            false => PVMOF_A::Higher,
            true => PVMOF_A::Lower,
        }
    }
    #[doc = "Power voltage is higher than the threshold"]
    #[inline(always)]
    pub fn is_higher(&self) -> bool {
        *self == PVMOF_A::Higher
    }
    #[doc = "Power voltage is lower than the threshold"]
    #[inline(always)]
    pub fn is_lower(&self) -> bool {
        *self == PVMOF_A::Lower
    }
}
#[doc = "Standby wake-up pin enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWPEN_A {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<SWPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SWPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWPEN` reader - Standby wake-up pin enable"]
pub type SWPEN_R = crate::BitReader<SWPEN_A>;
impl SWPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWPEN_A {
        match self.bits {
            false => SWPEN_A::Disabled,
            true => SWPEN_A::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SWPEN_A::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SWPEN_A::Enabled
    }
}
#[doc = "Field `SWPEN` writer - Standby wake-up pin enable"]
pub type SWPEN_W<'a, REG> = crate::BitWriter<'a, REG, SWPEN_A>;
impl<'a, REG> SWPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SWPEN_A::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SWPEN_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Standby wake-up event flag"]
    #[inline(always)]
    pub fn swef(&self) -> SWEF_R {
        SWEF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Standby mode entry flag"]
    #[inline(always)]
    pub fn sef(&self) -> SEF_R {
        SEF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power voltage monitoring output flag"]
    #[inline(always)]
    pub fn pvmof(&self) -> PVMOF_R {
        PVMOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Standby wake-up pin enable"]
    #[inline(always)]
    pub fn swpen(&self) -> SWPEN_R {
        SWPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRLSTS")
            .field("swef", &self.swef())
            .field("sef", &self.sef())
            .field("pvmof", &self.pvmof())
            .field("swpen", &self.swpen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - Standby wake-up pin enable"]
    #[inline(always)]
    pub fn swpen(&mut self) -> SWPEN_W<'_, CTRLSTS_SPEC> {
        SWPEN_W::new(self, 8)
    }
}
#[doc = "Power control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLSTS_SPEC;
impl crate::RegisterSpec for CTRLSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlsts::R`](R) reader structure"]
impl crate::Readable for CTRLSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlsts::W`](W) writer structure"]
impl crate::Writable for CTRLSTS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLSTS to value 0"]
impl crate::Resettable for CTRLSTS_SPEC {}
