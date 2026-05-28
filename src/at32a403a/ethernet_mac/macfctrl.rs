#[doc = "Register `MACFCTRL` reader"]
pub type R = crate::R<MACFCTRL_SPEC>;
#[doc = "Register `MACFCTRL` writer"]
pub type W = crate::W<MACFCTRL_SPEC>;
#[doc = "Flow control busy/back pressure activate\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCB_BPAW_A {
    #[doc = "1: Initiate a pause frame. After the completion of the Pause frame, this bit resets to 0"]
    InitiatePause = 1,
}
impl From<FCB_BPAW_A> for bool {
    #[inline(always)]
    fn from(variant: FCB_BPAW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCB_BPA` reader - Flow control busy/back pressure activate"]
pub type FCB_BPA_R = crate::BitReader<FCB_BPAW_A>;
impl FCB_BPA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FCB_BPAW_A> {
        match self.bits {
            true => Some(FCB_BPAW_A::InitiatePause),
            _ => None,
        }
    }
    #[doc = "Initiate a pause frame. After the completion of the Pause frame, this bit resets to 0"]
    #[inline(always)]
    pub fn is_initiate_pause(&self) -> bool {
        *self == FCB_BPAW_A::InitiatePause
    }
}
#[doc = "Field `FCB_BPA` writer - Flow control busy/back pressure activate"]
pub type FCB_BPA_W<'a, REG> = crate::BitWriter1S<'a, REG, FCB_BPAW_A>;
impl<'a, REG> FCB_BPA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Initiate a pause frame. After the completion of the Pause frame, this bit resets to 0"]
    #[inline(always)]
    pub fn initiate_pause(self) -> &'a mut crate::W<REG> {
        self.variant(FCB_BPAW_A::InitiatePause)
    }
}
#[doc = "Enable transmit flow control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETF_A {
    #[doc = "0: Transmit flow control disabled"]
    Disabled = 0,
    #[doc = "1: Transmit flow control enabled"]
    Enabled = 1,
}
impl From<ETF_A> for bool {
    #[inline(always)]
    fn from(variant: ETF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETF` reader - Enable transmit flow control"]
pub type ETF_R = crate::BitReader<ETF_A>;
impl ETF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ETF_A {
        match self.bits {
            false => ETF_A::Disabled,
            true => ETF_A::Enabled,
        }
    }
    #[doc = "Transmit flow control disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ETF_A::Disabled
    }
    #[doc = "Transmit flow control enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ETF_A::Enabled
    }
}
#[doc = "Field `ETF` writer - Enable transmit flow control"]
pub type ETF_W<'a, REG> = crate::BitWriter<'a, REG, ETF_A>;
impl<'a, REG> ETF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit flow control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::Disabled)
    }
    #[doc = "Transmit flow control enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::Enabled)
    }
}
#[doc = "Enable receive flow control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERF_A {
    #[doc = "0: Receive flow control disabled"]
    Disabled = 0,
    #[doc = "1: Receive flow control enabled"]
    Enabled = 1,
}
impl From<ERF_A> for bool {
    #[inline(always)]
    fn from(variant: ERF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERF` reader - Enable receive flow control"]
pub type ERF_R = crate::BitReader<ERF_A>;
impl ERF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERF_A {
        match self.bits {
            false => ERF_A::Disabled,
            true => ERF_A::Enabled,
        }
    }
    #[doc = "Receive flow control disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERF_A::Disabled
    }
    #[doc = "Receive flow control enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERF_A::Enabled
    }
}
#[doc = "Field `ERF` writer - Enable receive flow control"]
pub type ERF_W<'a, REG> = crate::BitWriter<'a, REG, ERF_A>;
impl<'a, REG> ERF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive flow control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERF_A::Disabled)
    }
    #[doc = "Receive flow control enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERF_A::Enabled)
    }
}
#[doc = "Detect unicast pause frame\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DUP_A {
    #[doc = "0: The MAC detects only a Pause frame with a unique multicast address"]
    OnlyMulticast = 0,
    #[doc = "1: The MAC detects the Pause frames with a unicast address specified in the MAC address0 high and MAC address0 low registers"]
    AnyPause = 1,
}
impl From<DUP_A> for bool {
    #[inline(always)]
    fn from(variant: DUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DUP` reader - Detect unicast pause frame"]
pub type DUP_R = crate::BitReader<DUP_A>;
impl DUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DUP_A {
        match self.bits {
            false => DUP_A::OnlyMulticast,
            true => DUP_A::AnyPause,
        }
    }
    #[doc = "The MAC detects only a Pause frame with a unique multicast address"]
    #[inline(always)]
    pub fn is_only_multicast(&self) -> bool {
        *self == DUP_A::OnlyMulticast
    }
    #[doc = "The MAC detects the Pause frames with a unicast address specified in the MAC address0 high and MAC address0 low registers"]
    #[inline(always)]
    pub fn is_any_pause(&self) -> bool {
        *self == DUP_A::AnyPause
    }
}
#[doc = "Field `DUP` writer - Detect unicast pause frame"]
pub type DUP_W<'a, REG> = crate::BitWriter<'a, REG, DUP_A>;
impl<'a, REG> DUP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The MAC detects only a Pause frame with a unique multicast address"]
    #[inline(always)]
    pub fn only_multicast(self) -> &'a mut crate::W<REG> {
        self.variant(DUP_A::OnlyMulticast)
    }
    #[doc = "The MAC detects the Pause frames with a unicast address specified in the MAC address0 high and MAC address0 low registers"]
    #[inline(always)]
    pub fn any_pause(self) -> &'a mut crate::W<REG> {
        self.variant(DUP_A::AnyPause)
    }
}
#[doc = "Pause low threshold\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLT_A {
    #[doc = "0: Pause time minus 4 slot times"]
    Minus4slot = 0,
    #[doc = "1: Pause time minus 28 slot times"]
    Minus28slot = 1,
    #[doc = "2: Pause time minus 144 slot times"]
    Minus144slot = 2,
    #[doc = "3: Pause time minus 256 slot times"]
    Minus256slot = 3,
}
impl From<PLT_A> for u8 {
    #[inline(always)]
    fn from(variant: PLT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLT_A {
    type Ux = u8;
}
impl crate::IsEnum for PLT_A {}
#[doc = "Field `PLT` reader - Pause low threshold"]
pub type PLT_R = crate::FieldReader<PLT_A>;
impl PLT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLT_A {
        match self.bits {
            0 => PLT_A::Minus4slot,
            1 => PLT_A::Minus28slot,
            2 => PLT_A::Minus144slot,
            3 => PLT_A::Minus256slot,
            _ => unreachable!(),
        }
    }
    #[doc = "Pause time minus 4 slot times"]
    #[inline(always)]
    pub fn is_minus4slot(&self) -> bool {
        *self == PLT_A::Minus4slot
    }
    #[doc = "Pause time minus 28 slot times"]
    #[inline(always)]
    pub fn is_minus28slot(&self) -> bool {
        *self == PLT_A::Minus28slot
    }
    #[doc = "Pause time minus 144 slot times"]
    #[inline(always)]
    pub fn is_minus144slot(&self) -> bool {
        *self == PLT_A::Minus144slot
    }
    #[doc = "Pause time minus 256 slot times"]
    #[inline(always)]
    pub fn is_minus256slot(&self) -> bool {
        *self == PLT_A::Minus256slot
    }
}
#[doc = "Field `PLT` writer - Pause low threshold"]
pub type PLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLT_A, crate::Safe>;
impl<'a, REG> PLT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pause time minus 4 slot times"]
    #[inline(always)]
    pub fn minus4slot(self) -> &'a mut crate::W<REG> {
        self.variant(PLT_A::Minus4slot)
    }
    #[doc = "Pause time minus 28 slot times"]
    #[inline(always)]
    pub fn minus28slot(self) -> &'a mut crate::W<REG> {
        self.variant(PLT_A::Minus28slot)
    }
    #[doc = "Pause time minus 144 slot times"]
    #[inline(always)]
    pub fn minus144slot(self) -> &'a mut crate::W<REG> {
        self.variant(PLT_A::Minus144slot)
    }
    #[doc = "Pause time minus 256 slot times"]
    #[inline(always)]
    pub fn minus256slot(self) -> &'a mut crate::W<REG> {
        self.variant(PLT_A::Minus256slot)
    }
}
#[doc = "Disable zero-quanta pause\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DZQP_A {
    #[doc = "0: Normal operation"]
    Normal = 0,
    #[doc = "1: Disables the automatic generation of Zero-quanta Pause frame while the flow control signal of the FIFO layer is disabled"]
    DisableZeroQuanta = 1,
}
impl From<DZQP_A> for bool {
    #[inline(always)]
    fn from(variant: DZQP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DZQP` reader - Disable zero-quanta pause"]
pub type DZQP_R = crate::BitReader<DZQP_A>;
impl DZQP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DZQP_A {
        match self.bits {
            false => DZQP_A::Normal,
            true => DZQP_A::DisableZeroQuanta,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == DZQP_A::Normal
    }
    #[doc = "Disables the automatic generation of Zero-quanta Pause frame while the flow control signal of the FIFO layer is disabled"]
    #[inline(always)]
    pub fn is_disable_zero_quanta(&self) -> bool {
        *self == DZQP_A::DisableZeroQuanta
    }
}
#[doc = "Field `DZQP` writer - Disable zero-quanta pause"]
pub type DZQP_W<'a, REG> = crate::BitWriter<'a, REG, DZQP_A>;
impl<'a, REG> DZQP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(DZQP_A::Normal)
    }
    #[doc = "Disables the automatic generation of Zero-quanta Pause frame while the flow control signal of the FIFO layer is disabled"]
    #[inline(always)]
    pub fn disable_zero_quanta(self) -> &'a mut crate::W<REG> {
        self.variant(DZQP_A::DisableZeroQuanta)
    }
}
#[doc = "Field `PT` reader - Pass time"]
pub type PT_R = crate::FieldReader<u16>;
#[doc = "Field `PT` writer - Pass time"]
pub type PT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Flow control busy/back pressure activate"]
    #[inline(always)]
    pub fn fcb_bpa(&self) -> FCB_BPA_R {
        FCB_BPA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable transmit flow control"]
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable receive flow control"]
    #[inline(always)]
    pub fn erf(&self) -> ERF_R {
        ERF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Detect unicast pause frame"]
    #[inline(always)]
    pub fn dup(&self) -> DUP_R {
        DUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Pause low threshold"]
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Disable zero-quanta pause"]
    #[inline(always)]
    pub fn dzqp(&self) -> DZQP_R {
        DZQP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Pass time"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACFCTRL")
            .field("fcb_bpa", &self.fcb_bpa())
            .field("etf", &self.etf())
            .field("erf", &self.erf())
            .field("dup", &self.dup())
            .field("plt", &self.plt())
            .field("dzqp", &self.dzqp())
            .field("pt", &self.pt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Flow control busy/back pressure activate"]
    #[inline(always)]
    pub fn fcb_bpa(&mut self) -> FCB_BPA_W<'_, MACFCTRL_SPEC> {
        FCB_BPA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable transmit flow control"]
    #[inline(always)]
    pub fn etf(&mut self) -> ETF_W<'_, MACFCTRL_SPEC> {
        ETF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable receive flow control"]
    #[inline(always)]
    pub fn erf(&mut self) -> ERF_W<'_, MACFCTRL_SPEC> {
        ERF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Detect unicast pause frame"]
    #[inline(always)]
    pub fn dup(&mut self) -> DUP_W<'_, MACFCTRL_SPEC> {
        DUP_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Pause low threshold"]
    #[inline(always)]
    pub fn plt(&mut self) -> PLT_W<'_, MACFCTRL_SPEC> {
        PLT_W::new(self, 4)
    }
    #[doc = "Bit 7 - Disable zero-quanta pause"]
    #[inline(always)]
    pub fn dzqp(&mut self) -> DZQP_W<'_, MACFCTRL_SPEC> {
        DZQP_W::new(self, 7)
    }
    #[doc = "Bits 16:31 - Pass time"]
    #[inline(always)]
    pub fn pt(&mut self) -> PT_W<'_, MACFCTRL_SPEC> {
        PT_W::new(self, 16)
    }
}
#[doc = "Ethernet MAC flow control register\n\nYou can [`read`](crate::Reg::read) this register and get [`macfctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macfctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACFCTRL_SPEC;
impl crate::RegisterSpec for MACFCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macfctrl::R`](R) reader structure"]
impl crate::Readable for MACFCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macfctrl::W`](W) writer structure"]
impl crate::Writable for MACFCTRL_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets MACFCTRL to value 0"]
impl crate::Resettable for MACFCTRL_SPEC {}
