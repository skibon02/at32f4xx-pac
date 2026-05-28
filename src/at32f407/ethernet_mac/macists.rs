#[doc = "Register `MACISTS` reader"]
pub type R = crate::R<MACISTS_SPEC>;
#[doc = "Register `MACISTS` writer"]
pub type W = crate::W<MACISTS_SPEC>;
#[doc = "PMT interrupt status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIS_A {
    #[doc = "0: No PMT interrupt"]
    NoPmt = 0,
    #[doc = "1: Magic packet or a remote wakeup event is received in power-down mode"]
    PmtInterrupt = 1,
}
impl From<PIS_A> for bool {
    #[inline(always)]
    fn from(variant: PIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIS` reader - PMT interrupt status"]
pub type PIS_R = crate::BitReader<PIS_A>;
impl PIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIS_A {
        match self.bits {
            false => PIS_A::NoPmt,
            true => PIS_A::PmtInterrupt,
        }
    }
    #[doc = "No PMT interrupt"]
    #[inline(always)]
    pub fn is_no_pmt(&self) -> bool {
        *self == PIS_A::NoPmt
    }
    #[doc = "Magic packet or a remote wakeup event is received in power-down mode"]
    #[inline(always)]
    pub fn is_pmt_interrupt(&self) -> bool {
        *self == PIS_A::PmtInterrupt
    }
}
#[doc = "MMC interrupt status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIS_A {
    #[doc = "0: No MMC interrupt"]
    NoMmc = 0,
    #[doc = "1: Interrupt event is generated in the EMAC_MMCRI or EMAC_MMCTI register."]
    MmcInterrupt = 1,
}
impl From<MIS_A> for bool {
    #[inline(always)]
    fn from(variant: MIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS` reader - MMC interrupt status"]
pub type MIS_R = crate::BitReader<MIS_A>;
impl MIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MIS_A {
        match self.bits {
            false => MIS_A::NoMmc,
            true => MIS_A::MmcInterrupt,
        }
    }
    #[doc = "No MMC interrupt"]
    #[inline(always)]
    pub fn is_no_mmc(&self) -> bool {
        *self == MIS_A::NoMmc
    }
    #[doc = "Interrupt event is generated in the EMAC_MMCRI or EMAC_MMCTI register."]
    #[inline(always)]
    pub fn is_mmc_interrupt(&self) -> bool {
        *self == MIS_A::MmcInterrupt
    }
}
#[doc = "MMC receive interrupt status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MRIS_A {
    #[doc = "0: No MMC receive interrupt"]
    NoMmcReceive = 0,
    #[doc = "1: Interrupt event is generated in the EMAC_MMCRI register."]
    MmcReceiveInterrupt = 1,
}
impl From<MRIS_A> for bool {
    #[inline(always)]
    fn from(variant: MRIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRIS` reader - MMC receive interrupt status"]
pub type MRIS_R = crate::BitReader<MRIS_A>;
impl MRIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MRIS_A {
        match self.bits {
            false => MRIS_A::NoMmcReceive,
            true => MRIS_A::MmcReceiveInterrupt,
        }
    }
    #[doc = "No MMC receive interrupt"]
    #[inline(always)]
    pub fn is_no_mmc_receive(&self) -> bool {
        *self == MRIS_A::NoMmcReceive
    }
    #[doc = "Interrupt event is generated in the EMAC_MMCRI register."]
    #[inline(always)]
    pub fn is_mmc_receive_interrupt(&self) -> bool {
        *self == MRIS_A::MmcReceiveInterrupt
    }
}
#[doc = "MMC transmit interrupt status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTIS_A {
    #[doc = "0: No MMC transmit interrupt"]
    NoMmcTransmit = 0,
    #[doc = "1: Interrupt event is generated in the EMAC_MMCTI register."]
    MmcTransmitInterrupt = 1,
}
impl From<MTIS_A> for bool {
    #[inline(always)]
    fn from(variant: MTIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTIS` reader - MMC transmit interrupt status"]
pub type MTIS_R = crate::BitReader<MTIS_A>;
impl MTIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MTIS_A {
        match self.bits {
            false => MTIS_A::NoMmcTransmit,
            true => MTIS_A::MmcTransmitInterrupt,
        }
    }
    #[doc = "No MMC transmit interrupt"]
    #[inline(always)]
    pub fn is_no_mmc_transmit(&self) -> bool {
        *self == MTIS_A::NoMmcTransmit
    }
    #[doc = "Interrupt event is generated in the EMAC_MMCTI register."]
    #[inline(always)]
    pub fn is_mmc_transmit_interrupt(&self) -> bool {
        *self == MTIS_A::MmcTransmitInterrupt
    }
}
#[doc = "Timestamp interrupt status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TISR_A {
    #[doc = "0: No timestamp interrupt"]
    NoTimestamp = 0,
    #[doc = "1: System time value equals or exceeds the value programmed in the destination time registers"]
    TimestampInterrupt = 1,
}
impl From<TISR_A> for bool {
    #[inline(always)]
    fn from(variant: TISR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIS` reader - Timestamp interrupt status\n\n<div class=\"warning\">The field is <b>cleared</b> (set to zero) following a read operation.</div>"]
pub type TIS_R = crate::BitReader<TISR_A>;
impl TIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TISR_A {
        match self.bits {
            false => TISR_A::NoTimestamp,
            true => TISR_A::TimestampInterrupt,
        }
    }
    #[doc = "No timestamp interrupt"]
    #[inline(always)]
    pub fn is_no_timestamp(&self) -> bool {
        *self == TISR_A::NoTimestamp
    }
    #[doc = "System time value equals or exceeds the value programmed in the destination time registers"]
    #[inline(always)]
    pub fn is_timestamp_interrupt(&self) -> bool {
        *self == TISR_A::TimestampInterrupt
    }
}
impl R {
    #[doc = "Bit 3 - PMT interrupt status"]
    #[inline(always)]
    pub fn pis(&self) -> PIS_R {
        PIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC interrupt status"]
    #[inline(always)]
    pub fn mis(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC receive interrupt status"]
    #[inline(always)]
    pub fn mris(&self) -> MRIS_R {
        MRIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC transmit interrupt status"]
    #[inline(always)]
    pub fn mtis(&self) -> MTIS_R {
        MTIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp interrupt status"]
    #[inline(always)]
    pub fn tis(&self) -> TIS_R {
        TIS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACISTS")
            .field("pis", &self.pis())
            .field("mis", &self.mis())
            .field("mris", &self.mris())
            .field("mtis", &self.mtis())
            .finish()
    }
}
impl W {}
#[doc = "Ethernet MAC interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`macists::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macists::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACISTS_SPEC;
impl crate::RegisterSpec for MACISTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macists::R`](R) reader structure"]
impl crate::Readable for MACISTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macists::W`](W) writer structure"]
impl crate::Writable for MACISTS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACISTS to value 0"]
impl crate::Resettable for MACISTS_SPEC {}
