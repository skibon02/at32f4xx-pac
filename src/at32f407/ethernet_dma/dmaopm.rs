#[doc = "Register `DMAOPM` reader"]
pub type R = crate::R<DMAOPM_SPEC>;
#[doc = "Register `DMAOPM` writer"]
pub type W = crate::W<DMAOPM_SPEC>;
#[doc = "Start or stop receive\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSR_A {
    #[doc = "0: Receive is stopped"]
    Stop = 0,
    #[doc = "1: Receive is started or resumed"]
    Start = 1,
}
impl From<SSR_A> for bool {
    #[inline(always)]
    fn from(variant: SSR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSR` reader - Start or stop receive"]
pub type SSR_R = crate::BitReader<SSR_A>;
impl SSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSR_A {
        match self.bits {
            false => SSR_A::Stop,
            true => SSR_A::Start,
        }
    }
    #[doc = "Receive is stopped"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == SSR_A::Stop
    }
    #[doc = "Receive is started or resumed"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SSR_A::Start
    }
}
#[doc = "Field `SSR` writer - Start or stop receive"]
pub type SSR_W<'a, REG> = crate::BitWriter<'a, REG, SSR_A>;
impl<'a, REG> SSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive is stopped"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(SSR_A::Stop)
    }
    #[doc = "Receive is started or resumed"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(SSR_A::Start)
    }
}
#[doc = "Operate on second frame\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSF_A {
    #[doc = "0: Operate on the first descriptor in the list of receive descriptors"]
    Normal = 0,
    #[doc = "1: Process second frame before status of first is obtained"]
    Next = 1,
}
impl From<OSF_A> for bool {
    #[inline(always)]
    fn from(variant: OSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSF` reader - Operate on second frame"]
pub type OSF_R = crate::BitReader<OSF_A>;
impl OSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSF_A {
        match self.bits {
            false => OSF_A::Normal,
            true => OSF_A::Next,
        }
    }
    #[doc = "Operate on the first descriptor in the list of receive descriptors"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == OSF_A::Normal
    }
    #[doc = "Process second frame before status of first is obtained"]
    #[inline(always)]
    pub fn is_next(&self) -> bool {
        *self == OSF_A::Next
    }
}
#[doc = "Field `OSF` writer - Operate on second frame"]
pub type OSF_W<'a, REG> = crate::BitWriter<'a, REG, OSF_A>;
impl<'a, REG> OSF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Operate on the first descriptor in the list of receive descriptors"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(OSF_A::Normal)
    }
    #[doc = "Process second frame before status of first is obtained"]
    #[inline(always)]
    pub fn next(self) -> &'a mut crate::W<REG> {
        self.variant(OSF_A::Next)
    }
}
#[doc = "Receive threshold control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTC_A {
    #[doc = "0: Threshold for short frames is 64 bytes"]
    Thr64 = 0,
    #[doc = "1: Threshold for short frames is 32 bytes"]
    Thr32 = 1,
    #[doc = "2: Threshold for short frames is 96 bytes"]
    Thr96 = 2,
    #[doc = "3: Threshold for short frames is 128 bytes"]
    Thr128 = 3,
}
impl From<RTC_A> for u8 {
    #[inline(always)]
    fn from(variant: RTC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RTC_A {
    type Ux = u8;
}
impl crate::IsEnum for RTC_A {}
#[doc = "Field `RTC` reader - Receive threshold control"]
pub type RTC_R = crate::FieldReader<RTC_A>;
impl RTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTC_A {
        match self.bits {
            0 => RTC_A::Thr64,
            1 => RTC_A::Thr32,
            2 => RTC_A::Thr96,
            3 => RTC_A::Thr128,
            _ => unreachable!(),
        }
    }
    #[doc = "Threshold for short frames is 64 bytes"]
    #[inline(always)]
    pub fn is_thr64(&self) -> bool {
        *self == RTC_A::Thr64
    }
    #[doc = "Threshold for short frames is 32 bytes"]
    #[inline(always)]
    pub fn is_thr32(&self) -> bool {
        *self == RTC_A::Thr32
    }
    #[doc = "Threshold for short frames is 96 bytes"]
    #[inline(always)]
    pub fn is_thr96(&self) -> bool {
        *self == RTC_A::Thr96
    }
    #[doc = "Threshold for short frames is 128 bytes"]
    #[inline(always)]
    pub fn is_thr128(&self) -> bool {
        *self == RTC_A::Thr128
    }
}
#[doc = "Field `RTC` writer - Receive threshold control"]
pub type RTC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RTC_A, crate::Safe>;
impl<'a, REG> RTC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Threshold for short frames is 64 bytes"]
    #[inline(always)]
    pub fn thr64(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_A::Thr64)
    }
    #[doc = "Threshold for short frames is 32 bytes"]
    #[inline(always)]
    pub fn thr32(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_A::Thr32)
    }
    #[doc = "Threshold for short frames is 96 bytes"]
    #[inline(always)]
    pub fn thr96(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_A::Thr96)
    }
    #[doc = "Threshold for short frames is 128 bytes"]
    #[inline(always)]
    pub fn thr128(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_A::Thr128)
    }
}
#[doc = "Forward undersized good frames\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FUGF_A {
    #[doc = "0: Drop frames less than 64 bytes"]
    Normal = 0,
    #[doc = "1: Forward undersized frames < 64 bytes if no error"]
    PassUndersized = 1,
}
impl From<FUGF_A> for bool {
    #[inline(always)]
    fn from(variant: FUGF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FUGF` reader - Forward undersized good frames"]
pub type FUGF_R = crate::BitReader<FUGF_A>;
impl FUGF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FUGF_A {
        match self.bits {
            false => FUGF_A::Normal,
            true => FUGF_A::PassUndersized,
        }
    }
    #[doc = "Drop frames less than 64 bytes"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == FUGF_A::Normal
    }
    #[doc = "Forward undersized frames < 64 bytes if no error"]
    #[inline(always)]
    pub fn is_pass_undersized(&self) -> bool {
        *self == FUGF_A::PassUndersized
    }
}
#[doc = "Field `FUGF` writer - Forward undersized good frames"]
pub type FUGF_W<'a, REG> = crate::BitWriter<'a, REG, FUGF_A>;
impl<'a, REG> FUGF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Drop frames less than 64 bytes"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(FUGF_A::Normal)
    }
    #[doc = "Forward undersized frames < 64 bytes if no error"]
    #[inline(always)]
    pub fn pass_undersized(self) -> &'a mut crate::W<REG> {
        self.variant(FUGF_A::PassUndersized)
    }
}
#[doc = "Forward error frames\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEF_A {
    #[doc = "0: Forward all frames except those with errors"]
    Normal = 0,
    #[doc = "1: Forward all frames including those with CRC/collision/watchdog/overflow errors"]
    PassError = 1,
}
impl From<FEF_A> for bool {
    #[inline(always)]
    fn from(variant: FEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEF` reader - Forward error frames"]
pub type FEF_R = crate::BitReader<FEF_A>;
impl FEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FEF_A {
        match self.bits {
            false => FEF_A::Normal,
            true => FEF_A::PassError,
        }
    }
    #[doc = "Forward all frames except those with errors"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == FEF_A::Normal
    }
    #[doc = "Forward all frames including those with CRC/collision/watchdog/overflow errors"]
    #[inline(always)]
    pub fn is_pass_error(&self) -> bool {
        *self == FEF_A::PassError
    }
}
#[doc = "Field `FEF` writer - Forward error frames"]
pub type FEF_W<'a, REG> = crate::BitWriter<'a, REG, FEF_A>;
impl<'a, REG> FEF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Forward all frames except those with errors"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(FEF_A::Normal)
    }
    #[doc = "Forward all frames including those with CRC/collision/watchdog/overflow errors"]
    #[inline(always)]
    pub fn pass_error(self) -> &'a mut crate::W<REG> {
        self.variant(FEF_A::PassError)
    }
}
#[doc = "Start of stop transmission command\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSTC_A {
    #[doc = "0: Transmit is stopped"]
    Stop = 0,
    #[doc = "1: Transmit is started or resumed"]
    Start = 1,
}
impl From<SSTC_A> for bool {
    #[inline(always)]
    fn from(variant: SSTC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSTC` reader - Start of stop transmission command"]
pub type SSTC_R = crate::BitReader<SSTC_A>;
impl SSTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSTC_A {
        match self.bits {
            false => SSTC_A::Stop,
            true => SSTC_A::Start,
        }
    }
    #[doc = "Transmit is stopped"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == SSTC_A::Stop
    }
    #[doc = "Transmit is started or resumed"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SSTC_A::Start
    }
}
#[doc = "Field `SSTC` writer - Start of stop transmission command"]
pub type SSTC_W<'a, REG> = crate::BitWriter<'a, REG, SSTC_A>;
impl<'a, REG> SSTC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit is stopped"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(SSTC_A::Stop)
    }
    #[doc = "Transmit is started or resumed"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(SSTC_A::Start)
    }
}
#[doc = "Transmit threshold control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TTC_A {
    #[doc = "0: Threshold for short frames is 64 bytes"]
    Thr64 = 0,
    #[doc = "1: Threshold for short frames is 128 bytes"]
    Thr128 = 1,
    #[doc = "2: Threshold for short frames is 192 bytes"]
    Thr192 = 2,
    #[doc = "3: Threshold for short frames is 256 bytes"]
    Thr256 = 3,
    #[doc = "4: Threshold for short frames is 40 bytes"]
    Thr40 = 4,
    #[doc = "5: Threshold for short frames is 32 bytes"]
    Thr32 = 5,
    #[doc = "6: Threshold for short frames is 24 bytes"]
    Thr24 = 6,
    #[doc = "7: Threshold for short frames is 16 bytes"]
    Thr16 = 7,
}
impl From<TTC_A> for u8 {
    #[inline(always)]
    fn from(variant: TTC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TTC_A {
    type Ux = u8;
}
impl crate::IsEnum for TTC_A {}
#[doc = "Field `TTC` reader - Transmit threshold control"]
pub type TTC_R = crate::FieldReader<TTC_A>;
impl TTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TTC_A {
        match self.bits {
            0 => TTC_A::Thr64,
            1 => TTC_A::Thr128,
            2 => TTC_A::Thr192,
            3 => TTC_A::Thr256,
            4 => TTC_A::Thr40,
            5 => TTC_A::Thr32,
            6 => TTC_A::Thr24,
            7 => TTC_A::Thr16,
            _ => unreachable!(),
        }
    }
    #[doc = "Threshold for short frames is 64 bytes"]
    #[inline(always)]
    pub fn is_thr64(&self) -> bool {
        *self == TTC_A::Thr64
    }
    #[doc = "Threshold for short frames is 128 bytes"]
    #[inline(always)]
    pub fn is_thr128(&self) -> bool {
        *self == TTC_A::Thr128
    }
    #[doc = "Threshold for short frames is 192 bytes"]
    #[inline(always)]
    pub fn is_thr192(&self) -> bool {
        *self == TTC_A::Thr192
    }
    #[doc = "Threshold for short frames is 256 bytes"]
    #[inline(always)]
    pub fn is_thr256(&self) -> bool {
        *self == TTC_A::Thr256
    }
    #[doc = "Threshold for short frames is 40 bytes"]
    #[inline(always)]
    pub fn is_thr40(&self) -> bool {
        *self == TTC_A::Thr40
    }
    #[doc = "Threshold for short frames is 32 bytes"]
    #[inline(always)]
    pub fn is_thr32(&self) -> bool {
        *self == TTC_A::Thr32
    }
    #[doc = "Threshold for short frames is 24 bytes"]
    #[inline(always)]
    pub fn is_thr24(&self) -> bool {
        *self == TTC_A::Thr24
    }
    #[doc = "Threshold for short frames is 16 bytes"]
    #[inline(always)]
    pub fn is_thr16(&self) -> bool {
        *self == TTC_A::Thr16
    }
}
#[doc = "Field `TTC` writer - Transmit threshold control"]
pub type TTC_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TTC_A, crate::Safe>;
impl<'a, REG> TTC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Threshold for short frames is 64 bytes"]
    #[inline(always)]
    pub fn thr64(self) -> &'a mut crate::W<REG> {
        self.variant(TTC_A::Thr64)
    }
    #[doc = "Threshold for short frames is 128 bytes"]
    #[inline(always)]
    pub fn thr128(self) -> &'a mut crate::W<REG> {
        self.variant(TTC_A::Thr128)
    }
    #[doc = "Threshold for short frames is 192 bytes"]
    #[inline(always)]
    pub fn thr192(self) -> &'a mut crate::W<REG> {
        self.variant(TTC_A::Thr192)
    }
    #[doc = "Threshold for short frames is 256 bytes"]
    #[inline(always)]
    pub fn thr256(self) -> &'a mut crate::W<REG> {
        self.variant(TTC_A::Thr256)
    }
    #[doc = "Threshold for short frames is 40 bytes"]
    #[inline(always)]
    pub fn thr40(self) -> &'a mut crate::W<REG> {
        self.variant(TTC_A::Thr40)
    }
    #[doc = "Threshold for short frames is 32 bytes"]
    #[inline(always)]
    pub fn thr32(self) -> &'a mut crate::W<REG> {
        self.variant(TTC_A::Thr32)
    }
    #[doc = "Threshold for short frames is 24 bytes"]
    #[inline(always)]
    pub fn thr24(self) -> &'a mut crate::W<REG> {
        self.variant(TTC_A::Thr24)
    }
    #[doc = "Threshold for short frames is 16 bytes"]
    #[inline(always)]
    pub fn thr16(self) -> &'a mut crate::W<REG> {
        self.variant(TTC_A::Thr16)
    }
}
#[doc = "Flush transmit FIFO\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTFW_A {
    #[doc = "1: Flush the transmit FIFO. This bit is automatically cleared by hardware when the flush operation is completed"]
    Flush = 1,
}
impl From<FTFW_A> for bool {
    #[inline(always)]
    fn from(variant: FTFW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTF` reader - Flush transmit FIFO"]
pub type FTF_R = crate::BitReader<FTFW_A>;
impl FTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FTFW_A> {
        match self.bits {
            true => Some(FTFW_A::Flush),
            _ => None,
        }
    }
    #[doc = "Flush the transmit FIFO. This bit is automatically cleared by hardware when the flush operation is completed"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == FTFW_A::Flush
    }
}
#[doc = "Field `FTF` writer - Flush transmit FIFO"]
pub type FTF_W<'a, REG> = crate::BitWriter1S<'a, REG, FTFW_A>;
impl<'a, REG> FTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flush the transmit FIFO. This bit is automatically cleared by hardware when the flush operation is completed"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(FTFW_A::Flush)
    }
}
#[doc = "Transmit store and forward\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSF_A {
    #[doc = "0: Transmit store and forward disabled"]
    Disable = 0,
    #[doc = "1: Transmit store and forward enabled. Transmission starts when a full frame resides in the Tx FIFO, and the TTC values specified in the bit \\[16: 14\\] are ignored"]
    Enable = 1,
}
impl From<TSF_A> for bool {
    #[inline(always)]
    fn from(variant: TSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSF` reader - Transmit store and forward"]
pub type TSF_R = crate::BitReader<TSF_A>;
impl TSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSF_A {
        match self.bits {
            false => TSF_A::Disable,
            true => TSF_A::Enable,
        }
    }
    #[doc = "Transmit store and forward disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TSF_A::Disable
    }
    #[doc = "Transmit store and forward enabled. Transmission starts when a full frame resides in the Tx FIFO, and the TTC values specified in the bit \\[16: 14\\] are ignored"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TSF_A::Enable
    }
}
#[doc = "Field `TSF` writer - Transmit store and forward"]
pub type TSF_W<'a, REG> = crate::BitWriter<'a, REG, TSF_A>;
impl<'a, REG> TSF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit store and forward disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TSF_A::Disable)
    }
    #[doc = "Transmit store and forward enabled. Transmission starts when a full frame resides in the Tx FIFO, and the TTC values specified in the bit \\[16: 14\\] are ignored"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TSF_A::Enable)
    }
}
#[doc = "Disable flushing of received frames\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFRF_A {
    #[doc = "0: Normal operation, flush RX DMA if descriptors are unavailable"]
    Normal = 0,
    #[doc = "1: Rx DMA does not flush any receive frame due to the unavailability of receive descriptors or receive buffers"]
    DontFlush = 1,
}
impl From<DFRF_A> for bool {
    #[inline(always)]
    fn from(variant: DFRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFRF` reader - Disable flushing of received frames"]
pub type DFRF_R = crate::BitReader<DFRF_A>;
impl DFRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DFRF_A {
        match self.bits {
            false => DFRF_A::Normal,
            true => DFRF_A::DontFlush,
        }
    }
    #[doc = "Normal operation, flush RX DMA if descriptors are unavailable"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == DFRF_A::Normal
    }
    #[doc = "Rx DMA does not flush any receive frame due to the unavailability of receive descriptors or receive buffers"]
    #[inline(always)]
    pub fn is_dont_flush(&self) -> bool {
        *self == DFRF_A::DontFlush
    }
}
#[doc = "Field `DFRF` writer - Disable flushing of received frames"]
pub type DFRF_W<'a, REG> = crate::BitWriter<'a, REG, DFRF_A>;
impl<'a, REG> DFRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation, flush RX DMA if descriptors are unavailable"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(DFRF_A::Normal)
    }
    #[doc = "Rx DMA does not flush any receive frame due to the unavailability of receive descriptors or receive buffers"]
    #[inline(always)]
    pub fn dont_flush(self) -> &'a mut crate::W<REG> {
        self.variant(DFRF_A::DontFlush)
    }
}
#[doc = "Receive store and forward\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSF_A {
    #[doc = "0: Receive store and forward disabled"]
    Disable = 0,
    #[doc = "1: Receive store and forward enabled. The MTL reads the Rx FIFO only after a full frame is written to the Rx FIFO, ignoring the RTC bit"]
    Enable = 1,
}
impl From<RSF_A> for bool {
    #[inline(always)]
    fn from(variant: RSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSF` reader - Receive store and forward"]
pub type RSF_R = crate::BitReader<RSF_A>;
impl RSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RSF_A {
        match self.bits {
            false => RSF_A::Disable,
            true => RSF_A::Enable,
        }
    }
    #[doc = "Receive store and forward disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RSF_A::Disable
    }
    #[doc = "Receive store and forward enabled. The MTL reads the Rx FIFO only after a full frame is written to the Rx FIFO, ignoring the RTC bit"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RSF_A::Enable
    }
}
#[doc = "Field `RSF` writer - Receive store and forward"]
pub type RSF_W<'a, REG> = crate::BitWriter<'a, REG, RSF_A>;
impl<'a, REG> RSF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive store and forward disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RSF_A::Disable)
    }
    #[doc = "Receive store and forward enabled. The MTL reads the Rx FIFO only after a full frame is written to the Rx FIFO, ignoring the RTC bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RSF_A::Enable)
    }
}
#[doc = "Disable dropping of TCP/IP checksum error frames\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DT_A {
    #[doc = "0: Normal operation"]
    Normal = 0,
    #[doc = "1: Don't drop frames with checksum errors in encapsulated payload. However ethernet checksum (FCS) must be valid"]
    DontDropErrorFrames = 1,
}
impl From<DT_A> for bool {
    #[inline(always)]
    fn from(variant: DT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DT` reader - Disable dropping of TCP/IP checksum error frames"]
pub type DT_R = crate::BitReader<DT_A>;
impl DT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DT_A {
        match self.bits {
            false => DT_A::Normal,
            true => DT_A::DontDropErrorFrames,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == DT_A::Normal
    }
    #[doc = "Don't drop frames with checksum errors in encapsulated payload. However ethernet checksum (FCS) must be valid"]
    #[inline(always)]
    pub fn is_dont_drop_error_frames(&self) -> bool {
        *self == DT_A::DontDropErrorFrames
    }
}
#[doc = "Field `DT` writer - Disable dropping of TCP/IP checksum error frames"]
pub type DT_W<'a, REG> = crate::BitWriter<'a, REG, DT_A>;
impl<'a, REG> DT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(DT_A::Normal)
    }
    #[doc = "Don't drop frames with checksum errors in encapsulated payload. However ethernet checksum (FCS) must be valid"]
    #[inline(always)]
    pub fn dont_drop_error_frames(self) -> &'a mut crate::W<REG> {
        self.variant(DT_A::DontDropErrorFrames)
    }
}
impl R {
    #[doc = "Bit 1 - Start or stop receive"]
    #[inline(always)]
    pub fn ssr(&self) -> SSR_R {
        SSR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Operate on second frame"]
    #[inline(always)]
    pub fn osf(&self) -> OSF_R {
        OSF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Receive threshold control"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 6 - Forward undersized good frames"]
    #[inline(always)]
    pub fn fugf(&self) -> FUGF_R {
        FUGF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Forward error frames"]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - Start of stop transmission command"]
    #[inline(always)]
    pub fn sstc(&self) -> SSTC_R {
        SSTC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:16 - Transmit threshold control"]
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 20 - Flush transmit FIFO"]
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit store and forward"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Disable flushing of received frames"]
    #[inline(always)]
    pub fn dfrf(&self) -> DFRF_R {
        DFRF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Receive store and forward"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Disable dropping of TCP/IP checksum error frames"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAOPM")
            .field("ssr", &self.ssr())
            .field("osf", &self.osf())
            .field("rtc", &self.rtc())
            .field("fugf", &self.fugf())
            .field("fef", &self.fef())
            .field("sstc", &self.sstc())
            .field("ttc", &self.ttc())
            .field("ftf", &self.ftf())
            .field("tsf", &self.tsf())
            .field("dfrf", &self.dfrf())
            .field("rsf", &self.rsf())
            .field("dt", &self.dt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Start or stop receive"]
    #[inline(always)]
    pub fn ssr(&mut self) -> SSR_W<'_, DMAOPM_SPEC> {
        SSR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Operate on second frame"]
    #[inline(always)]
    pub fn osf(&mut self) -> OSF_W<'_, DMAOPM_SPEC> {
        OSF_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - Receive threshold control"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W<'_, DMAOPM_SPEC> {
        RTC_W::new(self, 3)
    }
    #[doc = "Bit 6 - Forward undersized good frames"]
    #[inline(always)]
    pub fn fugf(&mut self) -> FUGF_W<'_, DMAOPM_SPEC> {
        FUGF_W::new(self, 6)
    }
    #[doc = "Bit 7 - Forward error frames"]
    #[inline(always)]
    pub fn fef(&mut self) -> FEF_W<'_, DMAOPM_SPEC> {
        FEF_W::new(self, 7)
    }
    #[doc = "Bit 13 - Start of stop transmission command"]
    #[inline(always)]
    pub fn sstc(&mut self) -> SSTC_W<'_, DMAOPM_SPEC> {
        SSTC_W::new(self, 13)
    }
    #[doc = "Bits 14:16 - Transmit threshold control"]
    #[inline(always)]
    pub fn ttc(&mut self) -> TTC_W<'_, DMAOPM_SPEC> {
        TTC_W::new(self, 14)
    }
    #[doc = "Bit 20 - Flush transmit FIFO"]
    #[inline(always)]
    pub fn ftf(&mut self) -> FTF_W<'_, DMAOPM_SPEC> {
        FTF_W::new(self, 20)
    }
    #[doc = "Bit 21 - Transmit store and forward"]
    #[inline(always)]
    pub fn tsf(&mut self) -> TSF_W<'_, DMAOPM_SPEC> {
        TSF_W::new(self, 21)
    }
    #[doc = "Bit 24 - Disable flushing of received frames"]
    #[inline(always)]
    pub fn dfrf(&mut self) -> DFRF_W<'_, DMAOPM_SPEC> {
        DFRF_W::new(self, 24)
    }
    #[doc = "Bit 25 - Receive store and forward"]
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W<'_, DMAOPM_SPEC> {
        RSF_W::new(self, 25)
    }
    #[doc = "Bit 26 - Disable dropping of TCP/IP checksum error frames"]
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W<'_, DMAOPM_SPEC> {
        DT_W::new(self, 26)
    }
}
#[doc = "Ethernet DMA operation mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaopm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaopm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAOPM_SPEC;
impl crate::RegisterSpec for DMAOPM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaopm::R`](R) reader structure"]
impl crate::Readable for DMAOPM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmaopm::W`](W) writer structure"]
impl crate::Writable for DMAOPM_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0010_0000;
}
#[doc = "`reset()` method sets DMAOPM to value 0"]
impl crate::Resettable for DMAOPM_SPEC {}
