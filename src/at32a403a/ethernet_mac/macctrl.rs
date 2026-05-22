#[doc = "Register `MACCTRL` reader"]
pub type R = crate::R<MACCTRL_SPEC>;
#[doc = "Register `MACCTRL` writer"]
pub type W = crate::W<MACCTRL_SPEC>;
#[doc = "Receiver enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RE_A {
    #[doc = "0: Receive disabled"]
    Disabled = 0,
    #[doc = "1: Receive enabled"]
    Enabled = 1,
}
impl From<RE_A> for bool {
    #[inline(always)]
    fn from(variant: RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RE` reader - Receiver enable"]
pub type RE_R = crate::BitReader<RE_A>;
impl RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RE_A {
        match self.bits {
            false => RE_A::Disabled,
            true => RE_A::Enabled,
        }
    }
    #[doc = "Receive disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RE_A::Disabled
    }
    #[doc = "Receive enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RE_A::Enabled
    }
}
#[doc = "Field `RE` writer - Receiver enable"]
pub type RE_W<'a, REG> = crate::BitWriter<'a, REG, RE_A>;
impl<'a, REG> RE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RE_A::Disabled)
    }
    #[doc = "Receive enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RE_A::Enabled)
    }
}
#[doc = "Transmitter enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TE_A {
    #[doc = "0: Transmit disabled"]
    Disabled = 0,
    #[doc = "1: Transmit enabled"]
    Enabled = 1,
}
impl From<TE_A> for bool {
    #[inline(always)]
    fn from(variant: TE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE` reader - Transmitter enable"]
pub type TE_R = crate::BitReader<TE_A>;
impl TE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TE_A {
        match self.bits {
            false => TE_A::Disabled,
            true => TE_A::Enabled,
        }
    }
    #[doc = "Transmit disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TE_A::Disabled
    }
    #[doc = "Transmit enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TE_A::Enabled
    }
}
#[doc = "Field `TE` writer - Transmitter enable"]
pub type TE_W<'a, REG> = crate::BitWriter<'a, REG, TE_A>;
impl<'a, REG> TE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TE_A::Disabled)
    }
    #[doc = "Transmit enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TE_A::Enabled)
    }
}
#[doc = "Field `DC` reader - Only in half-duplex mode! Deferral check"]
pub type DC_R = crate::BitReader;
#[doc = "Field `DC` writer - Only in half-duplex mode! Deferral check"]
pub type DC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BL` reader - Only in half-duplex mode! Back-off Limit"]
pub type BL_R = crate::FieldReader;
#[doc = "Field `BL` writer - Only in half-duplex mode! Back-off Limit"]
pub type BL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Automatic pad/CRC stripping\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACS_A {
    #[doc = "0: The MAC will forward all received frames to the master without changing its contents"]
    NoPadCrcStripping = 0,
    #[doc = "1: The MAC strips the pad/FCS field on received frames only when the frame length is shorter than 1536 bytes. All received frame with length field greater than or equal to 1536 bytes are passed on to the application without stripping the Pad or FCS field"]
    AutomaticPadCrcStripping = 1,
}
impl From<ACS_A> for bool {
    #[inline(always)]
    fn from(variant: ACS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACS` reader - Automatic pad/CRC stripping"]
pub type ACS_R = crate::BitReader<ACS_A>;
impl ACS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ACS_A {
        match self.bits {
            false => ACS_A::NoPadCrcStripping,
            true => ACS_A::AutomaticPadCrcStripping,
        }
    }
    #[doc = "The MAC will forward all received frames to the master without changing its contents"]
    #[inline(always)]
    pub fn is_no_pad_crc_stripping(&self) -> bool {
        *self == ACS_A::NoPadCrcStripping
    }
    #[doc = "The MAC strips the pad/FCS field on received frames only when the frame length is shorter than 1536 bytes. All received frame with length field greater than or equal to 1536 bytes are passed on to the application without stripping the Pad or FCS field"]
    #[inline(always)]
    pub fn is_automatic_pad_crc_stripping(&self) -> bool {
        *self == ACS_A::AutomaticPadCrcStripping
    }
}
#[doc = "Field `ACS` writer - Automatic pad/CRC stripping"]
pub type ACS_W<'a, REG> = crate::BitWriter<'a, REG, ACS_A>;
impl<'a, REG> ACS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The MAC will forward all received frames to the master without changing its contents"]
    #[inline(always)]
    pub fn no_pad_crc_stripping(self) -> &'a mut crate::W<REG> {
        self.variant(ACS_A::NoPadCrcStripping)
    }
    #[doc = "The MAC strips the pad/FCS field on received frames only when the frame length is shorter than 1536 bytes. All received frame with length field greater than or equal to 1536 bytes are passed on to the application without stripping the Pad or FCS field"]
    #[inline(always)]
    pub fn automatic_pad_crc_stripping(self) -> &'a mut crate::W<REG> {
        self.variant(ACS_A::AutomaticPadCrcStripping)
    }
}
#[doc = "Field `DR` reader - Only in half-duplex mode! Disable Retry"]
pub type DR_R = crate::BitReader;
#[doc = "Field `DR` writer - Only in half-duplex mode! Disable Retry"]
pub type DR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "IPv4 checksum offload\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPC_A {
    #[doc = "0: IPv4 checksum calculation disabled"]
    NoIpv4checksum = 0,
    #[doc = "1: IPv4 checksum calculation enabled"]
    Ipv4checksumEnabled = 1,
}
impl From<IPC_A> for bool {
    #[inline(always)]
    fn from(variant: IPC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IPC` reader - IPv4 checksum offload"]
pub type IPC_R = crate::BitReader<IPC_A>;
impl IPC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IPC_A {
        match self.bits {
            false => IPC_A::NoIpv4checksum,
            true => IPC_A::Ipv4checksumEnabled,
        }
    }
    #[doc = "IPv4 checksum calculation disabled"]
    #[inline(always)]
    pub fn is_no_ipv4checksum(&self) -> bool {
        *self == IPC_A::NoIpv4checksum
    }
    #[doc = "IPv4 checksum calculation enabled"]
    #[inline(always)]
    pub fn is_ipv4checksum_enabled(&self) -> bool {
        *self == IPC_A::Ipv4checksumEnabled
    }
}
#[doc = "Field `IPC` writer - IPv4 checksum offload"]
pub type IPC_W<'a, REG> = crate::BitWriter<'a, REG, IPC_A>;
impl<'a, REG> IPC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IPv4 checksum calculation disabled"]
    #[inline(always)]
    pub fn no_ipv4checksum(self) -> &'a mut crate::W<REG> {
        self.variant(IPC_A::NoIpv4checksum)
    }
    #[doc = "IPv4 checksum calculation enabled"]
    #[inline(always)]
    pub fn ipv4checksum_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IPC_A::Ipv4checksumEnabled)
    }
}
#[doc = "Duplex mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DM_A {
    #[doc = "0: Half-duplex mode"]
    HalfDuplex = 0,
    #[doc = "1: Full-duplex mode"]
    FullDuplex = 1,
}
impl From<DM_A> for bool {
    #[inline(always)]
    fn from(variant: DM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DM` reader - Duplex mode"]
pub type DM_R = crate::BitReader<DM_A>;
impl DM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DM_A {
        match self.bits {
            false => DM_A::HalfDuplex,
            true => DM_A::FullDuplex,
        }
    }
    #[doc = "Half-duplex mode"]
    #[inline(always)]
    pub fn is_half_duplex(&self) -> bool {
        *self == DM_A::HalfDuplex
    }
    #[doc = "Full-duplex mode"]
    #[inline(always)]
    pub fn is_full_duplex(&self) -> bool {
        *self == DM_A::FullDuplex
    }
}
#[doc = "Field `DM` writer - Duplex mode"]
pub type DM_W<'a, REG> = crate::BitWriter<'a, REG, DM_A>;
impl<'a, REG> DM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Half-duplex mode"]
    #[inline(always)]
    pub fn half_duplex(self) -> &'a mut crate::W<REG> {
        self.variant(DM_A::HalfDuplex)
    }
    #[doc = "Full-duplex mode"]
    #[inline(always)]
    pub fn full_duplex(self) -> &'a mut crate::W<REG> {
        self.variant(DM_A::FullDuplex)
    }
}
#[doc = "Loopback mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LM_A {
    #[doc = "0: Loopback mode disabled"]
    NoLoopback = 0,
    #[doc = "1: Loopback mode. The MII receive clock input (clk_rx_i) is required for the loopback mode to work normally, for the transmit clock is not looped-back internally"]
    Loopback = 1,
}
impl From<LM_A> for bool {
    #[inline(always)]
    fn from(variant: LM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LM` reader - Loopback mode"]
pub type LM_R = crate::BitReader<LM_A>;
impl LM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LM_A {
        match self.bits {
            false => LM_A::NoLoopback,
            true => LM_A::Loopback,
        }
    }
    #[doc = "Loopback mode disabled"]
    #[inline(always)]
    pub fn is_no_loopback(&self) -> bool {
        *self == LM_A::NoLoopback
    }
    #[doc = "Loopback mode. The MII receive clock input (clk_rx_i) is required for the loopback mode to work normally, for the transmit clock is not looped-back internally"]
    #[inline(always)]
    pub fn is_loopback(&self) -> bool {
        *self == LM_A::Loopback
    }
}
#[doc = "Field `LM` writer - Loopback mode"]
pub type LM_W<'a, REG> = crate::BitWriter<'a, REG, LM_A>;
impl<'a, REG> LM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Loopback mode disabled"]
    #[inline(always)]
    pub fn no_loopback(self) -> &'a mut crate::W<REG> {
        self.variant(LM_A::NoLoopback)
    }
    #[doc = "Loopback mode. The MII receive clock input (clk_rx_i) is required for the loopback mode to work normally, for the transmit clock is not looped-back internally"]
    #[inline(always)]
    pub fn loopback(self) -> &'a mut crate::W<REG> {
        self.variant(LM_A::Loopback)
    }
}
#[doc = "Field `DRO` reader - Only in half-duplex mode! Disable Receive Own."]
pub type DRO_R = crate::BitReader;
#[doc = "Field `DRO` writer - Only in half-duplex mode! Disable Receive Own."]
pub type DRO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Fast EMAC speed\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FES_A {
    #[doc = "0: The MAC operates at 10 Mbps"]
    Speed10mbps = 0,
    #[doc = "1: The MAC operates at 100 Mbps"]
    Speed100mbps = 1,
}
impl From<FES_A> for bool {
    #[inline(always)]
    fn from(variant: FES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FES` reader - Fast EMAC speed"]
pub type FES_R = crate::BitReader<FES_A>;
impl FES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FES_A {
        match self.bits {
            false => FES_A::Speed10mbps,
            true => FES_A::Speed100mbps,
        }
    }
    #[doc = "The MAC operates at 10 Mbps"]
    #[inline(always)]
    pub fn is_speed10mbps(&self) -> bool {
        *self == FES_A::Speed10mbps
    }
    #[doc = "The MAC operates at 100 Mbps"]
    #[inline(always)]
    pub fn is_speed100mbps(&self) -> bool {
        *self == FES_A::Speed100mbps
    }
}
#[doc = "Field `FES` writer - Fast EMAC speed"]
pub type FES_W<'a, REG> = crate::BitWriter<'a, REG, FES_A>;
impl<'a, REG> FES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The MAC operates at 10 Mbps"]
    #[inline(always)]
    pub fn speed10mbps(self) -> &'a mut crate::W<REG> {
        self.variant(FES_A::Speed10mbps)
    }
    #[doc = "The MAC operates at 100 Mbps"]
    #[inline(always)]
    pub fn speed100mbps(self) -> &'a mut crate::W<REG> {
        self.variant(FES_A::Speed100mbps)
    }
}
#[doc = "Field `DCS` reader - Only in half-duplex mode! Disable Carrier Sense"]
pub type DCS_R = crate::BitReader;
#[doc = "Field `DCS` writer - Only in half-duplex mode! Disable Carrier Sense"]
pub type DCS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interframe gap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IFG_A {
    #[doc = "0: Minimum interframe gap is 96 bits (12 bytes) during transmission"]
    Ifg96bit = 0,
    #[doc = "1: Minimum interframe gap is 88 bits (11 bytes) during transmission"]
    Ifg88bit = 1,
    #[doc = "2: Minimum interframe gap is 80 bits (10 bytes) during transmission"]
    Ifg80bit = 2,
    #[doc = "3: Minimum interframe gap is 72 bits (9 bytes) during transmission"]
    Ifg72bit = 3,
    #[doc = "4: Minimum interframe gap is 64 bits (8 bytes) during transmission"]
    Ifg64bit = 4,
    #[doc = "5: Minimum interframe gap is 56 bits (7 bytes) during transmission"]
    Ifg56bit = 5,
    #[doc = "6: Minimum interframe gap is 48 bits (6 bytes) during transmission"]
    Ifg48bit = 6,
    #[doc = "7: Minimum interframe gap is 40 bits (5 bytes) during transmission"]
    Ifg40bit = 7,
}
impl From<IFG_A> for u8 {
    #[inline(always)]
    fn from(variant: IFG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IFG_A {
    type Ux = u8;
}
impl crate::IsEnum for IFG_A {}
#[doc = "Field `IFG` reader - Interframe gap"]
pub type IFG_R = crate::FieldReader<IFG_A>;
impl IFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IFG_A {
        match self.bits {
            0 => IFG_A::Ifg96bit,
            1 => IFG_A::Ifg88bit,
            2 => IFG_A::Ifg80bit,
            3 => IFG_A::Ifg72bit,
            4 => IFG_A::Ifg64bit,
            5 => IFG_A::Ifg56bit,
            6 => IFG_A::Ifg48bit,
            7 => IFG_A::Ifg40bit,
            _ => unreachable!(),
        }
    }
    #[doc = "Minimum interframe gap is 96 bits (12 bytes) during transmission"]
    #[inline(always)]
    pub fn is_ifg96bit(&self) -> bool {
        *self == IFG_A::Ifg96bit
    }
    #[doc = "Minimum interframe gap is 88 bits (11 bytes) during transmission"]
    #[inline(always)]
    pub fn is_ifg88bit(&self) -> bool {
        *self == IFG_A::Ifg88bit
    }
    #[doc = "Minimum interframe gap is 80 bits (10 bytes) during transmission"]
    #[inline(always)]
    pub fn is_ifg80bit(&self) -> bool {
        *self == IFG_A::Ifg80bit
    }
    #[doc = "Minimum interframe gap is 72 bits (9 bytes) during transmission"]
    #[inline(always)]
    pub fn is_ifg72bit(&self) -> bool {
        *self == IFG_A::Ifg72bit
    }
    #[doc = "Minimum interframe gap is 64 bits (8 bytes) during transmission"]
    #[inline(always)]
    pub fn is_ifg64bit(&self) -> bool {
        *self == IFG_A::Ifg64bit
    }
    #[doc = "Minimum interframe gap is 56 bits (7 bytes) during transmission"]
    #[inline(always)]
    pub fn is_ifg56bit(&self) -> bool {
        *self == IFG_A::Ifg56bit
    }
    #[doc = "Minimum interframe gap is 48 bits (6 bytes) during transmission"]
    #[inline(always)]
    pub fn is_ifg48bit(&self) -> bool {
        *self == IFG_A::Ifg48bit
    }
    #[doc = "Minimum interframe gap is 40 bits (5 bytes) during transmission"]
    #[inline(always)]
    pub fn is_ifg40bit(&self) -> bool {
        *self == IFG_A::Ifg40bit
    }
}
#[doc = "Field `IFG` writer - Interframe gap"]
pub type IFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3, IFG_A, crate::Safe>;
impl<'a, REG> IFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Minimum interframe gap is 96 bits (12 bytes) during transmission"]
    #[inline(always)]
    pub fn ifg96bit(self) -> &'a mut crate::W<REG> {
        self.variant(IFG_A::Ifg96bit)
    }
    #[doc = "Minimum interframe gap is 88 bits (11 bytes) during transmission"]
    #[inline(always)]
    pub fn ifg88bit(self) -> &'a mut crate::W<REG> {
        self.variant(IFG_A::Ifg88bit)
    }
    #[doc = "Minimum interframe gap is 80 bits (10 bytes) during transmission"]
    #[inline(always)]
    pub fn ifg80bit(self) -> &'a mut crate::W<REG> {
        self.variant(IFG_A::Ifg80bit)
    }
    #[doc = "Minimum interframe gap is 72 bits (9 bytes) during transmission"]
    #[inline(always)]
    pub fn ifg72bit(self) -> &'a mut crate::W<REG> {
        self.variant(IFG_A::Ifg72bit)
    }
    #[doc = "Minimum interframe gap is 64 bits (8 bytes) during transmission"]
    #[inline(always)]
    pub fn ifg64bit(self) -> &'a mut crate::W<REG> {
        self.variant(IFG_A::Ifg64bit)
    }
    #[doc = "Minimum interframe gap is 56 bits (7 bytes) during transmission"]
    #[inline(always)]
    pub fn ifg56bit(self) -> &'a mut crate::W<REG> {
        self.variant(IFG_A::Ifg56bit)
    }
    #[doc = "Minimum interframe gap is 48 bits (6 bytes) during transmission"]
    #[inline(always)]
    pub fn ifg48bit(self) -> &'a mut crate::W<REG> {
        self.variant(IFG_A::Ifg48bit)
    }
    #[doc = "Minimum interframe gap is 40 bits (5 bytes) during transmission"]
    #[inline(always)]
    pub fn ifg40bit(self) -> &'a mut crate::W<REG> {
        self.variant(IFG_A::Ifg40bit)
    }
}
#[doc = "Jabber disable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JD_A {
    #[doc = "0: The MAC disables the Jabber timer on the transmitter, and can transfer frames of up to 16,384 bytes"]
    NoDisable = 0,
    #[doc = "1: Jabber disabled. The MAC allows the application to send out more than 2048 bytes of data during transmission. The MAC does not cut off the transmitter if the application sends out more than 2048 bytes of data during transmission"]
    JabberDisabled = 1,
}
impl From<JD_A> for bool {
    #[inline(always)]
    fn from(variant: JD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JD` reader - Jabber disable"]
pub type JD_R = crate::BitReader<JD_A>;
impl JD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JD_A {
        match self.bits {
            false => JD_A::NoDisable,
            true => JD_A::JabberDisabled,
        }
    }
    #[doc = "The MAC disables the Jabber timer on the transmitter, and can transfer frames of up to 16,384 bytes"]
    #[inline(always)]
    pub fn is_no_disable(&self) -> bool {
        *self == JD_A::NoDisable
    }
    #[doc = "Jabber disabled. The MAC allows the application to send out more than 2048 bytes of data during transmission. The MAC does not cut off the transmitter if the application sends out more than 2048 bytes of data during transmission"]
    #[inline(always)]
    pub fn is_jabber_disabled(&self) -> bool {
        *self == JD_A::JabberDisabled
    }
}
#[doc = "Field `JD` writer - Jabber disable"]
pub type JD_W<'a, REG> = crate::BitWriter<'a, REG, JD_A>;
impl<'a, REG> JD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The MAC disables the Jabber timer on the transmitter, and can transfer frames of up to 16,384 bytes"]
    #[inline(always)]
    pub fn no_disable(self) -> &'a mut crate::W<REG> {
        self.variant(JD_A::NoDisable)
    }
    #[doc = "Jabber disabled. The MAC allows the application to send out more than 2048 bytes of data during transmission. The MAC does not cut off the transmitter if the application sends out more than 2048 bytes of data during transmission"]
    #[inline(always)]
    pub fn jabber_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JD_A::JabberDisabled)
    }
}
#[doc = "Watchdog disable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WD_A {
    #[doc = "0: The MAC allows no more than 2048 bytes of the frames being received"]
    NoDisable = 0,
    #[doc = "1: Watchdog disabled. The MAC disables the watchdog timer on the receiver, and can receive frames of up to 16,384 bytes"]
    WatchdogDisabled = 1,
}
impl From<WD_A> for bool {
    #[inline(always)]
    fn from(variant: WD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WD` reader - Watchdog disable"]
pub type WD_R = crate::BitReader<WD_A>;
impl WD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WD_A {
        match self.bits {
            false => WD_A::NoDisable,
            true => WD_A::WatchdogDisabled,
        }
    }
    #[doc = "The MAC allows no more than 2048 bytes of the frames being received"]
    #[inline(always)]
    pub fn is_no_disable(&self) -> bool {
        *self == WD_A::NoDisable
    }
    #[doc = "Watchdog disabled. The MAC disables the watchdog timer on the receiver, and can receive frames of up to 16,384 bytes"]
    #[inline(always)]
    pub fn is_watchdog_disabled(&self) -> bool {
        *self == WD_A::WatchdogDisabled
    }
}
#[doc = "Field `WD` writer - Watchdog disable"]
pub type WD_W<'a, REG> = crate::BitWriter<'a, REG, WD_A>;
impl<'a, REG> WD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The MAC allows no more than 2048 bytes of the frames being received"]
    #[inline(always)]
    pub fn no_disable(self) -> &'a mut crate::W<REG> {
        self.variant(WD_A::NoDisable)
    }
    #[doc = "Watchdog disabled. The MAC disables the watchdog timer on the receiver, and can receive frames of up to 16,384 bytes"]
    #[inline(always)]
    pub fn watchdog_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WD_A::WatchdogDisabled)
    }
}
#[doc = "When this bit is set, the last four bytes (FCS) of the Ethernet type frame is stripped before being transmitted to the application.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CST_A {
    #[doc = "0: CRC stripping disabled"]
    NoStrip = 0,
    #[doc = "1: CRC stripping enabled. The last four bytes (FCS) of the Ethernet type frame is stripped before being transmitted to the application"]
    StrippingEnabled = 1,
}
impl From<CST_A> for bool {
    #[inline(always)]
    fn from(variant: CST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CST` reader - When this bit is set, the last four bytes (FCS) of the Ethernet type frame is stripped before being transmitted to the application."]
pub type CST_R = crate::BitReader<CST_A>;
impl CST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CST_A {
        match self.bits {
            false => CST_A::NoStrip,
            true => CST_A::StrippingEnabled,
        }
    }
    #[doc = "CRC stripping disabled"]
    #[inline(always)]
    pub fn is_no_strip(&self) -> bool {
        *self == CST_A::NoStrip
    }
    #[doc = "CRC stripping enabled. The last four bytes (FCS) of the Ethernet type frame is stripped before being transmitted to the application"]
    #[inline(always)]
    pub fn is_stripping_enabled(&self) -> bool {
        *self == CST_A::StrippingEnabled
    }
}
#[doc = "Field `CST` writer - When this bit is set, the last four bytes (FCS) of the Ethernet type frame is stripped before being transmitted to the application."]
pub type CST_W<'a, REG> = crate::BitWriter<'a, REG, CST_A>;
impl<'a, REG> CST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC stripping disabled"]
    #[inline(always)]
    pub fn no_strip(self) -> &'a mut crate::W<REG> {
        self.variant(CST_A::NoStrip)
    }
    #[doc = "CRC stripping enabled. The last four bytes (FCS) of the Ethernet type frame is stripped before being transmitted to the application"]
    #[inline(always)]
    pub fn stripping_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CST_A::StrippingEnabled)
    }
}
impl R {
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Only in half-duplex mode! Deferral check"]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Only in half-duplex mode! Back-off Limit"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Automatic pad/CRC stripping"]
    #[inline(always)]
    pub fn acs(&self) -> ACS_R {
        ACS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Only in half-duplex mode! Disable Retry"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IPv4 checksum offload"]
    #[inline(always)]
    pub fn ipc(&self) -> IPC_R {
        IPC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Duplex mode"]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Loopback mode"]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Only in half-duplex mode! Disable Receive Own."]
    #[inline(always)]
    pub fn dro(&self) -> DRO_R {
        DRO_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Fast EMAC speed"]
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Only in half-duplex mode! Disable Carrier Sense"]
    #[inline(always)]
    pub fn dcs(&self) -> DCS_R {
        DCS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Interframe gap"]
    #[inline(always)]
    pub fn ifg(&self) -> IFG_R {
        IFG_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 22 - Jabber disable"]
    #[inline(always)]
    pub fn jd(&self) -> JD_R {
        JD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Watchdog disable"]
    #[inline(always)]
    pub fn wd(&self) -> WD_R {
        WD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - When this bit is set, the last four bytes (FCS) of the Ethernet type frame is stripped before being transmitted to the application."]
    #[inline(always)]
    pub fn cst(&self) -> CST_R {
        CST_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACCTRL")
            .field("re", &self.re())
            .field("te", &self.te())
            .field("dc", &self.dc())
            .field("bl", &self.bl())
            .field("acs", &self.acs())
            .field("dr", &self.dr())
            .field("ipc", &self.ipc())
            .field("dm", &self.dm())
            .field("lm", &self.lm())
            .field("dro", &self.dro())
            .field("fes", &self.fes())
            .field("dcs", &self.dcs())
            .field("ifg", &self.ifg())
            .field("jd", &self.jd())
            .field("wd", &self.wd())
            .field("cst", &self.cst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn re(&mut self) -> RE_W<'_, MACCTRL_SPEC> {
        RE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn te(&mut self) -> TE_W<'_, MACCTRL_SPEC> {
        TE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Only in half-duplex mode! Deferral check"]
    #[inline(always)]
    pub fn dc(&mut self) -> DC_W<'_, MACCTRL_SPEC> {
        DC_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - Only in half-duplex mode! Back-off Limit"]
    #[inline(always)]
    pub fn bl(&mut self) -> BL_W<'_, MACCTRL_SPEC> {
        BL_W::new(self, 5)
    }
    #[doc = "Bit 7 - Automatic pad/CRC stripping"]
    #[inline(always)]
    pub fn acs(&mut self) -> ACS_W<'_, MACCTRL_SPEC> {
        ACS_W::new(self, 7)
    }
    #[doc = "Bit 9 - Only in half-duplex mode! Disable Retry"]
    #[inline(always)]
    pub fn dr(&mut self) -> DR_W<'_, MACCTRL_SPEC> {
        DR_W::new(self, 9)
    }
    #[doc = "Bit 10 - IPv4 checksum offload"]
    #[inline(always)]
    pub fn ipc(&mut self) -> IPC_W<'_, MACCTRL_SPEC> {
        IPC_W::new(self, 10)
    }
    #[doc = "Bit 11 - Duplex mode"]
    #[inline(always)]
    pub fn dm(&mut self) -> DM_W<'_, MACCTRL_SPEC> {
        DM_W::new(self, 11)
    }
    #[doc = "Bit 12 - Loopback mode"]
    #[inline(always)]
    pub fn lm(&mut self) -> LM_W<'_, MACCTRL_SPEC> {
        LM_W::new(self, 12)
    }
    #[doc = "Bit 13 - Only in half-duplex mode! Disable Receive Own."]
    #[inline(always)]
    pub fn dro(&mut self) -> DRO_W<'_, MACCTRL_SPEC> {
        DRO_W::new(self, 13)
    }
    #[doc = "Bit 14 - Fast EMAC speed"]
    #[inline(always)]
    pub fn fes(&mut self) -> FES_W<'_, MACCTRL_SPEC> {
        FES_W::new(self, 14)
    }
    #[doc = "Bit 16 - Only in half-duplex mode! Disable Carrier Sense"]
    #[inline(always)]
    pub fn dcs(&mut self) -> DCS_W<'_, MACCTRL_SPEC> {
        DCS_W::new(self, 16)
    }
    #[doc = "Bits 17:19 - Interframe gap"]
    #[inline(always)]
    pub fn ifg(&mut self) -> IFG_W<'_, MACCTRL_SPEC> {
        IFG_W::new(self, 17)
    }
    #[doc = "Bit 22 - Jabber disable"]
    #[inline(always)]
    pub fn jd(&mut self) -> JD_W<'_, MACCTRL_SPEC> {
        JD_W::new(self, 22)
    }
    #[doc = "Bit 23 - Watchdog disable"]
    #[inline(always)]
    pub fn wd(&mut self) -> WD_W<'_, MACCTRL_SPEC> {
        WD_W::new(self, 23)
    }
    #[doc = "Bit 25 - When this bit is set, the last four bytes (FCS) of the Ethernet type frame is stripped before being transmitted to the application."]
    #[inline(always)]
    pub fn cst(&mut self) -> CST_W<'_, MACCTRL_SPEC> {
        CST_W::new(self, 25)
    }
}
#[doc = "Ethernet MAC configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`macctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACCTRL_SPEC;
impl crate::RegisterSpec for MACCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macctrl::R`](R) reader structure"]
impl crate::Readable for MACCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macctrl::W`](W) writer structure"]
impl crate::Writable for MACCTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACCTRL to value 0x8000"]
impl crate::Resettable for MACCTRL_SPEC {
    const RESET_VALUE: u32 = 0x8000;
}
