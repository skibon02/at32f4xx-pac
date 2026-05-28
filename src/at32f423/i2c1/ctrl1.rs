#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<CTRL1_SPEC>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<CTRL1_SPEC>;
#[doc = "I2C peripheral enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2cenr {
    #[doc = "0: I2C peripheral is disabled"]
    Disabled = 0,
    #[doc = "1: I2C peripheral is enabled"]
    Enabled = 1,
}
impl From<I2cenr> for bool {
    #[inline(always)]
    fn from(variant: I2cenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2CEN` reader - I2C peripheral enable"]
pub type I2CEN_R = crate::BitReader<I2cenr>;
impl I2CEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2cenr {
        match self.bits {
            false => I2cenr::Disabled,
            true => I2cenr::Enabled,
        }
    }
    #[doc = "I2C peripheral is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2cenr::Disabled
    }
    #[doc = "I2C peripheral is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2cenr::Enabled
    }
}
#[doc = "I2C peripheral enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2cenwWO {
    #[doc = "0: Disable I2C peripheral"]
    Disable = 0,
    #[doc = "1: Enable I2C peripheral"]
    Enable = 1,
}
impl From<I2cenwWO> for bool {
    #[inline(always)]
    fn from(variant: I2cenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2CEN` writer - I2C peripheral enable"]
pub type I2CEN_W<'a, REG> = crate::BitWriter<'a, REG, I2cenwWO>;
impl<'a, REG> I2CEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable I2C peripheral"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(I2cenwWO::Disable)
    }
    #[doc = "Enable I2C peripheral"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(I2cenwWO::Enable)
    }
}
#[doc = "Transmit data interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdienr {
    #[doc = "0: Data transmit interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Data transmit interrupt is enabled"]
    Enabled = 1,
}
impl From<Tdienr> for bool {
    #[inline(always)]
    fn from(variant: Tdienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDIEN` reader - Transmit data interrupt enable"]
pub type TDIEN_R = crate::BitReader<Tdienr>;
impl TDIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tdienr {
        match self.bits {
            false => Tdienr::Disabled,
            true => Tdienr::Enabled,
        }
    }
    #[doc = "Data transmit interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tdienr::Disabled
    }
    #[doc = "Data transmit interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tdienr::Enabled
    }
}
#[doc = "Transmit data interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TdienwWO {
    #[doc = "0: Disable Data transmit interrupt"]
    Disable = 0,
    #[doc = "1: Enable Data transmit interrupt"]
    Enable = 1,
}
impl From<TdienwWO> for bool {
    #[inline(always)]
    fn from(variant: TdienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDIEN` writer - Transmit data interrupt enable"]
pub type TDIEN_W<'a, REG> = crate::BitWriter<'a, REG, TdienwWO>;
impl<'a, REG> TDIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Data transmit interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TdienwWO::Disable)
    }
    #[doc = "Enable Data transmit interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TdienwWO::Enable)
    }
}
#[doc = "Receive data interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdienr {
    #[doc = "0: Data receive interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Data receive interrupt is enabled"]
    Enabled = 1,
}
impl From<Rdienr> for bool {
    #[inline(always)]
    fn from(variant: Rdienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDIEN` reader - Receive data interrupt enable"]
pub type RDIEN_R = crate::BitReader<Rdienr>;
impl RDIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdienr {
        match self.bits {
            false => Rdienr::Disabled,
            true => Rdienr::Enabled,
        }
    }
    #[doc = "Data receive interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rdienr::Disabled
    }
    #[doc = "Data receive interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rdienr::Enabled
    }
}
#[doc = "Receive data interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RdienwWO {
    #[doc = "0: Disable Data receive interrupt"]
    Disable = 0,
    #[doc = "1: Enable Data receive interrupt"]
    Enable = 1,
}
impl From<RdienwWO> for bool {
    #[inline(always)]
    fn from(variant: RdienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDIEN` writer - Receive data interrupt enable"]
pub type RDIEN_W<'a, REG> = crate::BitWriter<'a, REG, RdienwWO>;
impl<'a, REG> RDIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Data receive interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RdienwWO::Disable)
    }
    #[doc = "Enable Data receive interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RdienwWO::Enable)
    }
}
#[doc = "Address match interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addrienr {
    #[doc = "0: Address match interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Address match interrupt is enabled"]
    Enabled = 1,
}
impl From<Addrienr> for bool {
    #[inline(always)]
    fn from(variant: Addrienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRIEN` reader - Address match interrupt enable"]
pub type ADDRIEN_R = crate::BitReader<Addrienr>;
impl ADDRIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addrienr {
        match self.bits {
            false => Addrienr::Disabled,
            true => Addrienr::Enabled,
        }
    }
    #[doc = "Address match interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Addrienr::Disabled
    }
    #[doc = "Address match interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Addrienr::Enabled
    }
}
#[doc = "Address match interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddrienwWO {
    #[doc = "0: Disable Address match interrupt"]
    Disable = 0,
    #[doc = "1: Enable Address match interrupt"]
    Enable = 1,
}
impl From<AddrienwWO> for bool {
    #[inline(always)]
    fn from(variant: AddrienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRIEN` writer - Address match interrupt enable"]
pub type ADDRIEN_W<'a, REG> = crate::BitWriter<'a, REG, AddrienwWO>;
impl<'a, REG> ADDRIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Address match interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(AddrienwWO::Disable)
    }
    #[doc = "Enable Address match interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(AddrienwWO::Enable)
    }
}
#[doc = "Acknowledge fail interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ackfailienr {
    #[doc = "0: Acknowledge failure interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Acknowledge failure interrupt is enabled"]
    Enabled = 1,
}
impl From<Ackfailienr> for bool {
    #[inline(always)]
    fn from(variant: Ackfailienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKFAILIEN` reader - Acknowledge fail interrupt enable"]
pub type ACKFAILIEN_R = crate::BitReader<Ackfailienr>;
impl ACKFAILIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ackfailienr {
        match self.bits {
            false => Ackfailienr::Disabled,
            true => Ackfailienr::Enabled,
        }
    }
    #[doc = "Acknowledge failure interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ackfailienr::Disabled
    }
    #[doc = "Acknowledge failure interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ackfailienr::Enabled
    }
}
#[doc = "Acknowledge fail interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AckfailienwWO {
    #[doc = "0: Disable Acknowledge failure interrupt"]
    Disable = 0,
    #[doc = "1: Enable Acknowledge failure interrupt"]
    Enable = 1,
}
impl From<AckfailienwWO> for bool {
    #[inline(always)]
    fn from(variant: AckfailienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKFAILIEN` writer - Acknowledge fail interrupt enable"]
pub type ACKFAILIEN_W<'a, REG> = crate::BitWriter<'a, REG, AckfailienwWO>;
impl<'a, REG> ACKFAILIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Acknowledge failure interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(AckfailienwWO::Disable)
    }
    #[doc = "Enable Acknowledge failure interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(AckfailienwWO::Enable)
    }
}
#[doc = "Stop generation complete interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopienr {
    #[doc = "0: Stop generation complete interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Stop generation complete interrupt is enabled"]
    Enabled = 1,
}
impl From<Stopienr> for bool {
    #[inline(always)]
    fn from(variant: Stopienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPIEN` reader - Stop generation complete interrupt enable"]
pub type STOPIEN_R = crate::BitReader<Stopienr>;
impl STOPIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopienr {
        match self.bits {
            false => Stopienr::Disabled,
            true => Stopienr::Enabled,
        }
    }
    #[doc = "Stop generation complete interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Stopienr::Disabled
    }
    #[doc = "Stop generation complete interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Stopienr::Enabled
    }
}
#[doc = "Stop generation complete interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StopienwWO {
    #[doc = "0: Disable Stop generation complete interrupt"]
    Disable = 0,
    #[doc = "1: Enable Stop generation complete interrupt"]
    Enable = 1,
}
impl From<StopienwWO> for bool {
    #[inline(always)]
    fn from(variant: StopienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPIEN` writer - Stop generation complete interrupt enable"]
pub type STOPIEN_W<'a, REG> = crate::BitWriter<'a, REG, StopienwWO>;
impl<'a, REG> STOPIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Stop generation complete interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(StopienwWO::Disable)
    }
    #[doc = "Enable Stop generation complete interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(StopienwWO::Enable)
    }
}
#[doc = "Transfer data complete interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdcienr {
    #[doc = "0: Data transfer complete interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Data transfer complete interrupt is enabled"]
    Enabled = 1,
}
impl From<Tdcienr> for bool {
    #[inline(always)]
    fn from(variant: Tdcienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDCIEN` reader - Transfer data complete interrupt enable"]
pub type TDCIEN_R = crate::BitReader<Tdcienr>;
impl TDCIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tdcienr {
        match self.bits {
            false => Tdcienr::Disabled,
            true => Tdcienr::Enabled,
        }
    }
    #[doc = "Data transfer complete interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tdcienr::Disabled
    }
    #[doc = "Data transfer complete interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tdcienr::Enabled
    }
}
#[doc = "Transfer data complete interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TdcienwWO {
    #[doc = "0: Disable Data transfer complete interrupt"]
    Disable = 0,
    #[doc = "1: Enable Data transfer complete interrupt"]
    Enable = 1,
}
impl From<TdcienwWO> for bool {
    #[inline(always)]
    fn from(variant: TdcienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDCIEN` writer - Transfer data complete interrupt enable"]
pub type TDCIEN_W<'a, REG> = crate::BitWriter<'a, REG, TdcienwWO>;
impl<'a, REG> TDCIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Data transfer complete interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TdcienwWO::Disable)
    }
    #[doc = "Enable Data transfer complete interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TdcienwWO::Enable)
    }
}
#[doc = "Error interrupts enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errienr {
    #[doc = "0: Error interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Error interrupt is enabled"]
    Enabled = 1,
}
impl From<Errienr> for bool {
    #[inline(always)]
    fn from(variant: Errienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIEN` reader - Error interrupts enable"]
pub type ERRIEN_R = crate::BitReader<Errienr>;
impl ERRIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errienr {
        match self.bits {
            false => Errienr::Disabled,
            true => Errienr::Enabled,
        }
    }
    #[doc = "Error interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Errienr::Disabled
    }
    #[doc = "Error interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Errienr::Enabled
    }
}
#[doc = "Error interrupts enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrienwWO {
    #[doc = "0: Disable error interrupt"]
    Disable = 0,
    #[doc = "1: Enable error interrupt"]
    Enable = 1,
}
impl From<ErrienwWO> for bool {
    #[inline(always)]
    fn from(variant: ErrienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIEN` writer - Error interrupts enable"]
pub type ERRIEN_W<'a, REG> = crate::BitWriter<'a, REG, ErrienwWO>;
impl<'a, REG> ERRIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable error interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ErrienwWO::Disable)
    }
    #[doc = "Enable error interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ErrienwWO::Enable)
    }
}
#[doc = "Field `DFLT` reader - Digital filter value"]
pub type DFLT_R = crate::FieldReader;
#[doc = "Field `DFLT` writer - Digital filter value"]
pub type DFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "DMA Transmit data request enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMATEN_A {
    #[doc = "0: DMA transmit data request disabled"]
    Disabled = 0,
    #[doc = "1: DMA transmit data request enabled"]
    Enabled = 1,
}
impl From<DMATEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMATEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMATEN` reader - DMA Transmit data request enable"]
pub type DMATEN_R = crate::BitReader<DMATEN_A>;
impl DMATEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMATEN_A {
        match self.bits {
            false => DMATEN_A::Disabled,
            true => DMATEN_A::Enabled,
        }
    }
    #[doc = "DMA transmit data request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMATEN_A::Disabled
    }
    #[doc = "DMA transmit data request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMATEN_A::Enabled
    }
}
#[doc = "Field `DMATEN` writer - DMA Transmit data request enable"]
pub type DMATEN_W<'a, REG> = crate::BitWriter<'a, REG, DMATEN_A>;
impl<'a, REG> DMATEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA transmit data request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMATEN_A::Disabled)
    }
    #[doc = "DMA transmit data request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMATEN_A::Enabled)
    }
}
#[doc = "DMA receive data request enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAREN_A {
    #[doc = "0: DMA receive data request disabled"]
    Disabled = 0,
    #[doc = "1: DMA receive data request enabled"]
    Enabled = 1,
}
impl From<DMAREN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAREN` reader - DMA receive data request enable"]
pub type DMAREN_R = crate::BitReader<DMAREN_A>;
impl DMAREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAREN_A {
        match self.bits {
            false => DMAREN_A::Disabled,
            true => DMAREN_A::Enabled,
        }
    }
    #[doc = "DMA receive data request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAREN_A::Disabled
    }
    #[doc = "DMA receive data request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAREN_A::Enabled
    }
}
#[doc = "Field `DMAREN` writer - DMA receive data request enable"]
pub type DMAREN_W<'a, REG> = crate::BitWriter<'a, REG, DMAREN_A>;
impl<'a, REG> DMAREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA receive data request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREN_A::Disabled)
    }
    #[doc = "DMA receive data request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREN_A::Enabled)
    }
}
#[doc = "Slave receiving data control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCTRL_A {
    #[doc = "0: Slave receive data control disabled"]
    Disabled = 0,
    #[doc = "1: Slave receive data control enabled"]
    Enabled = 1,
}
impl From<SCTRL_A> for bool {
    #[inline(always)]
    fn from(variant: SCTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCTRL` reader - Slave receiving data control"]
pub type SCTRL_R = crate::BitReader<SCTRL_A>;
impl SCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SCTRL_A {
        match self.bits {
            false => SCTRL_A::Disabled,
            true => SCTRL_A::Enabled,
        }
    }
    #[doc = "Slave receive data control disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCTRL_A::Disabled
    }
    #[doc = "Slave receive data control enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCTRL_A::Enabled
    }
}
#[doc = "Field `SCTRL` writer - Slave receiving data control"]
pub type SCTRL_W<'a, REG> = crate::BitWriter<'a, REG, SCTRL_A>;
impl<'a, REG> SCTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave receive data control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCTRL_A::Disabled)
    }
    #[doc = "Slave receive data control enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCTRL_A::Enabled)
    }
}
#[doc = "Clock stretching mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stretchr {
    #[doc = "0: Clock stretching mode is disabled"]
    Disabled = 0,
    #[doc = "1: Clock stretching mode is enabled"]
    Enabled = 1,
}
impl From<Stretchr> for bool {
    #[inline(always)]
    fn from(variant: Stretchr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRETCH` reader - Clock stretching mode"]
pub type STRETCH_R = crate::BitReader<Stretchr>;
impl STRETCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stretchr {
        match self.bits {
            false => Stretchr::Disabled,
            true => Stretchr::Enabled,
        }
    }
    #[doc = "Clock stretching mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Stretchr::Disabled
    }
    #[doc = "Clock stretching mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Stretchr::Enabled
    }
}
#[doc = "Clock stretching mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StretchwWO {
    #[doc = "0: Disable clock stretching mode"]
    Disable = 0,
    #[doc = "1: Enable clock stretching mode"]
    Enable = 1,
}
impl From<StretchwWO> for bool {
    #[inline(always)]
    fn from(variant: StretchwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRETCH` writer - Clock stretching mode"]
pub type STRETCH_W<'a, REG> = crate::BitWriter<'a, REG, StretchwWO>;
impl<'a, REG> STRETCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable clock stretching mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(StretchwWO::Disable)
    }
    #[doc = "Enable clock stretching mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(StretchwWO::Enable)
    }
}
#[doc = "General call address enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gcaenr {
    #[doc = "0: General call address is disabled"]
    Disabled = 0,
    #[doc = "1: General call address is enabled, response 0000000x"]
    Enabled = 1,
}
impl From<Gcaenr> for bool {
    #[inline(always)]
    fn from(variant: Gcaenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GCAEN` reader - General call address enable"]
pub type GCAEN_R = crate::BitReader<Gcaenr>;
impl GCAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gcaenr {
        match self.bits {
            false => Gcaenr::Disabled,
            true => Gcaenr::Enabled,
        }
    }
    #[doc = "General call address is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gcaenr::Disabled
    }
    #[doc = "General call address is enabled, response 0000000x"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gcaenr::Enabled
    }
}
#[doc = "General call address enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GcaenwWO {
    #[doc = "0: Disable general call address"]
    Disable = 0,
    #[doc = "1: Enable general call address, response 0000000x"]
    Enable = 1,
}
impl From<GcaenwWO> for bool {
    #[inline(always)]
    fn from(variant: GcaenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GCAEN` writer - General call address enable"]
pub type GCAEN_W<'a, REG> = crate::BitWriter<'a, REG, GcaenwWO>;
impl<'a, REG> GCAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable general call address"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(GcaenwWO::Disable)
    }
    #[doc = "Enable general call address, response 0000000x"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(GcaenwWO::Enable)
    }
}
#[doc = "SMBus host address enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HADDREN_A {
    #[doc = "0: SMBus host address disabled"]
    Disabled = 0,
    #[doc = "1: SMBus host address enabled, response with host address 0001000x"]
    Enabled = 1,
}
impl From<HADDREN_A> for bool {
    #[inline(always)]
    fn from(variant: HADDREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HADDREN` reader - SMBus host address enable"]
pub type HADDREN_R = crate::BitReader<HADDREN_A>;
impl HADDREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HADDREN_A {
        match self.bits {
            false => HADDREN_A::Disabled,
            true => HADDREN_A::Enabled,
        }
    }
    #[doc = "SMBus host address disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HADDREN_A::Disabled
    }
    #[doc = "SMBus host address enabled, response with host address 0001000x"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HADDREN_A::Enabled
    }
}
#[doc = "Field `HADDREN` writer - SMBus host address enable"]
pub type HADDREN_W<'a, REG> = crate::BitWriter<'a, REG, HADDREN_A>;
impl<'a, REG> HADDREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SMBus host address disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HADDREN_A::Disabled)
    }
    #[doc = "SMBus host address enabled, response with host address 0001000x"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HADDREN_A::Enabled)
    }
}
#[doc = "SMBus device default address enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEVADDREN_A {
    #[doc = "0: SMBus device default address disabled"]
    Disabled = 0,
    #[doc = "1: SMBus device default address enabled, response with default address 1100001x"]
    Enabled = 1,
}
impl From<DEVADDREN_A> for bool {
    #[inline(always)]
    fn from(variant: DEVADDREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEVADDREN` reader - SMBus device default address enable"]
pub type DEVADDREN_R = crate::BitReader<DEVADDREN_A>;
impl DEVADDREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DEVADDREN_A {
        match self.bits {
            false => DEVADDREN_A::Disabled,
            true => DEVADDREN_A::Enabled,
        }
    }
    #[doc = "SMBus device default address disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DEVADDREN_A::Disabled
    }
    #[doc = "SMBus device default address enabled, response with default address 1100001x"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DEVADDREN_A::Enabled
    }
}
#[doc = "Field `DEVADDREN` writer - SMBus device default address enable"]
pub type DEVADDREN_W<'a, REG> = crate::BitWriter<'a, REG, DEVADDREN_A>;
impl<'a, REG> DEVADDREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SMBus device default address disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DEVADDREN_A::Disabled)
    }
    #[doc = "SMBus device default address enabled, response with default address 1100001x"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DEVADDREN_A::Enabled)
    }
}
#[doc = "SMBus alert enable / pin set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMBALERT_A {
    #[doc = "0: Slave SMBus alert pin high / Master SMBus alert disabled"]
    HighDisabled = 0,
    #[doc = "1: Slave SMBus alert pin low, response address 0001100x / Master SMBus alert enabled"]
    LowEnabled = 1,
}
impl From<SMBALERT_A> for bool {
    #[inline(always)]
    fn from(variant: SMBALERT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMBALERT` reader - SMBus alert enable / pin set"]
pub type SMBALERT_R = crate::BitReader<SMBALERT_A>;
impl SMBALERT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMBALERT_A {
        match self.bits {
            false => SMBALERT_A::HighDisabled,
            true => SMBALERT_A::LowEnabled,
        }
    }
    #[doc = "Slave SMBus alert pin high / Master SMBus alert disabled"]
    #[inline(always)]
    pub fn is_high_disabled(&self) -> bool {
        *self == SMBALERT_A::HighDisabled
    }
    #[doc = "Slave SMBus alert pin low, response address 0001100x / Master SMBus alert enabled"]
    #[inline(always)]
    pub fn is_low_enabled(&self) -> bool {
        *self == SMBALERT_A::LowEnabled
    }
}
#[doc = "Field `SMBALERT` writer - SMBus alert enable / pin set"]
pub type SMBALERT_W<'a, REG> = crate::BitWriter<'a, REG, SMBALERT_A>;
impl<'a, REG> SMBALERT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave SMBus alert pin high / Master SMBus alert disabled"]
    #[inline(always)]
    pub fn high_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SMBALERT_A::HighDisabled)
    }
    #[doc = "Slave SMBus alert pin low, response address 0001100x / Master SMBus alert enabled"]
    #[inline(always)]
    pub fn low_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SMBALERT_A::LowEnabled)
    }
}
#[doc = "PEC calculation enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pecenr {
    #[doc = "0: PEC calculation is disabled"]
    Disabled = 0,
    #[doc = "1: PEC calculation is enabled"]
    Enabled = 1,
}
impl From<Pecenr> for bool {
    #[inline(always)]
    fn from(variant: Pecenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECEN` reader - PEC calculation enable"]
pub type PECEN_R = crate::BitReader<Pecenr>;
impl PECEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pecenr {
        match self.bits {
            false => Pecenr::Disabled,
            true => Pecenr::Enabled,
        }
    }
    #[doc = "PEC calculation is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pecenr::Disabled
    }
    #[doc = "PEC calculation is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pecenr::Enabled
    }
}
#[doc = "PEC calculation enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PecenwWO {
    #[doc = "0: Disable PEC calculation"]
    Disable = 0,
    #[doc = "1: Enable PEC calculation"]
    Enable = 1,
}
impl From<PecenwWO> for bool {
    #[inline(always)]
    fn from(variant: PecenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECEN` writer - PEC calculation enable"]
pub type PECEN_W<'a, REG> = crate::BitWriter<'a, REG, PecenwWO>;
impl<'a, REG> PECEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable PEC calculation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PecenwWO::Disable)
    }
    #[doc = "Enable PEC calculation"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PecenwWO::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - I2C peripheral enable"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2CEN_R {
        I2CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit data interrupt enable"]
    #[inline(always)]
    pub fn tdien(&self) -> TDIEN_R {
        TDIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive data interrupt enable"]
    #[inline(always)]
    pub fn rdien(&self) -> RDIEN_R {
        RDIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Address match interrupt enable"]
    #[inline(always)]
    pub fn addrien(&self) -> ADDRIEN_R {
        ADDRIEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Acknowledge fail interrupt enable"]
    #[inline(always)]
    pub fn ackfailien(&self) -> ACKFAILIEN_R {
        ACKFAILIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop generation complete interrupt enable"]
    #[inline(always)]
    pub fn stopien(&self) -> STOPIEN_R {
        STOPIEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer data complete interrupt enable"]
    #[inline(always)]
    pub fn tdcien(&self) -> TDCIEN_R {
        TDCIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error interrupts enable"]
    #[inline(always)]
    pub fn errien(&self) -> ERRIEN_R {
        ERRIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Digital filter value"]
    #[inline(always)]
    pub fn dflt(&self) -> DFLT_R {
        DFLT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - DMA Transmit data request enable"]
    #[inline(always)]
    pub fn dmaten(&self) -> DMATEN_R {
        DMATEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMA receive data request enable"]
    #[inline(always)]
    pub fn dmaren(&self) -> DMAREN_R {
        DMAREN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave receiving data control"]
    #[inline(always)]
    pub fn sctrl(&self) -> SCTRL_R {
        SCTRL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Clock stretching mode"]
    #[inline(always)]
    pub fn stretch(&self) -> STRETCH_R {
        STRETCH_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - General call address enable"]
    #[inline(always)]
    pub fn gcaen(&self) -> GCAEN_R {
        GCAEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SMBus host address enable"]
    #[inline(always)]
    pub fn haddren(&self) -> HADDREN_R {
        HADDREN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SMBus device default address enable"]
    #[inline(always)]
    pub fn devaddren(&self) -> DEVADDREN_R {
        DEVADDREN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SMBus alert enable / pin set"]
    #[inline(always)]
    pub fn smbalert(&self) -> SMBALERT_R {
        SMBALERT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PEC calculation enable"]
    #[inline(always)]
    pub fn pecen(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL1")
            .field("i2cen", &self.i2cen())
            .field("tdien", &self.tdien())
            .field("rdien", &self.rdien())
            .field("addrien", &self.addrien())
            .field("ackfailien", &self.ackfailien())
            .field("stopien", &self.stopien())
            .field("tdcien", &self.tdcien())
            .field("errien", &self.errien())
            .field("dflt", &self.dflt())
            .field("dmaten", &self.dmaten())
            .field("dmaren", &self.dmaren())
            .field("sctrl", &self.sctrl())
            .field("stretch", &self.stretch())
            .field("gcaen", &self.gcaen())
            .field("haddren", &self.haddren())
            .field("devaddren", &self.devaddren())
            .field("smbalert", &self.smbalert())
            .field("pecen", &self.pecen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - I2C peripheral enable"]
    #[inline(always)]
    pub fn i2cen(&mut self) -> I2CEN_W<'_, CTRL1_SPEC> {
        I2CEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit data interrupt enable"]
    #[inline(always)]
    pub fn tdien(&mut self) -> TDIEN_W<'_, CTRL1_SPEC> {
        TDIEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Receive data interrupt enable"]
    #[inline(always)]
    pub fn rdien(&mut self) -> RDIEN_W<'_, CTRL1_SPEC> {
        RDIEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Address match interrupt enable"]
    #[inline(always)]
    pub fn addrien(&mut self) -> ADDRIEN_W<'_, CTRL1_SPEC> {
        ADDRIEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Acknowledge fail interrupt enable"]
    #[inline(always)]
    pub fn ackfailien(&mut self) -> ACKFAILIEN_W<'_, CTRL1_SPEC> {
        ACKFAILIEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Stop generation complete interrupt enable"]
    #[inline(always)]
    pub fn stopien(&mut self) -> STOPIEN_W<'_, CTRL1_SPEC> {
        STOPIEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transfer data complete interrupt enable"]
    #[inline(always)]
    pub fn tdcien(&mut self) -> TDCIEN_W<'_, CTRL1_SPEC> {
        TDCIEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Error interrupts enable"]
    #[inline(always)]
    pub fn errien(&mut self) -> ERRIEN_W<'_, CTRL1_SPEC> {
        ERRIEN_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - Digital filter value"]
    #[inline(always)]
    pub fn dflt(&mut self) -> DFLT_W<'_, CTRL1_SPEC> {
        DFLT_W::new(self, 8)
    }
    #[doc = "Bit 14 - DMA Transmit data request enable"]
    #[inline(always)]
    pub fn dmaten(&mut self) -> DMATEN_W<'_, CTRL1_SPEC> {
        DMATEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - DMA receive data request enable"]
    #[inline(always)]
    pub fn dmaren(&mut self) -> DMAREN_W<'_, CTRL1_SPEC> {
        DMAREN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Slave receiving data control"]
    #[inline(always)]
    pub fn sctrl(&mut self) -> SCTRL_W<'_, CTRL1_SPEC> {
        SCTRL_W::new(self, 16)
    }
    #[doc = "Bit 17 - Clock stretching mode"]
    #[inline(always)]
    pub fn stretch(&mut self) -> STRETCH_W<'_, CTRL1_SPEC> {
        STRETCH_W::new(self, 17)
    }
    #[doc = "Bit 19 - General call address enable"]
    #[inline(always)]
    pub fn gcaen(&mut self) -> GCAEN_W<'_, CTRL1_SPEC> {
        GCAEN_W::new(self, 19)
    }
    #[doc = "Bit 20 - SMBus host address enable"]
    #[inline(always)]
    pub fn haddren(&mut self) -> HADDREN_W<'_, CTRL1_SPEC> {
        HADDREN_W::new(self, 20)
    }
    #[doc = "Bit 21 - SMBus device default address enable"]
    #[inline(always)]
    pub fn devaddren(&mut self) -> DEVADDREN_W<'_, CTRL1_SPEC> {
        DEVADDREN_W::new(self, 21)
    }
    #[doc = "Bit 22 - SMBus alert enable / pin set"]
    #[inline(always)]
    pub fn smbalert(&mut self) -> SMBALERT_W<'_, CTRL1_SPEC> {
        SMBALERT_W::new(self, 22)
    }
    #[doc = "Bit 23 - PEC calculation enable"]
    #[inline(always)]
    pub fn pecen(&mut self) -> PECEN_W<'_, CTRL1_SPEC> {
        PECEN_W::new(self, 23)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for CTRL1_SPEC {}
