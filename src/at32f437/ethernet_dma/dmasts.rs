#[doc = "Register `DMASTS` reader"]
pub type R = crate::R<DMASTS_SPEC>;
#[doc = "Register `DMASTS` writer"]
pub type W = crate::W<DMASTS_SPEC>;
#[doc = "Field `TI` reader - Transmit interrupt"]
pub type TI_R = crate::BitReader;
#[doc = "Field `TI` writer - Transmit interrupt"]
pub type TI_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TPS` reader - Transmit process stopped"]
pub type TPS_R = crate::BitReader;
#[doc = "Field `TPS` writer - Transmit process stopped"]
pub type TPS_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TBU` reader - Transmit buffer unavailable"]
pub type TBU_R = crate::BitReader;
#[doc = "Field `TBU` writer - Transmit buffer unavailable"]
pub type TBU_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TJT` reader - Transmit jabber timeout"]
pub type TJT_R = crate::BitReader;
#[doc = "Field `TJT` writer - Transmit jabber timeout"]
pub type TJT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OVF` reader - Receive overflow"]
pub type OVF_R = crate::BitReader;
#[doc = "Field `OVF` writer - Receive overflow"]
pub type OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `UNF` reader - Transmit underflow"]
pub type UNF_R = crate::BitReader;
#[doc = "Field `UNF` writer - Transmit underflow"]
pub type UNF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RI` reader - Receive interrupt"]
pub type RI_R = crate::BitReader;
#[doc = "Field `RI` writer - Receive interrupt"]
pub type RI_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RBU` reader - Receive buffer unavailable"]
pub type RBU_R = crate::BitReader;
#[doc = "Field `RBU` writer - Receive buffer unavailable"]
pub type RBU_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RPS` reader - Receive process stopped"]
pub type RPS_R = crate::BitReader;
#[doc = "Field `RPS` writer - Receive process stopped"]
pub type RPS_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RWT` reader - Receive watchdog timeout"]
pub type RWT_R = crate::BitReader;
#[doc = "Field `RWT` writer - Receive watchdog timeout"]
pub type RWT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ETI` reader - Early transmit interrupt"]
pub type ETI_R = crate::BitReader;
#[doc = "Field `ETI` writer - Early transmit interrupt"]
pub type ETI_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FBEI` reader - Fatal bus error interrupt"]
pub type FBEI_R = crate::BitReader;
#[doc = "Field `FBEI` writer - Fatal bus error interrupt"]
pub type FBEI_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ERI` reader - Early receive interrupt"]
pub type ERI_R = crate::BitReader;
#[doc = "Field `ERI` writer - Early receive interrupt"]
pub type ERI_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AIS` reader - Abnormal interrupt summary"]
pub type AIS_R = crate::BitReader;
#[doc = "Field `AIS` writer - Abnormal interrupt summary"]
pub type AIS_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `NIS` reader - Normal interrupt summary"]
pub type NIS_R = crate::BitReader;
#[doc = "Field `NIS` writer - Normal interrupt summary"]
pub type NIS_W<'a, REG> = crate::BitWriter1C<'a, REG>;
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
#[doc = "Field `MMI` reader - MAC MMC interrupt"]
pub type MMI_R = crate::BitReader;
#[doc = "Field `MPI` reader - MAC PMT interrupt"]
pub type MPI_R = crate::BitReader;
#[doc = "Field `TTI` reader - Timestamp trigger interrupt"]
pub type TTI_R = crate::BitReader;
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
