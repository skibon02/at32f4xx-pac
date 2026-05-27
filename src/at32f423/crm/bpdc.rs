#[doc = "Register `BPDC` reader"]
pub type R = crate::R<BPDC_SPEC>;
#[doc = "Register `BPDC` writer"]
pub type W = crate::W<BPDC_SPEC>;
#[doc = "Low speed external crystal enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lextenr {
    #[doc = "0: Low speed external crystal is disabled"]
    Disabled = 0,
    #[doc = "1: Low speed external crystal is enabled"]
    Enabled = 1,
}
impl From<Lextenr> for bool {
    #[inline(always)]
    fn from(variant: Lextenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEXTEN` reader - Low speed external crystal enable"]
pub type LEXTEN_R = crate::BitReader<Lextenr>;
impl LEXTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lextenr {
        match self.bits {
            false => Lextenr::Disabled,
            true => Lextenr::Enabled,
        }
    }
    #[doc = "Low speed external crystal is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lextenr::Disabled
    }
    #[doc = "Low speed external crystal is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lextenr::Enabled
    }
}
#[doc = "Low speed external crystal enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LextenwWO {
    #[doc = "0: Low speed external crystal disable"]
    Disable = 0,
    #[doc = "1: Low speed external crystal enable"]
    Enable = 1,
}
impl From<LextenwWO> for bool {
    #[inline(always)]
    fn from(variant: LextenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEXTEN` writer - Low speed external crystal enable"]
pub type LEXTEN_W<'a, REG> = crate::BitWriter<'a, REG, LextenwWO>;
impl<'a, REG> LEXTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low speed external crystal disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LextenwWO::Disable)
    }
    #[doc = "Low speed external crystal enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LextenwWO::Enable)
    }
}
#[doc = "Low speed external crystal ready\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LEXTSTBL_A {
    #[doc = "0: LEXT is not ready"]
    NotReady = 0,
    #[doc = "1: LEXT is ready"]
    Ready = 1,
}
impl From<LEXTSTBL_A> for bool {
    #[inline(always)]
    fn from(variant: LEXTSTBL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEXTSTBL` reader - Low speed external crystal ready"]
pub type LEXTSTBL_R = crate::BitReader<LEXTSTBL_A>;
impl LEXTSTBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LEXTSTBL_A {
        match self.bits {
            false => LEXTSTBL_A::NotReady,
            true => LEXTSTBL_A::Ready,
        }
    }
    #[doc = "LEXT is not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LEXTSTBL_A::NotReady
    }
    #[doc = "LEXT is ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LEXTSTBL_A::Ready
    }
}
#[doc = "Low speed external crystal bypass\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lextbypsr {
    #[doc = "0: LEXT bypass is disabled"]
    Disabled = 0,
    #[doc = "1: LEXT bypass is enabled"]
    Enabled = 1,
}
impl From<Lextbypsr> for bool {
    #[inline(always)]
    fn from(variant: Lextbypsr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEXTBYPS` reader - Low speed external crystal bypass"]
pub type LEXTBYPS_R = crate::BitReader<Lextbypsr>;
impl LEXTBYPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lextbypsr {
        match self.bits {
            false => Lextbypsr::Disabled,
            true => Lextbypsr::Enabled,
        }
    }
    #[doc = "LEXT bypass is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lextbypsr::Disabled
    }
    #[doc = "LEXT bypass is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lextbypsr::Enabled
    }
}
#[doc = "Low speed external crystal bypass\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LextbypswWO {
    #[doc = "0: LEXT bypass disable"]
    Disable = 0,
    #[doc = "1: LEXT bypass enable"]
    Enable = 1,
}
impl From<LextbypswWO> for bool {
    #[inline(always)]
    fn from(variant: LextbypswWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEXTBYPS` writer - Low speed external crystal bypass"]
pub type LEXTBYPS_W<'a, REG> = crate::BitWriter<'a, REG, LextbypswWO>;
impl<'a, REG> LEXTBYPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LEXT bypass disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LextbypswWO::Disable)
    }
    #[doc = "LEXT bypass enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LextbypswWO::Enable)
    }
}
#[doc = "ERTC clock source selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ERTCSEL_A {
    #[doc = "0: No clock"]
    NoClock = 0,
    #[doc = "1: LEXT"]
    Lext = 1,
    #[doc = "2: LICK"]
    Lick = 2,
    #[doc = "3: Divided HEXT (with the ERTC_DIV bit in the CRM_CFG)"]
    Hext = 3,
}
impl From<ERTCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ERTCSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ERTCSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for ERTCSEL_A {}
#[doc = "Field `ERTCSEL` reader - ERTC clock source selection"]
pub type ERTCSEL_R = crate::FieldReader<ERTCSEL_A>;
impl ERTCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERTCSEL_A {
        match self.bits {
            0 => ERTCSEL_A::NoClock,
            1 => ERTCSEL_A::Lext,
            2 => ERTCSEL_A::Lick,
            3 => ERTCSEL_A::Hext,
            _ => unreachable!(),
        }
    }
    #[doc = "No clock"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == ERTCSEL_A::NoClock
    }
    #[doc = "LEXT"]
    #[inline(always)]
    pub fn is_lext(&self) -> bool {
        *self == ERTCSEL_A::Lext
    }
    #[doc = "LICK"]
    #[inline(always)]
    pub fn is_lick(&self) -> bool {
        *self == ERTCSEL_A::Lick
    }
    #[doc = "Divided HEXT (with the ERTC_DIV bit in the CRM_CFG)"]
    #[inline(always)]
    pub fn is_hext(&self) -> bool {
        *self == ERTCSEL_A::Hext
    }
}
#[doc = "Field `ERTCSEL` writer - ERTC clock source selection"]
pub type ERTCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ERTCSEL_A, crate::Safe>;
impl<'a, REG> ERTCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut crate::W<REG> {
        self.variant(ERTCSEL_A::NoClock)
    }
    #[doc = "LEXT"]
    #[inline(always)]
    pub fn lext(self) -> &'a mut crate::W<REG> {
        self.variant(ERTCSEL_A::Lext)
    }
    #[doc = "LICK"]
    #[inline(always)]
    pub fn lick(self) -> &'a mut crate::W<REG> {
        self.variant(ERTCSEL_A::Lick)
    }
    #[doc = "Divided HEXT (with the ERTC_DIV bit in the CRM_CFG)"]
    #[inline(always)]
    pub fn hext(self) -> &'a mut crate::W<REG> {
        self.variant(ERTCSEL_A::Hext)
    }
}
#[doc = "ERTC clock enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERTCEN_A {
    #[doc = "0: ERTC clock is disabled"]
    Disabled = 0,
    #[doc = "1: ERTC clock is enabled"]
    Enabled = 1,
}
impl From<ERTCEN_A> for bool {
    #[inline(always)]
    fn from(variant: ERTCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERTCEN` reader - ERTC clock enable"]
pub type ERTCEN_R = crate::BitReader<ERTCEN_A>;
impl ERTCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERTCEN_A {
        match self.bits {
            false => ERTCEN_A::Disabled,
            true => ERTCEN_A::Enabled,
        }
    }
    #[doc = "ERTC clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERTCEN_A::Disabled
    }
    #[doc = "ERTC clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERTCEN_A::Enabled
    }
}
#[doc = "Field `ERTCEN` writer - ERTC clock enable"]
pub type ERTCEN_W<'a, REG> = crate::BitWriter<'a, REG, ERTCEN_A>;
impl<'a, REG> ERTCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ERTC clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERTCEN_A::Disabled)
    }
    #[doc = "ERTC clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERTCEN_A::Enabled)
    }
}
#[doc = "Battery powered domain software reset\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BPDRSTW_A {
    #[doc = "1: Backup domain software reset"]
    Reset = 1,
}
impl From<BPDRSTW_A> for bool {
    #[inline(always)]
    fn from(variant: BPDRSTW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDRST` reader - Battery powered domain software reset"]
pub type BPDRST_R = crate::BitReader<BPDRSTW_A>;
impl BPDRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BPDRSTW_A> {
        match self.bits {
            true => Some(BPDRSTW_A::Reset),
            _ => None,
        }
    }
    #[doc = "Backup domain software reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == BPDRSTW_A::Reset
    }
}
#[doc = "Field `BPDRST` writer - Battery powered domain software reset"]
pub type BPDRST_W<'a, REG> = crate::BitWriter1S<'a, REG, BPDRSTW_A>;
impl<'a, REG> BPDRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Backup domain software reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(BPDRSTW_A::Reset)
    }
}
impl R {
    #[doc = "Bit 0 - Low speed external crystal enable"]
    #[inline(always)]
    pub fn lexten(&self) -> LEXTEN_R {
        LEXTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low speed external crystal ready"]
    #[inline(always)]
    pub fn lextstbl(&self) -> LEXTSTBL_R {
        LEXTSTBL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low speed external crystal bypass"]
    #[inline(always)]
    pub fn lextbyps(&self) -> LEXTBYPS_R {
        LEXTBYPS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9 - ERTC clock source selection"]
    #[inline(always)]
    pub fn ertcsel(&self) -> ERTCSEL_R {
        ERTCSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - ERTC clock enable"]
    #[inline(always)]
    pub fn ertcen(&self) -> ERTCEN_R {
        ERTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Battery powered domain software reset"]
    #[inline(always)]
    pub fn bpdrst(&self) -> BPDRST_R {
        BPDRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BPDC")
            .field("lexten", &self.lexten())
            .field("lextstbl", &self.lextstbl())
            .field("lextbyps", &self.lextbyps())
            .field("ertcsel", &self.ertcsel())
            .field("ertcen", &self.ertcen())
            .field("bpdrst", &self.bpdrst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Low speed external crystal enable"]
    #[inline(always)]
    pub fn lexten(&mut self) -> LEXTEN_W<'_, BPDC_SPEC> {
        LEXTEN_W::new(self, 0)
    }
    #[doc = "Bit 2 - Low speed external crystal bypass"]
    #[inline(always)]
    pub fn lextbyps(&mut self) -> LEXTBYPS_W<'_, BPDC_SPEC> {
        LEXTBYPS_W::new(self, 2)
    }
    #[doc = "Bits 8:9 - ERTC clock source selection"]
    #[inline(always)]
    pub fn ertcsel(&mut self) -> ERTCSEL_W<'_, BPDC_SPEC> {
        ERTCSEL_W::new(self, 8)
    }
    #[doc = "Bit 15 - ERTC clock enable"]
    #[inline(always)]
    pub fn ertcen(&mut self) -> ERTCEN_W<'_, BPDC_SPEC> {
        ERTCEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Battery powered domain software reset"]
    #[inline(always)]
    pub fn bpdrst(&mut self) -> BPDRST_W<'_, BPDC_SPEC> {
        BPDRST_W::new(self, 16)
    }
}
#[doc = "Battery powered domain control register (CRM_BPDC)\n\nYou can [`read`](crate::Reg::read) this register and get [`bpdc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bpdc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BPDC_SPEC;
impl crate::RegisterSpec for BPDC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bpdc::R`](R) reader structure"]
impl crate::Readable for BPDC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bpdc::W`](W) writer structure"]
impl crate::Writable for BPDC_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0001_0000;
}
#[doc = "`reset()` method sets BPDC to value 0"]
impl crate::Resettable for BPDC_SPEC {}
