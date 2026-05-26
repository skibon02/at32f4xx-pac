#[doc = "Register `TAMP` reader"]
pub type R = crate::R<TAMP_SPEC>;
#[doc = "Register `TAMP` writer"]
pub type W = crate::W<TAMP_SPEC>;
#[doc = "Tamper detection 1 enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TP1EN_A {
    #[doc = "0: Tamper detection disabled"]
    Disabled = 0,
    #[doc = "1: Tamper detection enabled"]
    Enabled = 1,
}
impl From<TP1EN_A> for bool {
    #[inline(always)]
    fn from(variant: TP1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP1EN` reader - Tamper detection 1 enable"]
pub type TP1EN_R = crate::BitReader<TP1EN_A>;
impl TP1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TP1EN_A {
        match self.bits {
            false => TP1EN_A::Disabled,
            true => TP1EN_A::Enabled,
        }
    }
    #[doc = "Tamper detection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TP1EN_A::Disabled
    }
    #[doc = "Tamper detection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TP1EN_A::Enabled
    }
}
#[doc = "Field `TP1EN` writer - Tamper detection 1 enable"]
pub type TP1EN_W<'a, REG> = crate::BitWriter<'a, REG, TP1EN_A>;
impl<'a, REG> TP1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper detection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TP1EN_A::Disabled)
    }
    #[doc = "Tamper detection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TP1EN_A::Enabled)
    }
}
#[doc = "Tamper detection 1 valid edge\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TP1EDG_A {
    #[doc = "0: If TPFLT=0: Rising edge. If TPFLT>0: low level"]
    RisingOrLow = 0,
    #[doc = "1: If TPFLT=0: Falling edge. If TPFLT>0: High level"]
    FallingOrHigh = 1,
}
impl From<TP1EDG_A> for bool {
    #[inline(always)]
    fn from(variant: TP1EDG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP1EDG` reader - Tamper detection 1 valid edge"]
pub type TP1EDG_R = crate::BitReader<TP1EDG_A>;
impl TP1EDG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TP1EDG_A {
        match self.bits {
            false => TP1EDG_A::RisingOrLow,
            true => TP1EDG_A::FallingOrHigh,
        }
    }
    #[doc = "If TPFLT=0: Rising edge. If TPFLT>0: low level"]
    #[inline(always)]
    pub fn is_rising_or_low(&self) -> bool {
        *self == TP1EDG_A::RisingOrLow
    }
    #[doc = "If TPFLT=0: Falling edge. If TPFLT>0: High level"]
    #[inline(always)]
    pub fn is_falling_or_high(&self) -> bool {
        *self == TP1EDG_A::FallingOrHigh
    }
}
#[doc = "Field `TP1EDG` writer - Tamper detection 1 valid edge"]
pub type TP1EDG_W<'a, REG> = crate::BitWriter<'a, REG, TP1EDG_A>;
impl<'a, REG> TP1EDG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If TPFLT=0: Rising edge. If TPFLT>0: low level"]
    #[inline(always)]
    pub fn rising_or_low(self) -> &'a mut crate::W<REG> {
        self.variant(TP1EDG_A::RisingOrLow)
    }
    #[doc = "If TPFLT=0: Falling edge. If TPFLT>0: High level"]
    #[inline(always)]
    pub fn falling_or_high(self) -> &'a mut crate::W<REG> {
        self.variant(TP1EDG_A::FallingOrHigh)
    }
}
#[doc = "Tamper detection interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPIEN_A {
    #[doc = "0: Tamper interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Tamper interrupt enabled"]
    Enabled = 1,
}
impl From<TPIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TPIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPIEN` reader - Tamper detection interrupt enable"]
pub type TPIEN_R = crate::BitReader<TPIEN_A>;
impl TPIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TPIEN_A {
        match self.bits {
            false => TPIEN_A::Disabled,
            true => TPIEN_A::Enabled,
        }
    }
    #[doc = "Tamper interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TPIEN_A::Disabled
    }
    #[doc = "Tamper interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TPIEN_A::Enabled
    }
}
#[doc = "Field `TPIEN` writer - Tamper detection interrupt enable"]
pub type TPIEN_W<'a, REG> = crate::BitWriter<'a, REG, TPIEN_A>;
impl<'a, REG> TPIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TPIEN_A::Disabled)
    }
    #[doc = "Tamper interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TPIEN_A::Enabled)
    }
}
#[doc = "Tamper detection timestamp enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPTSEN_A {
    #[doc = "0: Tamper detection timestamp disabled"]
    Disabled = 0,
    #[doc = "1: Tamper detection timestamp enabled"]
    Enabled = 1,
}
impl From<TPTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: TPTSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPTSEN` reader - Tamper detection timestamp enable"]
pub type TPTSEN_R = crate::BitReader<TPTSEN_A>;
impl TPTSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TPTSEN_A {
        match self.bits {
            false => TPTSEN_A::Disabled,
            true => TPTSEN_A::Enabled,
        }
    }
    #[doc = "Tamper detection timestamp disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TPTSEN_A::Disabled
    }
    #[doc = "Tamper detection timestamp enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TPTSEN_A::Enabled
    }
}
#[doc = "Field `TPTSEN` writer - Tamper detection timestamp enable"]
pub type TPTSEN_W<'a, REG> = crate::BitWriter<'a, REG, TPTSEN_A>;
impl<'a, REG> TPTSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper detection timestamp disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TPTSEN_A::Disabled)
    }
    #[doc = "Tamper detection timestamp enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TPTSEN_A::Enabled)
    }
}
#[doc = "Tamper detection frequency\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TPFREQ_A {
    #[doc = "0: ERTC_CLK/32768"]
    Div32768 = 0,
    #[doc = "1: ERTC_CLK/16384"]
    Div16384 = 1,
    #[doc = "2: ERTC_CLK/8192"]
    Div8192 = 2,
    #[doc = "3: ERTC_CLK/4096"]
    Div4096 = 3,
    #[doc = "4: ERTC_CLK/2048"]
    Div2048 = 4,
    #[doc = "5: ERTC_CLK/1024"]
    Div1024 = 5,
    #[doc = "6: ERTC_CLK/512"]
    Div512 = 6,
    #[doc = "7: ERTC_CLK/256"]
    Div256 = 7,
}
impl From<TPFREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: TPFREQ_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TPFREQ_A {
    type Ux = u8;
}
impl crate::IsEnum for TPFREQ_A {}
#[doc = "Field `TPFREQ` reader - Tamper detection frequency"]
pub type TPFREQ_R = crate::FieldReader<TPFREQ_A>;
impl TPFREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TPFREQ_A {
        match self.bits {
            0 => TPFREQ_A::Div32768,
            1 => TPFREQ_A::Div16384,
            2 => TPFREQ_A::Div8192,
            3 => TPFREQ_A::Div4096,
            4 => TPFREQ_A::Div2048,
            5 => TPFREQ_A::Div1024,
            6 => TPFREQ_A::Div512,
            7 => TPFREQ_A::Div256,
            _ => unreachable!(),
        }
    }
    #[doc = "ERTC_CLK/32768"]
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == TPFREQ_A::Div32768
    }
    #[doc = "ERTC_CLK/16384"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == TPFREQ_A::Div16384
    }
    #[doc = "ERTC_CLK/8192"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == TPFREQ_A::Div8192
    }
    #[doc = "ERTC_CLK/4096"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == TPFREQ_A::Div4096
    }
    #[doc = "ERTC_CLK/2048"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == TPFREQ_A::Div2048
    }
    #[doc = "ERTC_CLK/1024"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == TPFREQ_A::Div1024
    }
    #[doc = "ERTC_CLK/512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == TPFREQ_A::Div512
    }
    #[doc = "ERTC_CLK/256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == TPFREQ_A::Div256
    }
}
#[doc = "Field `TPFREQ` writer - Tamper detection frequency"]
pub type TPFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TPFREQ_A, crate::Safe>;
impl<'a, REG> TPFREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ERTC_CLK/32768"]
    #[inline(always)]
    pub fn div32768(self) -> &'a mut crate::W<REG> {
        self.variant(TPFREQ_A::Div32768)
    }
    #[doc = "ERTC_CLK/16384"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut crate::W<REG> {
        self.variant(TPFREQ_A::Div16384)
    }
    #[doc = "ERTC_CLK/8192"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut crate::W<REG> {
        self.variant(TPFREQ_A::Div8192)
    }
    #[doc = "ERTC_CLK/4096"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut crate::W<REG> {
        self.variant(TPFREQ_A::Div4096)
    }
    #[doc = "ERTC_CLK/2048"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut crate::W<REG> {
        self.variant(TPFREQ_A::Div2048)
    }
    #[doc = "ERTC_CLK/1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(TPFREQ_A::Div1024)
    }
    #[doc = "ERTC_CLK/512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(TPFREQ_A::Div512)
    }
    #[doc = "ERTC_CLK/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(TPFREQ_A::Div256)
    }
}
#[doc = "Tamper detection filter time\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TPFLT_A {
    #[doc = "0: No filter"]
    NoFilter = 0,
    #[doc = "1: Tamper is detected after 2 consecutive samples"]
    Filter2 = 1,
    #[doc = "2: Tamper is detected after 4 consecutive samples"]
    Filter4 = 2,
    #[doc = "3: Tamper is detected after 8 consecutive samples"]
    Filter8 = 3,
}
impl From<TPFLT_A> for u8 {
    #[inline(always)]
    fn from(variant: TPFLT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TPFLT_A {
    type Ux = u8;
}
impl crate::IsEnum for TPFLT_A {}
#[doc = "Field `TPFLT` reader - Tamper detection filter time"]
pub type TPFLT_R = crate::FieldReader<TPFLT_A>;
impl TPFLT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TPFLT_A {
        match self.bits {
            0 => TPFLT_A::NoFilter,
            1 => TPFLT_A::Filter2,
            2 => TPFLT_A::Filter4,
            3 => TPFLT_A::Filter8,
            _ => unreachable!(),
        }
    }
    #[doc = "No filter"]
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == TPFLT_A::NoFilter
    }
    #[doc = "Tamper is detected after 2 consecutive samples"]
    #[inline(always)]
    pub fn is_filter2(&self) -> bool {
        *self == TPFLT_A::Filter2
    }
    #[doc = "Tamper is detected after 4 consecutive samples"]
    #[inline(always)]
    pub fn is_filter4(&self) -> bool {
        *self == TPFLT_A::Filter4
    }
    #[doc = "Tamper is detected after 8 consecutive samples"]
    #[inline(always)]
    pub fn is_filter8(&self) -> bool {
        *self == TPFLT_A::Filter8
    }
}
#[doc = "Field `TPFLT` writer - Tamper detection filter time"]
pub type TPFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TPFLT_A, crate::Safe>;
impl<'a, REG> TPFLT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No filter"]
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut crate::W<REG> {
        self.variant(TPFLT_A::NoFilter)
    }
    #[doc = "Tamper is detected after 2 consecutive samples"]
    #[inline(always)]
    pub fn filter2(self) -> &'a mut crate::W<REG> {
        self.variant(TPFLT_A::Filter2)
    }
    #[doc = "Tamper is detected after 4 consecutive samples"]
    #[inline(always)]
    pub fn filter4(self) -> &'a mut crate::W<REG> {
        self.variant(TPFLT_A::Filter4)
    }
    #[doc = "Tamper is detected after 8 consecutive samples"]
    #[inline(always)]
    pub fn filter8(self) -> &'a mut crate::W<REG> {
        self.variant(TPFLT_A::Filter8)
    }
}
#[doc = "Tamper detection pre-charge time\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TPPR_A {
    #[doc = "0: 1 ERTC_CLK cycle"]
    Cyc1 = 0,
    #[doc = "1: 2 ERTC_CLK cycles"]
    Cyc2 = 1,
    #[doc = "2: 4 ERTC_CLK cycles"]
    Cyc4 = 2,
    #[doc = "3: 8 ERTC_CLK cycles"]
    Cyc8 = 3,
}
impl From<TPPR_A> for u8 {
    #[inline(always)]
    fn from(variant: TPPR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TPPR_A {
    type Ux = u8;
}
impl crate::IsEnum for TPPR_A {}
#[doc = "Field `TPPR` reader - Tamper detection pre-charge time"]
pub type TPPR_R = crate::FieldReader<TPPR_A>;
impl TPPR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TPPR_A {
        match self.bits {
            0 => TPPR_A::Cyc1,
            1 => TPPR_A::Cyc2,
            2 => TPPR_A::Cyc4,
            3 => TPPR_A::Cyc8,
            _ => unreachable!(),
        }
    }
    #[doc = "1 ERTC_CLK cycle"]
    #[inline(always)]
    pub fn is_cyc1(&self) -> bool {
        *self == TPPR_A::Cyc1
    }
    #[doc = "2 ERTC_CLK cycles"]
    #[inline(always)]
    pub fn is_cyc2(&self) -> bool {
        *self == TPPR_A::Cyc2
    }
    #[doc = "4 ERTC_CLK cycles"]
    #[inline(always)]
    pub fn is_cyc4(&self) -> bool {
        *self == TPPR_A::Cyc4
    }
    #[doc = "8 ERTC_CLK cycles"]
    #[inline(always)]
    pub fn is_cyc8(&self) -> bool {
        *self == TPPR_A::Cyc8
    }
}
#[doc = "Field `TPPR` writer - Tamper detection pre-charge time"]
pub type TPPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TPPR_A, crate::Safe>;
impl<'a, REG> TPPR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 ERTC_CLK cycle"]
    #[inline(always)]
    pub fn cyc1(self) -> &'a mut crate::W<REG> {
        self.variant(TPPR_A::Cyc1)
    }
    #[doc = "2 ERTC_CLK cycles"]
    #[inline(always)]
    pub fn cyc2(self) -> &'a mut crate::W<REG> {
        self.variant(TPPR_A::Cyc2)
    }
    #[doc = "4 ERTC_CLK cycles"]
    #[inline(always)]
    pub fn cyc4(self) -> &'a mut crate::W<REG> {
        self.variant(TPPR_A::Cyc4)
    }
    #[doc = "8 ERTC_CLK cycles"]
    #[inline(always)]
    pub fn cyc8(self) -> &'a mut crate::W<REG> {
        self.variant(TPPR_A::Cyc8)
    }
}
#[doc = "Tamper detection pull-up\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPPU_A {
    #[doc = "0: Tamper detection pull-up enabled"]
    Enabled = 0,
    #[doc = "1: Tamper detection pull-up disabled"]
    Disabled = 1,
}
impl From<TPPU_A> for bool {
    #[inline(always)]
    fn from(variant: TPPU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPPU` reader - Tamper detection pull-up"]
pub type TPPU_R = crate::BitReader<TPPU_A>;
impl TPPU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TPPU_A {
        match self.bits {
            false => TPPU_A::Enabled,
            true => TPPU_A::Disabled,
        }
    }
    #[doc = "Tamper detection pull-up enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TPPU_A::Enabled
    }
    #[doc = "Tamper detection pull-up disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TPPU_A::Disabled
    }
}
#[doc = "Field `TPPU` writer - Tamper detection pull-up"]
pub type TPPU_W<'a, REG> = crate::BitWriter<'a, REG, TPPU_A>;
impl<'a, REG> TPPU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper detection pull-up enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TPPU_A::Enabled)
    }
    #[doc = "Tamper detection pull-up disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TPPU_A::Disabled)
    }
}
#[doc = "Output type\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTTYPE_A {
    #[doc = "0: Open-drain"]
    OpenDrain = 0,
    #[doc = "1: Push-pull"]
    PushPull = 1,
}
impl From<OUTTYPE_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTTYPE` reader - Output type"]
pub type OUTTYPE_R = crate::BitReader<OUTTYPE_A>;
impl OUTTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OUTTYPE_A {
        match self.bits {
            false => OUTTYPE_A::OpenDrain,
            true => OUTTYPE_A::PushPull,
        }
    }
    #[doc = "Open-drain"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == OUTTYPE_A::OpenDrain
    }
    #[doc = "Push-pull"]
    #[inline(always)]
    pub fn is_push_pull(&self) -> bool {
        *self == OUTTYPE_A::PushPull
    }
}
#[doc = "Field `OUTTYPE` writer - Output type"]
pub type OUTTYPE_W<'a, REG> = crate::BitWriter<'a, REG, OUTTYPE_A>;
impl<'a, REG> OUTTYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Open-drain"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(OUTTYPE_A::OpenDrain)
    }
    #[doc = "Push-pull"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut crate::W<REG> {
        self.variant(OUTTYPE_A::PushPull)
    }
}
impl R {
    #[doc = "Bit 0 - Tamper detection 1 enable"]
    #[inline(always)]
    pub fn tp1en(&self) -> TP1EN_R {
        TP1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper detection 1 valid edge"]
    #[inline(always)]
    pub fn tp1edg(&self) -> TP1EDG_R {
        TP1EDG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper detection interrupt enable"]
    #[inline(always)]
    pub fn tpien(&self) -> TPIEN_R {
        TPIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Tamper detection timestamp enable"]
    #[inline(always)]
    pub fn tptsen(&self) -> TPTSEN_R {
        TPTSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Tamper detection frequency"]
    #[inline(always)]
    pub fn tpfreq(&self) -> TPFREQ_R {
        TPFREQ_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - Tamper detection filter time"]
    #[inline(always)]
    pub fn tpflt(&self) -> TPFLT_R {
        TPFLT_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - Tamper detection pre-charge time"]
    #[inline(always)]
    pub fn tppr(&self) -> TPPR_R {
        TPPR_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Tamper detection pull-up"]
    #[inline(always)]
    pub fn tppu(&self) -> TPPU_R {
        TPPU_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - Output type"]
    #[inline(always)]
    pub fn outtype(&self) -> OUTTYPE_R {
        OUTTYPE_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TAMP")
            .field("outtype", &self.outtype())
            .field("tppu", &self.tppu())
            .field("tppr", &self.tppr())
            .field("tpflt", &self.tpflt())
            .field("tpfreq", &self.tpfreq())
            .field("tptsen", &self.tptsen())
            .field("tpien", &self.tpien())
            .field("tp1edg", &self.tp1edg())
            .field("tp1en", &self.tp1en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Tamper detection 1 enable"]
    #[inline(always)]
    pub fn tp1en(&mut self) -> TP1EN_W<'_, TAMP_SPEC> {
        TP1EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Tamper detection 1 valid edge"]
    #[inline(always)]
    pub fn tp1edg(&mut self) -> TP1EDG_W<'_, TAMP_SPEC> {
        TP1EDG_W::new(self, 1)
    }
    #[doc = "Bit 2 - Tamper detection interrupt enable"]
    #[inline(always)]
    pub fn tpien(&mut self) -> TPIEN_W<'_, TAMP_SPEC> {
        TPIEN_W::new(self, 2)
    }
    #[doc = "Bit 7 - Tamper detection timestamp enable"]
    #[inline(always)]
    pub fn tptsen(&mut self) -> TPTSEN_W<'_, TAMP_SPEC> {
        TPTSEN_W::new(self, 7)
    }
    #[doc = "Bits 8:10 - Tamper detection frequency"]
    #[inline(always)]
    pub fn tpfreq(&mut self) -> TPFREQ_W<'_, TAMP_SPEC> {
        TPFREQ_W::new(self, 8)
    }
    #[doc = "Bits 11:12 - Tamper detection filter time"]
    #[inline(always)]
    pub fn tpflt(&mut self) -> TPFLT_W<'_, TAMP_SPEC> {
        TPFLT_W::new(self, 11)
    }
    #[doc = "Bits 13:14 - Tamper detection pre-charge time"]
    #[inline(always)]
    pub fn tppr(&mut self) -> TPPR_W<'_, TAMP_SPEC> {
        TPPR_W::new(self, 13)
    }
    #[doc = "Bit 15 - Tamper detection pull-up"]
    #[inline(always)]
    pub fn tppu(&mut self) -> TPPU_W<'_, TAMP_SPEC> {
        TPPU_W::new(self, 15)
    }
    #[doc = "Bit 18 - Output type"]
    #[inline(always)]
    pub fn outtype(&mut self) -> OUTTYPE_W<'_, TAMP_SPEC> {
        OUTTYPE_W::new(self, 18)
    }
}
#[doc = "tamper and alternate function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tamp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAMP_SPEC;
impl crate::RegisterSpec for TAMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamp::R`](R) reader structure"]
impl crate::Readable for TAMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tamp::W`](W) writer structure"]
impl crate::Writable for TAMP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TAMP to value 0"]
impl crate::Resettable for TAMP_SPEC {}
