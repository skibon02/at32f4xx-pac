#[doc = "Register `CMDCTRL` reader"]
pub type R = crate::R<CMDCTRL_SPEC>;
#[doc = "Register `CMDCTRL` writer"]
pub type W = crate::W<CMDCTRL_SPEC>;
#[doc = "Field `CMDIDX` reader - CMDIDX"]
pub type CMDIDX_R = crate::FieldReader;
#[doc = "Field `CMDIDX` writer - CMDIDX"]
pub type CMDIDX_W<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
#[doc = "Wait for response\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSPWT_A {
    #[doc = "1: Short response"]
    ShortResponse = 1,
    #[doc = "3: Long response"]
    LongResponse = 3,
    #[doc = "0: No response"]
    NoResponse = 0,
}
impl From<RSPWT_A> for u8 {
    #[inline(always)]
    fn from(variant: RSPWT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RSPWT_A {
    type Ux = u8;
}
impl crate::IsEnum for RSPWT_A {}
#[doc = "Field `RSPWT` reader - Wait for response"]
pub type RSPWT_R = crate::FieldReader<RSPWT_A>;
impl RSPWT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RSPWT_A {
        match self.bits {
            1 => RSPWT_A::ShortResponse,
            3 => RSPWT_A::LongResponse,
            _ => RSPWT_A::NoResponse,
        }
    }
    #[doc = "Short response"]
    #[inline(always)]
    pub fn is_short_response(&self) -> bool {
        *self == RSPWT_A::ShortResponse
    }
    #[doc = "Long response"]
    #[inline(always)]
    pub fn is_long_response(&self) -> bool {
        *self == RSPWT_A::LongResponse
    }
    #[doc = "No response"]
    #[inline(always)]
    pub fn is_no_response(&self) -> bool {
        matches!(self.variant(), RSPWT_A::NoResponse)
    }
}
#[doc = "Field `RSPWT` writer - Wait for response"]
pub type RSPWT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RSPWT_A, crate::Safe>;
impl<'a, REG> RSPWT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Short response"]
    #[inline(always)]
    pub fn short_response(self) -> &'a mut crate::W<REG> {
        self.variant(RSPWT_A::ShortResponse)
    }
    #[doc = "Long response"]
    #[inline(always)]
    pub fn long_response(self) -> &'a mut crate::W<REG> {
        self.variant(RSPWT_A::LongResponse)
    }
    #[doc = "No response"]
    #[inline(always)]
    pub fn no_response(self) -> &'a mut crate::W<REG> {
        self.variant(RSPWT_A::NoResponse)
    }
}
#[doc = "CCSM wait for interrupt\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTWT_A {
    #[doc = "0: Wait for interrupt before sending command disabled"]
    Disabled = 0,
    #[doc = "1: Wait for interrupt before sending command enabled, CCSM command timeout disabled"]
    Enabled = 1,
}
impl From<INTWT_A> for bool {
    #[inline(always)]
    fn from(variant: INTWT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTWT` reader - CCSM wait for interrupt"]
pub type INTWT_R = crate::BitReader<INTWT_A>;
impl INTWT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INTWT_A {
        match self.bits {
            false => INTWT_A::Disabled,
            true => INTWT_A::Enabled,
        }
    }
    #[doc = "Wait for interrupt before sending command disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTWT_A::Disabled
    }
    #[doc = "Wait for interrupt before sending command enabled, CCSM command timeout disabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTWT_A::Enabled
    }
}
#[doc = "Field `INTWT` writer - CCSM wait for interrupt"]
pub type INTWT_W<'a, REG> = crate::BitWriter<'a, REG, INTWT_A>;
impl<'a, REG> INTWT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wait for interrupt before sending command disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(INTWT_A::Disabled)
    }
    #[doc = "Wait for interrupt before sending command enabled, CCSM command timeout disabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(INTWT_A::Enabled)
    }
}
#[doc = "CCSM wait for end of transfer\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PNDWT_A {
    #[doc = "0: Wait for transfer before sending command disabled"]
    Disabled = 0,
    #[doc = "1: Wait for transfer before sending command enabled"]
    Enabled = 1,
}
impl From<PNDWT_A> for bool {
    #[inline(always)]
    fn from(variant: PNDWT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PNDWT` reader - CCSM wait for end of transfer"]
pub type PNDWT_R = crate::BitReader<PNDWT_A>;
impl PNDWT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PNDWT_A {
        match self.bits {
            false => PNDWT_A::Disabled,
            true => PNDWT_A::Enabled,
        }
    }
    #[doc = "Wait for transfer before sending command disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PNDWT_A::Disabled
    }
    #[doc = "Wait for transfer before sending command enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PNDWT_A::Enabled
    }
}
#[doc = "Field `PNDWT` writer - CCSM wait for end of transfer"]
pub type PNDWT_W<'a, REG> = crate::BitWriter<'a, REG, PNDWT_A>;
impl<'a, REG> PNDWT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wait for transfer before sending command disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PNDWT_A::Disabled)
    }
    #[doc = "Wait for transfer before sending command enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PNDWT_A::Enabled)
    }
}
#[doc = "Command channel state machine\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccsmenr {
    #[doc = "0: Command channel state machine disabled"]
    Disabled = 0,
    #[doc = "1: Command channel state machine enabled"]
    Enabled = 1,
}
impl From<Ccsmenr> for bool {
    #[inline(always)]
    fn from(variant: Ccsmenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCSMEN` reader - Command channel state machine"]
pub type CCSMEN_R = crate::BitReader<Ccsmenr>;
impl CCSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccsmenr {
        match self.bits {
            false => Ccsmenr::Disabled,
            true => Ccsmenr::Enabled,
        }
    }
    #[doc = "Command channel state machine disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ccsmenr::Disabled
    }
    #[doc = "Command channel state machine enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ccsmenr::Enabled
    }
}
#[doc = "Command channel state machine\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CcsmenwWO {
    #[doc = "0: Disable command channel state machine"]
    Disable = 0,
    #[doc = "1: Enable command channel state machine"]
    Enable = 1,
}
impl From<CcsmenwWO> for bool {
    #[inline(always)]
    fn from(variant: CcsmenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCSMEN` writer - Command channel state machine"]
pub type CCSMEN_W<'a, REG> = crate::BitWriter<'a, REG, CcsmenwWO>;
impl<'a, REG> CCSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable command channel state machine"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CcsmenwWO::Disable)
    }
    #[doc = "Enable command channel state machine"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CcsmenwWO::Enable)
    }
}
#[doc = "SD I/O suspend command\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOSUSP_A {
    #[doc = "0: SD I/O suspend command disabled"]
    NotSuspended = 0,
    #[doc = "1: Next command is SD I/O suspend command"]
    Suspended = 1,
}
impl From<IOSUSP_A> for bool {
    #[inline(always)]
    fn from(variant: IOSUSP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOSUSP` reader - SD I/O suspend command"]
pub type IOSUSP_R = crate::BitReader<IOSUSP_A>;
impl IOSUSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IOSUSP_A {
        match self.bits {
            false => IOSUSP_A::NotSuspended,
            true => IOSUSP_A::Suspended,
        }
    }
    #[doc = "SD I/O suspend command disabled"]
    #[inline(always)]
    pub fn is_not_suspended(&self) -> bool {
        *self == IOSUSP_A::NotSuspended
    }
    #[doc = "Next command is SD I/O suspend command"]
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        *self == IOSUSP_A::Suspended
    }
}
#[doc = "Field `IOSUSP` writer - SD I/O suspend command"]
pub type IOSUSP_W<'a, REG> = crate::BitWriter<'a, REG, IOSUSP_A>;
impl<'a, REG> IOSUSP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SD I/O suspend command disabled"]
    #[inline(always)]
    pub fn not_suspended(self) -> &'a mut crate::W<REG> {
        self.variant(IOSUSP_A::NotSuspended)
    }
    #[doc = "Next command is SD I/O suspend command"]
    #[inline(always)]
    pub fn suspended(self) -> &'a mut crate::W<REG> {
        self.variant(IOSUSP_A::Suspended)
    }
}
impl R {
    #[doc = "Bits 0:5 - CMDIDX"]
    #[inline(always)]
    pub fn cmdidx(&self) -> CMDIDX_R {
        CMDIDX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Wait for response"]
    #[inline(always)]
    pub fn rspwt(&self) -> RSPWT_R {
        RSPWT_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - CCSM wait for interrupt"]
    #[inline(always)]
    pub fn intwt(&self) -> INTWT_R {
        INTWT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CCSM wait for end of transfer"]
    #[inline(always)]
    pub fn pndwt(&self) -> PNDWT_R {
        PNDWT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Command channel state machine"]
    #[inline(always)]
    pub fn ccsmen(&self) -> CCSMEN_R {
        CCSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SD I/O suspend command"]
    #[inline(always)]
    pub fn iosusp(&self) -> IOSUSP_R {
        IOSUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDCTRL")
            .field("cmdidx", &self.cmdidx())
            .field("rspwt", &self.rspwt())
            .field("intwt", &self.intwt())
            .field("pndwt", &self.pndwt())
            .field("ccsmen", &self.ccsmen())
            .field("iosusp", &self.iosusp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - CMDIDX"]
    #[inline(always)]
    pub fn cmdidx(&mut self) -> CMDIDX_W<'_, CMDCTRL_SPEC> {
        CMDIDX_W::new(self, 0)
    }
    #[doc = "Bits 6:7 - Wait for response"]
    #[inline(always)]
    pub fn rspwt(&mut self) -> RSPWT_W<'_, CMDCTRL_SPEC> {
        RSPWT_W::new(self, 6)
    }
    #[doc = "Bit 8 - CCSM wait for interrupt"]
    #[inline(always)]
    pub fn intwt(&mut self) -> INTWT_W<'_, CMDCTRL_SPEC> {
        INTWT_W::new(self, 8)
    }
    #[doc = "Bit 9 - CCSM wait for end of transfer"]
    #[inline(always)]
    pub fn pndwt(&mut self) -> PNDWT_W<'_, CMDCTRL_SPEC> {
        PNDWT_W::new(self, 9)
    }
    #[doc = "Bit 10 - Command channel state machine"]
    #[inline(always)]
    pub fn ccsmen(&mut self) -> CCSMEN_W<'_, CMDCTRL_SPEC> {
        CCSMEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - SD I/O suspend command"]
    #[inline(always)]
    pub fn iosusp(&mut self) -> IOSUSP_W<'_, CMDCTRL_SPEC> {
        IOSUSP_W::new(self, 11)
    }
}
#[doc = "SDIO command control register (SDIO_CMDCTRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMDCTRL_SPEC;
impl crate::RegisterSpec for CMDCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdctrl::R`](R) reader structure"]
impl crate::Readable for CMDCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmdctrl::W`](W) writer structure"]
impl crate::Writable for CMDCTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMDCTRL to value 0"]
impl crate::Resettable for CMDCTRL_SPEC {}
