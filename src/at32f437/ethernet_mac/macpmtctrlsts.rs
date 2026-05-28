#[doc = "Register `MACPMTCTRLSTS` reader"]
pub type R = crate::R<MACPMTCTRLSTS_SPEC>;
#[doc = "Register `MACPMTCTRLSTS` writer"]
pub type W = crate::W<MACPMTCTRLSTS_SPEC>;
#[doc = "Power down\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDW_A {
    #[doc = "1: Power down. Power back up when receive expected Magic packet or clear this bit"]
    PowerDown = 1,
}
impl From<PDW_A> for bool {
    #[inline(always)]
    fn from(variant: PDW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD` reader - Power down"]
pub type PD_R = crate::BitReader<PDW_A>;
impl PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PDW_A> {
        match self.bits {
            true => Some(PDW_A::PowerDown),
            _ => None,
        }
    }
    #[doc = "Power down. Power back up when receive expected Magic packet or clear this bit"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == PDW_A::PowerDown
    }
}
#[doc = "Field `PD` writer - Power down"]
pub type PD_W<'a, REG> = crate::BitWriter1S<'a, REG, PDW_A>;
impl<'a, REG> PD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power down. Power back up when receive expected Magic packet or clear this bit"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(PDW_A::PowerDown)
    }
}
#[doc = "Enable magic packet\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EMP_A {
    #[doc = "0: Disable magic packet detection"]
    Disable = 0,
    #[doc = "1: Enable magic packet detection. Power management event is generated when a magic packet is received."]
    Enable = 1,
}
impl From<EMP_A> for bool {
    #[inline(always)]
    fn from(variant: EMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMP` reader - Enable magic packet"]
pub type EMP_R = crate::BitReader<EMP_A>;
impl EMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EMP_A {
        match self.bits {
            false => EMP_A::Disable,
            true => EMP_A::Enable,
        }
    }
    #[doc = "Disable magic packet detection"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EMP_A::Disable
    }
    #[doc = "Enable magic packet detection. Power management event is generated when a magic packet is received."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EMP_A::Enable
    }
}
#[doc = "Field `EMP` writer - Enable magic packet"]
pub type EMP_W<'a, REG> = crate::BitWriter<'a, REG, EMP_A>;
impl<'a, REG> EMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable magic packet detection"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EMP_A::Disable)
    }
    #[doc = "Enable magic packet detection. Power management event is generated when a magic packet is received."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EMP_A::Enable)
    }
}
#[doc = "Enable remote wakeup frame\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERWF_A {
    #[doc = "0: Disable remote wakeup frame detection"]
    Disable = 0,
    #[doc = "1: Enable remote wakeup frame detection. Power management event is generated when a remote wakeup frame is received."]
    Enable = 1,
}
impl From<ERWF_A> for bool {
    #[inline(always)]
    fn from(variant: ERWF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERWF` reader - Enable remote wakeup frame"]
pub type ERWF_R = crate::BitReader<ERWF_A>;
impl ERWF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERWF_A {
        match self.bits {
            false => ERWF_A::Disable,
            true => ERWF_A::Enable,
        }
    }
    #[doc = "Disable remote wakeup frame detection"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERWF_A::Disable
    }
    #[doc = "Enable remote wakeup frame detection. Power management event is generated when a remote wakeup frame is received."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERWF_A::Enable
    }
}
#[doc = "Field `ERWF` writer - Enable remote wakeup frame"]
pub type ERWF_W<'a, REG> = crate::BitWriter<'a, REG, ERWF_A>;
impl<'a, REG> ERWF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable remote wakeup frame detection"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ERWF_A::Disable)
    }
    #[doc = "Enable remote wakeup frame detection. Power management event is generated when a remote wakeup frame is received."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ERWF_A::Enable)
    }
}
#[doc = "Received magic packet\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMP_A {
    #[doc = "0: No Magic Packet has been received since the last read of this register"]
    NoMagicPacket = 0,
    #[doc = "1: Magic Packet has been received since the last read of this register"]
    MagicPacketReceived = 1,
}
impl From<RMP_A> for bool {
    #[inline(always)]
    fn from(variant: RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMP` reader - Received magic packet\n\n<div class=\"warning\">The field is <b>cleared</b> (set to zero) following a read operation.</div>"]
pub type RMP_R = crate::BitReader<RMP_A>;
impl RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RMP_A {
        match self.bits {
            false => RMP_A::NoMagicPacket,
            true => RMP_A::MagicPacketReceived,
        }
    }
    #[doc = "No Magic Packet has been received since the last read of this register"]
    #[inline(always)]
    pub fn is_no_magic_packet(&self) -> bool {
        *self == RMP_A::NoMagicPacket
    }
    #[doc = "Magic Packet has been received since the last read of this register"]
    #[inline(always)]
    pub fn is_magic_packet_received(&self) -> bool {
        *self == RMP_A::MagicPacketReceived
    }
}
#[doc = "Received remote wakeup frame\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRWF_A {
    #[doc = "0: No remote wakeup frame has been received since the last read of this register"]
    NoWakeup = 0,
    #[doc = "1: Remote wakeup frame has been received since the last read of this register"]
    WakeupReceived = 1,
}
impl From<RRWF_A> for bool {
    #[inline(always)]
    fn from(variant: RRWF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRWF` reader - Received remote wakeup frame\n\n<div class=\"warning\">The field is <b>cleared</b> (set to zero) following a read operation.</div>"]
pub type RRWF_R = crate::BitReader<RRWF_A>;
impl RRWF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RRWF_A {
        match self.bits {
            false => RRWF_A::NoWakeup,
            true => RRWF_A::WakeupReceived,
        }
    }
    #[doc = "No remote wakeup frame has been received since the last read of this register"]
    #[inline(always)]
    pub fn is_no_wakeup(&self) -> bool {
        *self == RRWF_A::NoWakeup
    }
    #[doc = "Remote wakeup frame has been received since the last read of this register"]
    #[inline(always)]
    pub fn is_wakeup_received(&self) -> bool {
        *self == RRWF_A::WakeupReceived
    }
}
#[doc = "Global unicast\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GUC_A {
    #[doc = "0: Normal operation"]
    Disable = 0,
    #[doc = "1: All unicast packets filtered by the MAC address filters are treated as wakeup frames. If any of these packets is received, a PMT event is generated"]
    GlobalUnicast = 1,
}
impl From<GUC_A> for bool {
    #[inline(always)]
    fn from(variant: GUC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GUC` reader - Global unicast"]
pub type GUC_R = crate::BitReader<GUC_A>;
impl GUC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GUC_A {
        match self.bits {
            false => GUC_A::Disable,
            true => GUC_A::GlobalUnicast,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GUC_A::Disable
    }
    #[doc = "All unicast packets filtered by the MAC address filters are treated as wakeup frames. If any of these packets is received, a PMT event is generated"]
    #[inline(always)]
    pub fn is_global_unicast(&self) -> bool {
        *self == GUC_A::GlobalUnicast
    }
}
#[doc = "Field `GUC` writer - Global unicast"]
pub type GUC_W<'a, REG> = crate::BitWriter<'a, REG, GUC_A>;
impl<'a, REG> GUC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(GUC_A::Disable)
    }
    #[doc = "All unicast packets filtered by the MAC address filters are treated as wakeup frames. If any of these packets is received, a PMT event is generated"]
    #[inline(always)]
    pub fn global_unicast(self) -> &'a mut crate::W<REG> {
        self.variant(GUC_A::GlobalUnicast)
    }
}
#[doc = "Remote wakeup frame filter register pointer reset\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWFFPRW_A {
    #[doc = "1: Resets the remote frame filter register pointer to 3’b000"]
    ResetPointer = 1,
}
impl From<RWFFPRW_A> for bool {
    #[inline(always)]
    fn from(variant: RWFFPRW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWFFPR` reader - Remote wakeup frame filter register pointer reset"]
pub type RWFFPR_R = crate::BitReader<RWFFPRW_A>;
impl RWFFPR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RWFFPRW_A> {
        match self.bits {
            true => Some(RWFFPRW_A::ResetPointer),
            _ => None,
        }
    }
    #[doc = "Resets the remote frame filter register pointer to 3’b000"]
    #[inline(always)]
    pub fn is_reset_pointer(&self) -> bool {
        *self == RWFFPRW_A::ResetPointer
    }
}
#[doc = "Field `RWFFPR` writer - Remote wakeup frame filter register pointer reset"]
pub type RWFFPR_W<'a, REG> = crate::BitWriter1S<'a, REG, RWFFPRW_A>;
impl<'a, REG> RWFFPR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resets the remote frame filter register pointer to 3’b000"]
    #[inline(always)]
    pub fn reset_pointer(self) -> &'a mut crate::W<REG> {
        self.variant(RWFFPRW_A::ResetPointer)
    }
}
impl R {
    #[doc = "Bit 0 - Power down"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable magic packet"]
    #[inline(always)]
    pub fn emp(&self) -> EMP_R {
        EMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable remote wakeup frame"]
    #[inline(always)]
    pub fn erwf(&self) -> ERWF_R {
        ERWF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Received magic packet"]
    #[inline(always)]
    pub fn rmp(&self) -> RMP_R {
        RMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Received remote wakeup frame"]
    #[inline(always)]
    pub fn rrwf(&self) -> RRWF_R {
        RRWF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Global unicast"]
    #[inline(always)]
    pub fn guc(&self) -> GUC_R {
        GUC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - Remote wakeup frame filter register pointer reset"]
    #[inline(always)]
    pub fn rwffpr(&self) -> RWFFPR_R {
        RWFFPR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACPMTCTRLSTS")
            .field("pd", &self.pd())
            .field("emp", &self.emp())
            .field("erwf", &self.erwf())
            .field("guc", &self.guc())
            .field("rwffpr", &self.rwffpr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Power down"]
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W<'_, MACPMTCTRLSTS_SPEC> {
        PD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable magic packet"]
    #[inline(always)]
    pub fn emp(&mut self) -> EMP_W<'_, MACPMTCTRLSTS_SPEC> {
        EMP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable remote wakeup frame"]
    #[inline(always)]
    pub fn erwf(&mut self) -> ERWF_W<'_, MACPMTCTRLSTS_SPEC> {
        ERWF_W::new(self, 2)
    }
    #[doc = "Bit 9 - Global unicast"]
    #[inline(always)]
    pub fn guc(&mut self) -> GUC_W<'_, MACPMTCTRLSTS_SPEC> {
        GUC_W::new(self, 9)
    }
    #[doc = "Bit 31 - Remote wakeup frame filter register pointer reset"]
    #[inline(always)]
    pub fn rwffpr(&mut self) -> RWFFPR_W<'_, MACPMTCTRLSTS_SPEC> {
        RWFFPR_W::new(self, 31)
    }
}
#[doc = "Ethernet MAC PMT control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`macpmtctrlsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macpmtctrlsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACPMTCTRLSTS_SPEC;
impl crate::RegisterSpec for MACPMTCTRLSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macpmtctrlsts::R`](R) reader structure"]
impl crate::Readable for MACPMTCTRLSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macpmtctrlsts::W`](W) writer structure"]
impl crate::Writable for MACPMTCTRLSTS_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x8000_0001;
}
#[doc = "`reset()` method sets MACPMTCTRLSTS to value 0"]
impl crate::Resettable for MACPMTCTRLSTS_SPEC {}
