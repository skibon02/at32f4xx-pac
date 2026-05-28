#[doc = "Register `DMASTS` reader"]
pub type R = crate::R<DMASTS_SPEC>;
#[doc = "Register `DMASTS` writer"]
pub type W = crate::W<DMASTS_SPEC>;
#[doc = "Transmit interrupt\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tir {
    #[doc = "0: No interrupt"]
    NoInterrupt = 0,
    #[doc = "1: Interrupt occurred"]
    Interrupt = 1,
}
impl From<Tir> for bool {
    #[inline(always)]
    fn from(variant: Tir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TI` reader - Transmit interrupt"]
pub type TI_R = crate::BitReader<Tir>;
impl TI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tir {
        match self.bits {
            false => Tir::NoInterrupt,
            true => Tir::Interrupt,
        }
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == Tir::NoInterrupt
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Tir::Interrupt
    }
}
#[doc = "Transmit interrupt\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TiwWO {
    #[doc = "1: Clear this interrupt flag by writing 1 to it"]
    Clear = 1,
}
impl From<TiwWO> for bool {
    #[inline(always)]
    fn from(variant: TiwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TI` writer - Transmit interrupt"]
pub type TI_W<'a, REG> = crate::BitWriter1C<'a, REG, TiwWO>;
impl<'a, REG> TI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear this interrupt flag by writing 1 to it"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TiwWO::Clear)
    }
}
#[doc = "Field `TPS` reader - Transmit process stopped"]
pub use TI_R as TPS_R;
#[doc = "Field `TBU` reader - Transmit buffer unavailable"]
pub use TI_R as TBU_R;
#[doc = "Field `TJT` reader - Transmit jabber timeout"]
pub use TI_R as TJT_R;
#[doc = "Field `OVF` reader - Receive overflow"]
pub use TI_R as OVF_R;
#[doc = "Field `UNF` reader - Transmit underflow"]
pub use TI_R as UNF_R;
#[doc = "Field `RI` reader - Receive interrupt"]
pub use TI_R as RI_R;
#[doc = "Field `RBU` reader - Receive buffer unavailable"]
pub use TI_R as RBU_R;
#[doc = "Field `RPS` reader - Receive process stopped"]
pub use TI_R as RPS_R;
#[doc = "Field `RWT` reader - Receive watchdog timeout"]
pub use TI_R as RWT_R;
#[doc = "Field `ETI` reader - Early transmit interrupt"]
pub use TI_R as ETI_R;
#[doc = "Field `FBEI` reader - Fatal bus error interrupt"]
pub use TI_R as FBEI_R;
#[doc = "Field `ERI` reader - Early receive interrupt"]
pub use TI_R as ERI_R;
#[doc = "Field `TPS` writer - Transmit process stopped"]
pub use TI_W as TPS_W;
#[doc = "Field `TBU` writer - Transmit buffer unavailable"]
pub use TI_W as TBU_W;
#[doc = "Field `TJT` writer - Transmit jabber timeout"]
pub use TI_W as TJT_W;
#[doc = "Field `OVF` writer - Receive overflow"]
pub use TI_W as OVF_W;
#[doc = "Field `UNF` writer - Transmit underflow"]
pub use TI_W as UNF_W;
#[doc = "Field `RI` writer - Receive interrupt"]
pub use TI_W as RI_W;
#[doc = "Field `RBU` writer - Receive buffer unavailable"]
pub use TI_W as RBU_W;
#[doc = "Field `RPS` writer - Receive process stopped"]
pub use TI_W as RPS_W;
#[doc = "Field `RWT` writer - Receive watchdog timeout"]
pub use TI_W as RWT_W;
#[doc = "Field `ETI` writer - Early transmit interrupt"]
pub use TI_W as ETI_W;
#[doc = "Field `FBEI` writer - Fatal bus error interrupt"]
pub use TI_W as FBEI_W;
#[doc = "Field `ERI` writer - Early receive interrupt"]
pub use TI_W as ERI_W;
#[doc = "Abnormal interrupt summary\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aisr {
    #[doc = "0: No abnormal interrupt"]
    NoInterrupt = 0,
    #[doc = "1: TPS | TJT | OVF | UNF | RBU | RPS | RWT | ETI | FBEI"]
    AbnormalInterrupt = 1,
}
impl From<Aisr> for bool {
    #[inline(always)]
    fn from(variant: Aisr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AIS` reader - Abnormal interrupt summary"]
pub type AIS_R = crate::BitReader<Aisr>;
impl AIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aisr {
        match self.bits {
            false => Aisr::NoInterrupt,
            true => Aisr::AbnormalInterrupt,
        }
    }
    #[doc = "No abnormal interrupt"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == Aisr::NoInterrupt
    }
    #[doc = "TPS | TJT | OVF | UNF | RBU | RPS | RWT | ETI | FBEI"]
    #[inline(always)]
    pub fn is_abnormal_interrupt(&self) -> bool {
        *self == Aisr::AbnormalInterrupt
    }
}
#[doc = "Abnormal interrupt summary\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AiswWO {
    #[doc = "1: Clear this interrupt flag by writing 1 to it"]
    Clear = 1,
}
impl From<AiswWO> for bool {
    #[inline(always)]
    fn from(variant: AiswWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AIS` writer - Abnormal interrupt summary"]
pub type AIS_W<'a, REG> = crate::BitWriter1C<'a, REG, AiswWO>;
impl<'a, REG> AIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear this interrupt flag by writing 1 to it"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(AiswWO::Clear)
    }
}
#[doc = "Normal interrupt summary\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nisr {
    #[doc = "0: No normal interrupt"]
    NoInterrupt = 0,
    #[doc = "1: TI | TBU | RI | ERI"]
    NormalInterrupt = 1,
}
impl From<Nisr> for bool {
    #[inline(always)]
    fn from(variant: Nisr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NIS` reader - Normal interrupt summary"]
pub type NIS_R = crate::BitReader<Nisr>;
impl NIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nisr {
        match self.bits {
            false => Nisr::NoInterrupt,
            true => Nisr::NormalInterrupt,
        }
    }
    #[doc = "No normal interrupt"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == Nisr::NoInterrupt
    }
    #[doc = "TI | TBU | RI | ERI"]
    #[inline(always)]
    pub fn is_normal_interrupt(&self) -> bool {
        *self == Nisr::NormalInterrupt
    }
}
#[doc = "Normal interrupt summary\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NiswWO {
    #[doc = "1: Clear this interrupt flag by writing 1 to it"]
    Clear = 1,
}
impl From<NiswWO> for bool {
    #[inline(always)]
    fn from(variant: NiswWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NIS` writer - Normal interrupt summary"]
pub type NIS_W<'a, REG> = crate::BitWriter1C<'a, REG, NiswWO>;
impl<'a, REG> NIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear this interrupt flag by writing 1 to it"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(NiswWO::Clear)
    }
}
#[doc = "Receive process state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RS_A {
    #[doc = "0: Stopped. Rest or Stop transmit command issued"]
    Stopped = 0,
    #[doc = "1: Running. Fetching receive descriptor"]
    RunningFetching = 1,
    #[doc = "3: Running. Waiting for receive packet"]
    RunningWaiting = 3,
    #[doc = "4: Suspended. Receive descriptor unavailable"]
    Suspended = 4,
    #[doc = "5: Running. Closing receive descriptor"]
    RunningClosing = 5,
    #[doc = "6: Time stamp write status"]
    Timestamp = 6,
    #[doc = "7: Running. Transferring the receive buffer data to host memory"]
    RunningTransferring = 7,
}
impl From<RS_A> for u8 {
    #[inline(always)]
    fn from(variant: RS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RS_A {
    type Ux = u8;
}
impl crate::IsEnum for RS_A {}
#[doc = "Field `RS` reader - Receive process state"]
pub type RS_R = crate::FieldReader<RS_A>;
impl RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RS_A> {
        match self.bits {
            0 => Some(RS_A::Stopped),
            1 => Some(RS_A::RunningFetching),
            3 => Some(RS_A::RunningWaiting),
            4 => Some(RS_A::Suspended),
            5 => Some(RS_A::RunningClosing),
            6 => Some(RS_A::Timestamp),
            7 => Some(RS_A::RunningTransferring),
            _ => None,
        }
    }
    #[doc = "Stopped. Rest or Stop transmit command issued"]
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == RS_A::Stopped
    }
    #[doc = "Running. Fetching receive descriptor"]
    #[inline(always)]
    pub fn is_running_fetching(&self) -> bool {
        *self == RS_A::RunningFetching
    }
    #[doc = "Running. Waiting for receive packet"]
    #[inline(always)]
    pub fn is_running_waiting(&self) -> bool {
        *self == RS_A::RunningWaiting
    }
    #[doc = "Suspended. Receive descriptor unavailable"]
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        *self == RS_A::Suspended
    }
    #[doc = "Running. Closing receive descriptor"]
    #[inline(always)]
    pub fn is_running_closing(&self) -> bool {
        *self == RS_A::RunningClosing
    }
    #[doc = "Time stamp write status"]
    #[inline(always)]
    pub fn is_timestamp(&self) -> bool {
        *self == RS_A::Timestamp
    }
    #[doc = "Running. Transferring the receive buffer data to host memory"]
    #[inline(always)]
    pub fn is_running_transferring(&self) -> bool {
        *self == RS_A::RunningTransferring
    }
}
#[doc = "Transmit process state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TS_A {
    #[doc = "0: Stopped. Rest or Stop transmit command issued"]
    Stopped = 0,
    #[doc = "1: Running. Fetching receive descriptor"]
    RunningFetching = 1,
    #[doc = "2: Running. Waiting for status"]
    RunningWaiting = 2,
    #[doc = "3: Running. Reading data from host memory buffer and queuing it to Tx FIFO"]
    RunningReading = 3,
    #[doc = "4: Time stamp write status"]
    Timestamp = 4,
    #[doc = "6: Suspended. Transmit descriptor unavailable or transmit buffer underflow"]
    Suspended = 6,
    #[doc = "7: Running. Closing transmit descriptor"]
    RunningClosing = 7,
}
impl From<TS_A> for u8 {
    #[inline(always)]
    fn from(variant: TS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TS_A {
    type Ux = u8;
}
impl crate::IsEnum for TS_A {}
#[doc = "Field `TS` reader - Transmit process state"]
pub type TS_R = crate::FieldReader<TS_A>;
impl TS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TS_A> {
        match self.bits {
            0 => Some(TS_A::Stopped),
            1 => Some(TS_A::RunningFetching),
            2 => Some(TS_A::RunningWaiting),
            3 => Some(TS_A::RunningReading),
            4 => Some(TS_A::Timestamp),
            6 => Some(TS_A::Suspended),
            7 => Some(TS_A::RunningClosing),
            _ => None,
        }
    }
    #[doc = "Stopped. Rest or Stop transmit command issued"]
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == TS_A::Stopped
    }
    #[doc = "Running. Fetching receive descriptor"]
    #[inline(always)]
    pub fn is_running_fetching(&self) -> bool {
        *self == TS_A::RunningFetching
    }
    #[doc = "Running. Waiting for status"]
    #[inline(always)]
    pub fn is_running_waiting(&self) -> bool {
        *self == TS_A::RunningWaiting
    }
    #[doc = "Running. Reading data from host memory buffer and queuing it to Tx FIFO"]
    #[inline(always)]
    pub fn is_running_reading(&self) -> bool {
        *self == TS_A::RunningReading
    }
    #[doc = "Time stamp write status"]
    #[inline(always)]
    pub fn is_timestamp(&self) -> bool {
        *self == TS_A::Timestamp
    }
    #[doc = "Suspended. Transmit descriptor unavailable or transmit buffer underflow"]
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        *self == TS_A::Suspended
    }
    #[doc = "Running. Closing transmit descriptor"]
    #[inline(always)]
    pub fn is_running_closing(&self) -> bool {
        *self == TS_A::RunningClosing
    }
}
#[doc = "Error bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EB_A {
    #[doc = "0: Error during Rx DMA transfer"]
    RxDmaTransfer = 0,
    #[doc = "3: Error during Tx DMA transfer"]
    TxDmaTransfer = 3,
    #[doc = "4: Error during write access to Rx DMA descriptor"]
    RxDmaDescriptorWriteAccess = 4,
    #[doc = "5: Error during write access to Tx DMA descriptor"]
    TxDmaDescriptorWriteAccess = 5,
    #[doc = "6: Error during read access to Rx DMA descriptor"]
    RxDmaDescriptorReadAccess = 6,
    #[doc = "7: Error during read access to Tx DMA descriptor"]
    TxDmaDescriptorReadAccess = 7,
}
impl From<EB_A> for u8 {
    #[inline(always)]
    fn from(variant: EB_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EB_A {
    type Ux = u8;
}
impl crate::IsEnum for EB_A {}
#[doc = "Field `EB` reader - Error bits"]
pub type EB_R = crate::FieldReader<EB_A>;
impl EB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EB_A> {
        match self.bits {
            0 => Some(EB_A::RxDmaTransfer),
            3 => Some(EB_A::TxDmaTransfer),
            4 => Some(EB_A::RxDmaDescriptorWriteAccess),
            5 => Some(EB_A::TxDmaDescriptorWriteAccess),
            6 => Some(EB_A::RxDmaDescriptorReadAccess),
            7 => Some(EB_A::TxDmaDescriptorReadAccess),
            _ => None,
        }
    }
    #[doc = "Error during Rx DMA transfer"]
    #[inline(always)]
    pub fn is_rx_dma_transfer(&self) -> bool {
        *self == EB_A::RxDmaTransfer
    }
    #[doc = "Error during Tx DMA transfer"]
    #[inline(always)]
    pub fn is_tx_dma_transfer(&self) -> bool {
        *self == EB_A::TxDmaTransfer
    }
    #[doc = "Error during write access to Rx DMA descriptor"]
    #[inline(always)]
    pub fn is_rx_dma_descriptor_write_access(&self) -> bool {
        *self == EB_A::RxDmaDescriptorWriteAccess
    }
    #[doc = "Error during write access to Tx DMA descriptor"]
    #[inline(always)]
    pub fn is_tx_dma_descriptor_write_access(&self) -> bool {
        *self == EB_A::TxDmaDescriptorWriteAccess
    }
    #[doc = "Error during read access to Rx DMA descriptor"]
    #[inline(always)]
    pub fn is_rx_dma_descriptor_read_access(&self) -> bool {
        *self == EB_A::RxDmaDescriptorReadAccess
    }
    #[doc = "Error during read access to Tx DMA descriptor"]
    #[inline(always)]
    pub fn is_tx_dma_descriptor_read_access(&self) -> bool {
        *self == EB_A::TxDmaDescriptorReadAccess
    }
}
#[doc = "MAC MMC interrupt\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMIR_A {
    #[doc = "0: No MMC interrupt"]
    NoMmcInterrupt = 0,
    #[doc = "1: MMC interrupt occurred"]
    MmcInterrupt = 1,
}
impl From<MMIR_A> for bool {
    #[inline(always)]
    fn from(variant: MMIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMI` reader - MAC MMC interrupt"]
pub type MMI_R = crate::BitReader<MMIR_A>;
impl MMI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MMIR_A {
        match self.bits {
            false => MMIR_A::NoMmcInterrupt,
            true => MMIR_A::MmcInterrupt,
        }
    }
    #[doc = "No MMC interrupt"]
    #[inline(always)]
    pub fn is_no_mmc_interrupt(&self) -> bool {
        *self == MMIR_A::NoMmcInterrupt
    }
    #[doc = "MMC interrupt occurred"]
    #[inline(always)]
    pub fn is_mmc_interrupt(&self) -> bool {
        *self == MMIR_A::MmcInterrupt
    }
}
#[doc = "MAC PMT interrupt\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPIR_A {
    #[doc = "0: No PMT interrupt"]
    NoPmtInterrupt = 0,
    #[doc = "1: PMT interrupt occurred"]
    PmtInterrupt = 1,
}
impl From<MPIR_A> for bool {
    #[inline(always)]
    fn from(variant: MPIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPI` reader - MAC PMT interrupt"]
pub type MPI_R = crate::BitReader<MPIR_A>;
impl MPI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MPIR_A {
        match self.bits {
            false => MPIR_A::NoPmtInterrupt,
            true => MPIR_A::PmtInterrupt,
        }
    }
    #[doc = "No PMT interrupt"]
    #[inline(always)]
    pub fn is_no_pmt_interrupt(&self) -> bool {
        *self == MPIR_A::NoPmtInterrupt
    }
    #[doc = "PMT interrupt occurred"]
    #[inline(always)]
    pub fn is_pmt_interrupt(&self) -> bool {
        *self == MPIR_A::PmtInterrupt
    }
}
#[doc = "Timestamp trigger interrupt\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TTIR_A {
    #[doc = "0: No timestamp interrupt"]
    NoTimestampInterrupt = 0,
    #[doc = "1: Timestamp interrupt occurred"]
    TimestampInterrupt = 1,
}
impl From<TTIR_A> for bool {
    #[inline(always)]
    fn from(variant: TTIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TTI` reader - Timestamp trigger interrupt"]
pub type TTI_R = crate::BitReader<TTIR_A>;
impl TTI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TTIR_A {
        match self.bits {
            false => TTIR_A::NoTimestampInterrupt,
            true => TTIR_A::TimestampInterrupt,
        }
    }
    #[doc = "No timestamp interrupt"]
    #[inline(always)]
    pub fn is_no_timestamp_interrupt(&self) -> bool {
        *self == TTIR_A::NoTimestampInterrupt
    }
    #[doc = "Timestamp interrupt occurred"]
    #[inline(always)]
    pub fn is_timestamp_interrupt(&self) -> bool {
        *self == TTIR_A::TimestampInterrupt
    }
}
impl R {
    #[doc = "Bit 0 - Transmit interrupt"]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit process stopped"]
    #[inline(always)]
    pub fn tps(&self) -> TPS_R {
        TPS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable"]
    #[inline(always)]
    pub fn tbu(&self) -> TBU_R {
        TBU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit jabber timeout"]
    #[inline(always)]
    pub fn tjt(&self) -> TJT_R {
        TJT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive overflow"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit underflow"]
    #[inline(always)]
    pub fn unf(&self) -> UNF_R {
        UNF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive interrupt"]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive buffer unavailable"]
    #[inline(always)]
    pub fn rbu(&self) -> RBU_R {
        RBU_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive process stopped"]
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive watchdog timeout"]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early transmit interrupt"]
    #[inline(always)]
    pub fn eti(&self) -> ETI_R {
        ETI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal bus error interrupt"]
    #[inline(always)]
    pub fn fbei(&self) -> FBEI_R {
        FBEI_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early receive interrupt"]
    #[inline(always)]
    pub fn eri(&self) -> ERI_R {
        ERI_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal interrupt summary"]
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal interrupt summary"]
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Receive process state"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Transmit process state"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - Error bits"]
    #[inline(always)]
    pub fn eb(&self) -> EB_R {
        EB_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 27 - MAC MMC interrupt"]
    #[inline(always)]
    pub fn mmi(&self) -> MMI_R {
        MMI_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - MAC PMT interrupt"]
    #[inline(always)]
    pub fn mpi(&self) -> MPI_R {
        MPI_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Timestamp trigger interrupt"]
    #[inline(always)]
    pub fn tti(&self) -> TTI_R {
        TTI_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMASTS")
            .field("ti", &self.ti())
            .field("tps", &self.tps())
            .field("tbu", &self.tbu())
            .field("tjt", &self.tjt())
            .field("ovf", &self.ovf())
            .field("unf", &self.unf())
            .field("ri", &self.ri())
            .field("rbu", &self.rbu())
            .field("rps", &self.rps())
            .field("rwt", &self.rwt())
            .field("eti", &self.eti())
            .field("fbei", &self.fbei())
            .field("eri", &self.eri())
            .field("ais", &self.ais())
            .field("nis", &self.nis())
            .field("rs", &self.rs())
            .field("ts", &self.ts())
            .field("eb", &self.eb())
            .field("mmi", &self.mmi())
            .field("mpi", &self.mpi())
            .field("tti", &self.tti())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transmit interrupt"]
    #[inline(always)]
    pub fn ti(&mut self) -> TI_W<'_, DMASTS_SPEC> {
        TI_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit process stopped"]
    #[inline(always)]
    pub fn tps(&mut self) -> TPS_W<'_, DMASTS_SPEC> {
        TPS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable"]
    #[inline(always)]
    pub fn tbu(&mut self) -> TBU_W<'_, DMASTS_SPEC> {
        TBU_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit jabber timeout"]
    #[inline(always)]
    pub fn tjt(&mut self) -> TJT_W<'_, DMASTS_SPEC> {
        TJT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Receive overflow"]
    #[inline(always)]
    pub fn ovf(&mut self) -> OVF_W<'_, DMASTS_SPEC> {
        OVF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit underflow"]
    #[inline(always)]
    pub fn unf(&mut self) -> UNF_W<'_, DMASTS_SPEC> {
        UNF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Receive interrupt"]
    #[inline(always)]
    pub fn ri(&mut self) -> RI_W<'_, DMASTS_SPEC> {
        RI_W::new(self, 6)
    }
    #[doc = "Bit 7 - Receive buffer unavailable"]
    #[inline(always)]
    pub fn rbu(&mut self) -> RBU_W<'_, DMASTS_SPEC> {
        RBU_W::new(self, 7)
    }
    #[doc = "Bit 8 - Receive process stopped"]
    #[inline(always)]
    pub fn rps(&mut self) -> RPS_W<'_, DMASTS_SPEC> {
        RPS_W::new(self, 8)
    }
    #[doc = "Bit 9 - Receive watchdog timeout"]
    #[inline(always)]
    pub fn rwt(&mut self) -> RWT_W<'_, DMASTS_SPEC> {
        RWT_W::new(self, 9)
    }
    #[doc = "Bit 10 - Early transmit interrupt"]
    #[inline(always)]
    pub fn eti(&mut self) -> ETI_W<'_, DMASTS_SPEC> {
        ETI_W::new(self, 10)
    }
    #[doc = "Bit 13 - Fatal bus error interrupt"]
    #[inline(always)]
    pub fn fbei(&mut self) -> FBEI_W<'_, DMASTS_SPEC> {
        FBEI_W::new(self, 13)
    }
    #[doc = "Bit 14 - Early receive interrupt"]
    #[inline(always)]
    pub fn eri(&mut self) -> ERI_W<'_, DMASTS_SPEC> {
        ERI_W::new(self, 14)
    }
    #[doc = "Bit 15 - Abnormal interrupt summary"]
    #[inline(always)]
    pub fn ais(&mut self) -> AIS_W<'_, DMASTS_SPEC> {
        AIS_W::new(self, 15)
    }
    #[doc = "Bit 16 - Normal interrupt summary"]
    #[inline(always)]
    pub fn nis(&mut self) -> NIS_W<'_, DMASTS_SPEC> {
        NIS_W::new(self, 16)
    }
}
#[doc = "Ethernet DMA status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMASTS_SPEC;
impl crate::RegisterSpec for DMASTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmasts::R`](R) reader structure"]
impl crate::Readable for DMASTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmasts::W`](W) writer structure"]
impl crate::Writable for DMASTS_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0001_e7ff;
}
#[doc = "`reset()` method sets DMASTS to value 0"]
impl crate::Resettable for DMASTS_SPEC {}
