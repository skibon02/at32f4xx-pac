#[doc = "Register `CTRLSTS` reader"]
pub type R = crate::R<CTRLSTS_SPEC>;
#[doc = "Register `CTRLSTS` writer"]
pub type W = crate::W<CTRLSTS_SPEC>;
#[doc = "Low speed internal clock enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lickenr {
    #[doc = "0: LICK is disabled"]
    Disabled = 0,
    #[doc = "1: LICK is enabled"]
    Enabled = 1,
}
impl From<Lickenr> for bool {
    #[inline(always)]
    fn from(variant: Lickenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LICKEN` reader - Low speed internal clock enable"]
pub type LICKEN_R = crate::BitReader<Lickenr>;
impl LICKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lickenr {
        match self.bits {
            false => Lickenr::Disabled,
            true => Lickenr::Enabled,
        }
    }
    #[doc = "LICK is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lickenr::Disabled
    }
    #[doc = "LICK is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lickenr::Enabled
    }
}
#[doc = "Low speed internal clock enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LickenwWO {
    #[doc = "0: Low speed internal clock disable"]
    Disable = 0,
    #[doc = "1: Low speed internal clock enable"]
    Enable = 1,
}
impl From<LickenwWO> for bool {
    #[inline(always)]
    fn from(variant: LickenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LICKEN` writer - Low speed internal clock enable"]
pub type LICKEN_W<'a, REG> = crate::BitWriter<'a, REG, LickenwWO>;
impl<'a, REG> LICKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low speed internal clock disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LickenwWO::Disable)
    }
    #[doc = "Low speed internal clock enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LickenwWO::Enable)
    }
}
#[doc = "Low speed internal clock ready\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LICKSTBL_A {
    #[doc = "0: LICK is not ready"]
    NotReady = 0,
    #[doc = "1: LICK is ready"]
    Ready = 1,
}
impl From<LICKSTBL_A> for bool {
    #[inline(always)]
    fn from(variant: LICKSTBL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LICKSTBL` reader - Low speed internal clock ready"]
pub type LICKSTBL_R = crate::BitReader<LICKSTBL_A>;
impl LICKSTBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LICKSTBL_A {
        match self.bits {
            false => LICKSTBL_A::NotReady,
            true => LICKSTBL_A::Ready,
        }
    }
    #[doc = "LICK is not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LICKSTBL_A::NotReady
    }
    #[doc = "LICK is ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LICKSTBL_A::Ready
    }
}
#[doc = "Reset reset flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTFCW_A {
    #[doc = "0: Clear reset flags *RSTF"]
    Clear = 0,
}
impl From<RSTFCW_A> for bool {
    #[inline(always)]
    fn from(variant: RSTFCW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTFC` reader - Reset reset flag"]
pub type RSTFC_R = crate::BitReader<RSTFCW_A>;
impl RSTFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RSTFCW_A> {
        match self.bits {
            false => Some(RSTFCW_A::Clear),
            _ => None,
        }
    }
    #[doc = "Clear reset flags *RSTF"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RSTFCW_A::Clear
    }
}
#[doc = "Field `RSTFC` writer - Reset reset flag"]
pub type RSTFC_W<'a, REG> = crate::BitWriter1C<'a, REG, RSTFCW_A>;
impl<'a, REG> RSTFC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear reset flags *RSTF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RSTFCW_A::Clear)
    }
}
#[doc = "PIN reset flag\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NRSTF_A {
    #[doc = "0: No NRST pin reset occurred"]
    NoReset = 0,
    #[doc = "1: NRST pin reset occurred"]
    Reset = 1,
}
impl From<NRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: NRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NRSTF` reader - PIN reset flag"]
pub type NRSTF_R = crate::BitReader<NRSTF_A>;
impl NRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NRSTF_A {
        match self.bits {
            false => NRSTF_A::NoReset,
            true => NRSTF_A::Reset,
        }
    }
    #[doc = "No NRST pin reset occurred"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == NRSTF_A::NoReset
    }
    #[doc = "NRST pin reset occurred"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == NRSTF_A::Reset
    }
}
#[doc = "POR/LVR reset flag\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORRSTF_A {
    #[doc = "0: No POR/LVR reset occurred"]
    NoReset = 0,
    #[doc = "1: POR/LVR reset occurred"]
    Reset = 1,
}
impl From<PORRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: PORRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORRSTF` reader - POR/LVR reset flag"]
pub type PORRSTF_R = crate::BitReader<PORRSTF_A>;
impl PORRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PORRSTF_A {
        match self.bits {
            false => PORRSTF_A::NoReset,
            true => PORRSTF_A::Reset,
        }
    }
    #[doc = "No POR/LVR reset occurred"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == PORRSTF_A::NoReset
    }
    #[doc = "POR/LVR reset occurred"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PORRSTF_A::Reset
    }
}
#[doc = "Software reset flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWRSTF_A {
    #[doc = "0: No software reset occurred"]
    NoReset = 0,
    #[doc = "1: Software reset occurred"]
    Reset = 1,
}
impl From<SWRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: SWRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRSTF` reader - Software reset flag"]
pub type SWRSTF_R = crate::BitReader<SWRSTF_A>;
impl SWRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWRSTF_A {
        match self.bits {
            false => SWRSTF_A::NoReset,
            true => SWRSTF_A::Reset,
        }
    }
    #[doc = "No software reset occurred"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == SWRSTF_A::NoReset
    }
    #[doc = "Software reset occurred"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SWRSTF_A::Reset
    }
}
#[doc = "Watchdog timer reset flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTRSTF_A {
    #[doc = "0: No watchdog reset occurred"]
    NoReset = 0,
    #[doc = "1: Watchdog reset occurred"]
    Reset = 1,
}
impl From<WDTRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: WDTRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTRSTF` reader - Watchdog timer reset flag"]
pub type WDTRSTF_R = crate::BitReader<WDTRSTF_A>;
impl WDTRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDTRSTF_A {
        match self.bits {
            false => WDTRSTF_A::NoReset,
            true => WDTRSTF_A::Reset,
        }
    }
    #[doc = "No watchdog reset occurred"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == WDTRSTF_A::NoReset
    }
    #[doc = "Watchdog reset occurred"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WDTRSTF_A::Reset
    }
}
#[doc = "Window watchdog timer reset flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDTRSTF_A {
    #[doc = "0: No window watchdog reset occurred"]
    NoReset = 0,
    #[doc = "1: Window watchdog reset occurred"]
    Reset = 1,
}
impl From<WWDTRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: WWDTRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDTRSTF` reader - Window watchdog timer reset flag"]
pub type WWDTRSTF_R = crate::BitReader<WWDTRSTF_A>;
impl WWDTRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WWDTRSTF_A {
        match self.bits {
            false => WWDTRSTF_A::NoReset,
            true => WWDTRSTF_A::Reset,
        }
    }
    #[doc = "No window watchdog reset occurred"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == WWDTRSTF_A::NoReset
    }
    #[doc = "Window watchdog reset occurred"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WWDTRSTF_A::Reset
    }
}
#[doc = "Low-power reset flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPRSTF_A {
    #[doc = "0: No low power reset occurred"]
    NoReset = 0,
    #[doc = "1: Low power reset occurred"]
    Reset = 1,
}
impl From<LPRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: LPRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPRSTF` reader - Low-power reset flag"]
pub type LPRSTF_R = crate::BitReader<LPRSTF_A>;
impl LPRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPRSTF_A {
        match self.bits {
            false => LPRSTF_A::NoReset,
            true => LPRSTF_A::Reset,
        }
    }
    #[doc = "No low power reset occurred"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == LPRSTF_A::NoReset
    }
    #[doc = "Low power reset occurred"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == LPRSTF_A::Reset
    }
}
impl R {
    #[doc = "Bit 0 - Low speed internal clock enable"]
    #[inline(always)]
    pub fn licken(&self) -> LICKEN_R {
        LICKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low speed internal clock ready"]
    #[inline(always)]
    pub fn lickstbl(&self) -> LICKSTBL_R {
        LICKSTBL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 24 - Reset reset flag"]
    #[inline(always)]
    pub fn rstfc(&self) -> RSTFC_R {
        RSTFC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - PIN reset flag"]
    #[inline(always)]
    pub fn nrstf(&self) -> NRSTF_R {
        NRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - POR/LVR reset flag"]
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn swrstf(&self) -> SWRSTF_R {
        SWRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Watchdog timer reset flag"]
    #[inline(always)]
    pub fn wdtrstf(&self) -> WDTRSTF_R {
        WDTRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Window watchdog timer reset flag"]
    #[inline(always)]
    pub fn wwdtrstf(&self) -> WWDTRSTF_R {
        WWDTRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lprstf(&self) -> LPRSTF_R {
        LPRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRLSTS")
            .field("licken", &self.licken())
            .field("lickstbl", &self.lickstbl())
            .field("rstfc", &self.rstfc())
            .field("nrstf", &self.nrstf())
            .field("porrstf", &self.porrstf())
            .field("swrstf", &self.swrstf())
            .field("wdtrstf", &self.wdtrstf())
            .field("wwdtrstf", &self.wwdtrstf())
            .field("lprstf", &self.lprstf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Low speed internal clock enable"]
    #[inline(always)]
    pub fn licken(&mut self) -> LICKEN_W<'_, CTRLSTS_SPEC> {
        LICKEN_W::new(self, 0)
    }
    #[doc = "Bit 24 - Reset reset flag"]
    #[inline(always)]
    pub fn rstfc(&mut self) -> RSTFC_W<'_, CTRLSTS_SPEC> {
        RSTFC_W::new(self, 24)
    }
}
#[doc = "Control/status register (CRM_CTRLSTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLSTS_SPEC;
impl crate::RegisterSpec for CTRLSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlsts::R`](R) reader structure"]
impl crate::Readable for CTRLSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlsts::W`](W) writer structure"]
impl crate::Writable for CTRLSTS_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0100_0000;
}
#[doc = "`reset()` method sets CTRLSTS to value 0x0c00_0000"]
impl crate::Resettable for CTRLSTS_SPEC {
    const RESET_VALUE: u32 = 0x0c00_0000;
}
