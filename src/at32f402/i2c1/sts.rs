#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<STS_SPEC>;
#[doc = "Transmit data buffer empty flag\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdber {
    #[doc = "0: Data register not empty"]
    NotEmpty = 0,
    #[doc = "1: Data register empty"]
    Empty = 1,
}
impl From<Tdber> for bool {
    #[inline(always)]
    fn from(variant: Tdber) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDBE` reader - Transmit data buffer empty flag"]
pub type TDBE_R = crate::BitReader<Tdber>;
impl TDBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tdber {
        match self.bits {
            false => Tdber::NotEmpty,
            true => Tdber::Empty,
        }
    }
    #[doc = "Data register not empty"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == Tdber::NotEmpty
    }
    #[doc = "Data register empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Tdber::Empty
    }
}
#[doc = "Transmit data buffer empty flag\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TdbewWO {
    #[doc = "1: Write 1 to clear TDBE flag when clock stretching is disabled."]
    Clear = 1,
}
impl From<TdbewWO> for bool {
    #[inline(always)]
    fn from(variant: TdbewWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDBE` writer - Transmit data buffer empty flag"]
pub type TDBE_W<'a, REG> = crate::BitWriter1C<'a, REG, TdbewWO>;
impl<'a, REG> TDBE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write 1 to clear TDBE flag when clock stretching is disabled."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TdbewWO::Clear)
    }
}
#[doc = "Send interrupt status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdisr {
    #[doc = "0: Data has been written to the I2C_TXDT"]
    NotEmpty = 0,
    #[doc = "1: Data has been sent from the I2C_TXDT to the shift register. I2C_TXDT become empty, and thus the to-be- transferred data must be written to the I2C_TXDT"]
    Empty = 1,
}
impl From<Tdisr> for bool {
    #[inline(always)]
    fn from(variant: Tdisr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDIS` reader - Send interrupt status"]
pub type TDIS_R = crate::BitReader<Tdisr>;
impl TDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tdisr {
        match self.bits {
            false => Tdisr::NotEmpty,
            true => Tdisr::Empty,
        }
    }
    #[doc = "Data has been written to the I2C_TXDT"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == Tdisr::NotEmpty
    }
    #[doc = "Data has been sent from the I2C_TXDT to the shift register. I2C_TXDT become empty, and thus the to-be- transferred data must be written to the I2C_TXDT"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Tdisr::Empty
    }
}
#[doc = "Send interrupt status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TdiswWO {
    #[doc = "1: Write 1 to generate TDIS event when clock stretching is disabled."]
    Tdis = 1,
}
impl From<TdiswWO> for bool {
    #[inline(always)]
    fn from(variant: TdiswWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDIS` writer - Send interrupt status"]
pub type TDIS_W<'a, REG> = crate::BitWriter1S<'a, REG, TdiswWO>;
impl<'a, REG> TDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write 1 to generate TDIS event when clock stretching is disabled."]
    #[inline(always)]
    pub fn tdis(self) -> &'a mut crate::W<REG> {
        self.variant(TdiswWO::Tdis)
    }
}
#[doc = "Receive data buffer full flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDBF_A {
    #[doc = "0: Data register empty"]
    Empty = 0,
    #[doc = "1: Data register not empty"]
    HasData = 1,
}
impl From<RDBF_A> for bool {
    #[inline(always)]
    fn from(variant: RDBF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDBF` reader - Receive data buffer full flag"]
pub type RDBF_R = crate::BitReader<RDBF_A>;
impl RDBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDBF_A {
        match self.bits {
            false => RDBF_A::Empty,
            true => RDBF_A::HasData,
        }
    }
    #[doc = "Data register empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RDBF_A::Empty
    }
    #[doc = "Data register not empty"]
    #[inline(always)]
    pub fn is_has_data(&self) -> bool {
        *self == RDBF_A::HasData
    }
}
#[doc = "0~7 bit address match flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRF_A {
    #[doc = "0: No address matched"]
    NoMatch = 0,
    #[doc = "1: Address matched"]
    Matched = 1,
}
impl From<ADDRF_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRF` reader - 0~7 bit address match flag"]
pub type ADDRF_R = crate::BitReader<ADDRF_A>;
impl ADDRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADDRF_A {
        match self.bits {
            false => ADDRF_A::NoMatch,
            true => ADDRF_A::Matched,
        }
    }
    #[doc = "No address matched"]
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        *self == ADDRF_A::NoMatch
    }
    #[doc = "Address matched"]
    #[inline(always)]
    pub fn is_matched(&self) -> bool {
        *self == ADDRF_A::Matched
    }
}
#[doc = "Acknowledge failure flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACKFAIL_A {
    #[doc = "0: No acknowledge failure"]
    NoFailure = 0,
    #[doc = "1: Acknowledge failure"]
    Failure = 1,
}
impl From<ACKFAIL_A> for bool {
    #[inline(always)]
    fn from(variant: ACKFAIL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKFAIL` reader - Acknowledge failure flag"]
pub type ACKFAIL_R = crate::BitReader<ACKFAIL_A>;
impl ACKFAIL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ACKFAIL_A {
        match self.bits {
            false => ACKFAIL_A::NoFailure,
            true => ACKFAIL_A::Failure,
        }
    }
    #[doc = "No acknowledge failure"]
    #[inline(always)]
    pub fn is_no_failure(&self) -> bool {
        *self == ACKFAIL_A::NoFailure
    }
    #[doc = "Acknowledge failure"]
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        *self == ACKFAIL_A::Failure
    }
}
#[doc = "Stop condition generation complete flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPF_A {
    #[doc = "0: No Stop condition detected"]
    NoStop = 0,
    #[doc = "1: Stop condition detected"]
    Stop = 1,
}
impl From<STOPF_A> for bool {
    #[inline(always)]
    fn from(variant: STOPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPF` reader - Stop condition generation complete flag"]
pub type STOPF_R = crate::BitReader<STOPF_A>;
impl STOPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOPF_A {
        match self.bits {
            false => STOPF_A::NoStop,
            true => STOPF_A::Stop,
        }
    }
    #[doc = "No Stop condition detected"]
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == STOPF_A::NoStop
    }
    #[doc = "Stop condition detected"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STOPF_A::Stop
    }
}
#[doc = "Transmit data complete flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDC_A {
    #[doc = "0: Data transfer not complete yet"]
    NotComplete = 0,
    #[doc = "1: Data transfer complete, data loaded"]
    Complete = 1,
}
impl From<TDC_A> for bool {
    #[inline(always)]
    fn from(variant: TDC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDC` reader - Transmit data complete flag"]
pub type TDC_R = crate::BitReader<TDC_A>;
impl TDC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TDC_A {
        match self.bits {
            false => TDC_A::NotComplete,
            true => TDC_A::Complete,
        }
    }
    #[doc = "Data transfer not complete yet"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TDC_A::NotComplete
    }
    #[doc = "Data transfer complete, data loaded"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TDC_A::Complete
    }
}
#[doc = "Transmission is complete, waiting to load data\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCRLD_A {
    #[doc = "0: Data transfer not complete yet"]
    NotComplete = 0,
    #[doc = "1: Data transfer complete, waiting for data load"]
    Complete = 1,
}
impl From<TCRLD_A> for bool {
    #[inline(always)]
    fn from(variant: TCRLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCRLD` reader - Transmission is complete, waiting to load data"]
pub type TCRLD_R = crate::BitReader<TCRLD_A>;
impl TCRLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCRLD_A {
        match self.bits {
            false => TCRLD_A::NotComplete,
            true => TCRLD_A::Complete,
        }
    }
    #[doc = "Data transfer not complete yet"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TCRLD_A::NotComplete
    }
    #[doc = "Data transfer complete, waiting for data load"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TCRLD_A::Complete
    }
}
#[doc = "Bus error flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSERR_A {
    #[doc = "0: No bus error detected"]
    NoError = 0,
    #[doc = "1: Bus error detected"]
    Error = 1,
}
impl From<BUSERR_A> for bool {
    #[inline(always)]
    fn from(variant: BUSERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSERR` reader - Bus error flag"]
pub type BUSERR_R = crate::BitReader<BUSERR_A>;
impl BUSERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUSERR_A {
        match self.bits {
            false => BUSERR_A::NoError,
            true => BUSERR_A::Error,
        }
    }
    #[doc = "No bus error detected"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == BUSERR_A::NoError
    }
    #[doc = "Bus error detected"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == BUSERR_A::Error
    }
}
#[doc = "Arbitration lost flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARLOST_A {
    #[doc = "0: No arbitration lost detected"]
    NoLost = 0,
    #[doc = "1: Arbitration lost detected"]
    Lost = 1,
}
impl From<ARLOST_A> for bool {
    #[inline(always)]
    fn from(variant: ARLOST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARLOST` reader - Arbitration lost flag"]
pub type ARLOST_R = crate::BitReader<ARLOST_A>;
impl ARLOST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ARLOST_A {
        match self.bits {
            false => ARLOST_A::NoLost,
            true => ARLOST_A::Lost,
        }
    }
    #[doc = "No arbitration lost detected"]
    #[inline(always)]
    pub fn is_no_lost(&self) -> bool {
        *self == ARLOST_A::NoLost
    }
    #[doc = "Arbitration lost detected"]
    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        *self == ARLOST_A::Lost
    }
}
#[doc = "Overflow or underflow flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUF_A {
    #[doc = "0: No overrun/underrun occurred"]
    NoOverrun = 0,
    #[doc = "1: Overrun/underrun occurred"]
    Overrun = 1,
}
impl From<OUF_A> for bool {
    #[inline(always)]
    fn from(variant: OUF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUF` reader - Overflow or underflow flag"]
pub type OUF_R = crate::BitReader<OUF_A>;
impl OUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OUF_A {
        match self.bits {
            false => OUF_A::NoOverrun,
            true => OUF_A::Overrun,
        }
    }
    #[doc = "No overrun/underrun occurred"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OUF_A::NoOverrun
    }
    #[doc = "Overrun/underrun occurred"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OUF_A::Overrun
    }
}
#[doc = "PEC receive error flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECERR_A {
    #[doc = "0: PEC error in reception or transmission"]
    NoError = 0,
    #[doc = "1: PEC error in reception or transmission"]
    Error = 1,
}
impl From<PECERR_A> for bool {
    #[inline(always)]
    fn from(variant: PECERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECERR` reader - PEC receive error flag"]
pub type PECERR_R = crate::BitReader<PECERR_A>;
impl PECERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PECERR_A {
        match self.bits {
            false => PECERR_A::NoError,
            true => PECERR_A::Error,
        }
    }
    #[doc = "PEC error in reception or transmission"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PECERR_A::NoError
    }
    #[doc = "PEC error in reception or transmission"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PECERR_A::Error
    }
}
#[doc = "SMBus timeout flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMOUT_A {
    #[doc = "0: No SMBus timeout occurred"]
    NoTimeout = 0,
    #[doc = "1: SMBus timeout occurred"]
    Timeout = 1,
}
impl From<TMOUT_A> for bool {
    #[inline(always)]
    fn from(variant: TMOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMOUT` reader - SMBus timeout flag"]
pub type TMOUT_R = crate::BitReader<TMOUT_A>;
impl TMOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TMOUT_A {
        match self.bits {
            false => TMOUT_A::NoTimeout,
            true => TMOUT_A::Timeout,
        }
    }
    #[doc = "No SMBus timeout occurred"]
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == TMOUT_A::NoTimeout
    }
    #[doc = "SMBus timeout occurred"]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == TMOUT_A::Timeout
    }
}
#[doc = "SMBus alert flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALERTF_A {
    #[doc = "0: No SMBus alert"]
    NoAlert = 0,
    #[doc = "1: SMBus alert occurred"]
    Alert = 1,
}
impl From<ALERTF_A> for bool {
    #[inline(always)]
    fn from(variant: ALERTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALERTF` reader - SMBus alert flag"]
pub type ALERTF_R = crate::BitReader<ALERTF_A>;
impl ALERTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALERTF_A {
        match self.bits {
            false => ALERTF_A::NoAlert,
            true => ALERTF_A::Alert,
        }
    }
    #[doc = "No SMBus alert"]
    #[inline(always)]
    pub fn is_no_alert(&self) -> bool {
        *self == ALERTF_A::NoAlert
    }
    #[doc = "SMBus alert occurred"]
    #[inline(always)]
    pub fn is_alert(&self) -> bool {
        *self == ALERTF_A::Alert
    }
}
#[doc = "Bus busy\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSYF_A {
    #[doc = "0: I2C bus is idle"]
    Idle = 0,
    #[doc = "1: I2C bus is busy. Start condition was set"]
    Busy = 1,
}
impl From<BUSYF_A> for bool {
    #[inline(always)]
    fn from(variant: BUSYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSYF` reader - Bus busy"]
pub type BUSYF_R = crate::BitReader<BUSYF_A>;
impl BUSYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUSYF_A {
        match self.bits {
            false => BUSYF_A::Idle,
            true => BUSYF_A::Busy,
        }
    }
    #[doc = "I2C bus is idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSYF_A::Idle
    }
    #[doc = "I2C bus is busy. Start condition was set"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSYF_A::Busy
    }
}
#[doc = "Slave data transmit direction\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIR_A {
    #[doc = "0: Data byte received from slave"]
    Receive = 0,
    #[doc = "1: Data byte transmitted to slave"]
    Transmit = 1,
}
impl From<SDIR_A> for bool {
    #[inline(always)]
    fn from(variant: SDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIR` reader - Slave data transmit direction"]
pub type SDIR_R = crate::BitReader<SDIR_A>;
impl SDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDIR_A {
        match self.bits {
            false => SDIR_A::Receive,
            true => SDIR_A::Transmit,
        }
    }
    #[doc = "Data byte received from slave"]
    #[inline(always)]
    pub fn is_receive(&self) -> bool {
        *self == SDIR_A::Receive
    }
    #[doc = "Data byte transmitted to slave"]
    #[inline(always)]
    pub fn is_transmit(&self) -> bool {
        *self == SDIR_A::Transmit
    }
}
#[doc = "Field `ADDR` reader - Slave address matching value"]
pub type ADDR_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Transmit data buffer empty flag"]
    #[inline(always)]
    pub fn tdbe(&self) -> TDBE_R {
        TDBE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Send interrupt status"]
    #[inline(always)]
    pub fn tdis(&self) -> TDIS_R {
        TDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive data buffer full flag"]
    #[inline(always)]
    pub fn rdbf(&self) -> RDBF_R {
        RDBF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0~7 bit address match flag"]
    #[inline(always)]
    pub fn addrf(&self) -> ADDRF_R {
        ADDRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Acknowledge failure flag"]
    #[inline(always)]
    pub fn ackfail(&self) -> ACKFAIL_R {
        ACKFAIL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop condition generation complete flag"]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit data complete flag"]
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmission is complete, waiting to load data"]
    #[inline(always)]
    pub fn tcrld(&self) -> TCRLD_R {
        TCRLD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bus error flag"]
    #[inline(always)]
    pub fn buserr(&self) -> BUSERR_R {
        BUSERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration lost flag"]
    #[inline(always)]
    pub fn arlost(&self) -> ARLOST_R {
        ARLOST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Overflow or underflow flag"]
    #[inline(always)]
    pub fn ouf(&self) -> OUF_R {
        OUF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PEC receive error flag"]
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SMBus timeout flag"]
    #[inline(always)]
    pub fn tmout(&self) -> TMOUT_R {
        TMOUT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SMBus alert flag"]
    #[inline(always)]
    pub fn alertf(&self) -> ALERTF_R {
        ALERTF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Bus busy"]
    #[inline(always)]
    pub fn busyf(&self) -> BUSYF_R {
        BUSYF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave data transmit direction"]
    #[inline(always)]
    pub fn sdir(&self) -> SDIR_R {
        SDIR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - Slave address matching value"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("addr", &self.addr())
            .field("sdir", &self.sdir())
            .field("busyf", &self.busyf())
            .field("alertf", &self.alertf())
            .field("tmout", &self.tmout())
            .field("pecerr", &self.pecerr())
            .field("ouf", &self.ouf())
            .field("arlost", &self.arlost())
            .field("buserr", &self.buserr())
            .field("tcrld", &self.tcrld())
            .field("tdc", &self.tdc())
            .field("stopf", &self.stopf())
            .field("ackfail", &self.ackfail())
            .field("addrf", &self.addrf())
            .field("rdbf", &self.rdbf())
            .field("tdis", &self.tdis())
            .field("tdbe", &self.tdbe())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transmit data buffer empty flag"]
    #[inline(always)]
    pub fn tdbe(&mut self) -> TDBE_W<'_, STS_SPEC> {
        TDBE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Send interrupt status"]
    #[inline(always)]
    pub fn tdis(&mut self) -> TDIS_W<'_, STS_SPEC> {
        TDIS_W::new(self, 1)
    }
}
#[doc = "Interrupt and Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for STS_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x03;
}
#[doc = "`reset()` method sets STS to value 0x01"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
