#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Channel %s global event flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GF1_A {
    #[doc = "0: No event"]
    NoEvent = 0,
    #[doc = "1: An event occurred (transfer error, half transfer or transfer complete)"]
    Event = 1,
}
impl From<GF1_A> for bool {
    #[inline(always)]
    fn from(variant: GF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GF(1-5)` reader - Channel %s global event flag"]
pub type GF_R = crate::BitReader<GF1_A>;
impl GF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GF1_A {
        match self.bits {
            false => GF1_A::NoEvent,
            true => GF1_A::Event,
        }
    }
    #[doc = "No event"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == GF1_A::NoEvent
    }
    #[doc = "An event occurred (transfer error, half transfer or transfer complete)"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == GF1_A::Event
    }
}
#[doc = "Channel %s data transfer complete event flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FDTF1_A {
    #[doc = "0: No transfer complete"]
    NoTransferComplete = 0,
    #[doc = "1: A transfer complete occurred"]
    TransferComplete = 1,
}
impl From<FDTF1_A> for bool {
    #[inline(always)]
    fn from(variant: FDTF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDTF(1-5)` reader - Channel %s data transfer complete event flag"]
pub type FDTF_R = crate::BitReader<FDTF1_A>;
impl FDTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FDTF1_A {
        match self.bits {
            false => FDTF1_A::NoTransferComplete,
            true => FDTF1_A::TransferComplete,
        }
    }
    #[doc = "No transfer complete"]
    #[inline(always)]
    pub fn is_no_transfer_complete(&self) -> bool {
        *self == FDTF1_A::NoTransferComplete
    }
    #[doc = "A transfer complete occurred"]
    #[inline(always)]
    pub fn is_transfer_complete(&self) -> bool {
        *self == FDTF1_A::TransferComplete
    }
}
#[doc = "Channel %s half transfer event flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDTF1_A {
    #[doc = "0: No half transfer"]
    NoHalfTransfer = 0,
    #[doc = "1: A half transfer occurred"]
    HalfTransfer = 1,
}
impl From<HDTF1_A> for bool {
    #[inline(always)]
    fn from(variant: HDTF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDTF(1-5)` reader - Channel %s half transfer event flag"]
pub type HDTF_R = crate::BitReader<HDTF1_A>;
impl HDTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HDTF1_A {
        match self.bits {
            false => HDTF1_A::NoHalfTransfer,
            true => HDTF1_A::HalfTransfer,
        }
    }
    #[doc = "No half transfer"]
    #[inline(always)]
    pub fn is_no_half_transfer(&self) -> bool {
        *self == HDTF1_A::NoHalfTransfer
    }
    #[doc = "A half transfer occurred"]
    #[inline(always)]
    pub fn is_half_transfer(&self) -> bool {
        *self == HDTF1_A::HalfTransfer
    }
}
#[doc = "Channel %s data transfer error event flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTERRF1_A {
    #[doc = "0: No transfer error"]
    NoTransferError = 0,
    #[doc = "1: A transfer error occurred"]
    TransferError = 1,
}
impl From<DTERRF1_A> for bool {
    #[inline(always)]
    fn from(variant: DTERRF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTERRF(1-5)` reader - Channel %s data transfer error event flag"]
pub type DTERRF_R = crate::BitReader<DTERRF1_A>;
impl DTERRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTERRF1_A {
        match self.bits {
            false => DTERRF1_A::NoTransferError,
            true => DTERRF1_A::TransferError,
        }
    }
    #[doc = "No transfer error"]
    #[inline(always)]
    pub fn is_no_transfer_error(&self) -> bool {
        *self == DTERRF1_A::NoTransferError
    }
    #[doc = "A transfer error occurred"]
    #[inline(always)]
    pub fn is_transfer_error(&self) -> bool {
        *self == DTERRF1_A::TransferError
    }
}
impl R {
    #[doc = "Channel (1-5) global event flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GF1` field.</div>"]
    #[inline(always)]
    pub fn gf(&self, n: u8) -> GF_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        GF_R::new(((self.bits >> (n * 4)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-5) global event flag"]
    #[inline(always)]
    pub fn gf_iter(&self) -> impl Iterator<Item = GF_R> + '_ {
        (0..5).map(move |n| GF_R::new(((self.bits >> (n * 4)) & 1) != 0))
    }
    #[doc = "Bit 0 - Channel 1 global event flag"]
    #[inline(always)]
    pub fn gf1(&self) -> GF_R {
        GF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 2 global event flag"]
    #[inline(always)]
    pub fn gf2(&self) -> GF_R {
        GF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 3 global event flag"]
    #[inline(always)]
    pub fn gf3(&self) -> GF_R {
        GF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 4 global event flag"]
    #[inline(always)]
    pub fn gf4(&self) -> GF_R {
        GF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 5 global event flag"]
    #[inline(always)]
    pub fn gf5(&self) -> GF_R {
        GF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Channel (1-5) data transfer complete event flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `FDTF1` field.</div>"]
    #[inline(always)]
    pub fn fdtf(&self, n: u8) -> FDTF_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        FDTF_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-5) data transfer complete event flag"]
    #[inline(always)]
    pub fn fdtf_iter(&self) -> impl Iterator<Item = FDTF_R> + '_ {
        (0..5).map(move |n| FDTF_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - Channel 1 data transfer complete event flag"]
    #[inline(always)]
    pub fn fdtf1(&self) -> FDTF_R {
        FDTF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 2 data transfer complete event flag"]
    #[inline(always)]
    pub fn fdtf2(&self) -> FDTF_R {
        FDTF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 3 data transfer complete event flag"]
    #[inline(always)]
    pub fn fdtf3(&self) -> FDTF_R {
        FDTF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 4 data transfer complete event flag"]
    #[inline(always)]
    pub fn fdtf4(&self) -> FDTF_R {
        FDTF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel 5 data transfer complete event flag"]
    #[inline(always)]
    pub fn fdtf5(&self) -> FDTF_R {
        FDTF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Channel (1-5) half transfer event flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `HDTF1` field.</div>"]
    #[inline(always)]
    pub fn hdtf(&self, n: u8) -> HDTF_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        HDTF_R::new(((self.bits >> (n * 4 + 2)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-5) half transfer event flag"]
    #[inline(always)]
    pub fn hdtf_iter(&self) -> impl Iterator<Item = HDTF_R> + '_ {
        (0..5).map(move |n| HDTF_R::new(((self.bits >> (n * 4 + 2)) & 1) != 0))
    }
    #[doc = "Bit 2 - Channel 1 half transfer event flag"]
    #[inline(always)]
    pub fn hdtf1(&self) -> HDTF_R {
        HDTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 2 half transfer event flag"]
    #[inline(always)]
    pub fn hdtf2(&self) -> HDTF_R {
        HDTF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 3 half transfer event flag"]
    #[inline(always)]
    pub fn hdtf3(&self) -> HDTF_R {
        HDTF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 4 half transfer event flag"]
    #[inline(always)]
    pub fn hdtf4(&self) -> HDTF_R {
        HDTF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel 5 half transfer event flag"]
    #[inline(always)]
    pub fn hdtf5(&self) -> HDTF_R {
        HDTF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Channel (1-5) data transfer error event flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DTERRF1` field.</div>"]
    #[inline(always)]
    pub fn dterrf(&self, n: u8) -> DTERRF_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        DTERRF_R::new(((self.bits >> (n * 4 + 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-5) data transfer error event flag"]
    #[inline(always)]
    pub fn dterrf_iter(&self) -> impl Iterator<Item = DTERRF_R> + '_ {
        (0..5).map(move |n| DTERRF_R::new(((self.bits >> (n * 4 + 3)) & 1) != 0))
    }
    #[doc = "Bit 3 - Channel 1 data transfer error event flag"]
    #[inline(always)]
    pub fn dterrf1(&self) -> DTERRF_R {
        DTERRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 2 data transfer error event flag"]
    #[inline(always)]
    pub fn dterrf2(&self) -> DTERRF_R {
        DTERRF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 data transfer error event flag"]
    #[inline(always)]
    pub fn dterrf3(&self) -> DTERRF_R {
        DTERRF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 4 data transfer error event flag"]
    #[inline(always)]
    pub fn dterrf4(&self) -> DTERRF_R {
        DTERRF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel 5 data transfer error event flag"]
    #[inline(always)]
    pub fn dterrf5(&self) -> DTERRF_R {
        DTERRF_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("gf1", &self.gf1())
            .field("gf2", &self.gf2())
            .field("gf3", &self.gf3())
            .field("gf4", &self.gf4())
            .field("gf5", &self.gf5())
            .field("fdtf1", &self.fdtf1())
            .field("fdtf2", &self.fdtf2())
            .field("fdtf3", &self.fdtf3())
            .field("fdtf4", &self.fdtf4())
            .field("fdtf5", &self.fdtf5())
            .field("hdtf1", &self.hdtf1())
            .field("hdtf2", &self.hdtf2())
            .field("hdtf3", &self.hdtf3())
            .field("hdtf4", &self.hdtf4())
            .field("hdtf5", &self.hdtf5())
            .field("dterrf1", &self.dterrf1())
            .field("dterrf2", &self.dterrf2())
            .field("dterrf3", &self.dterrf3())
            .field("dterrf4", &self.dterrf4())
            .field("dterrf5", &self.dterrf5())
            .finish()
    }
}
#[doc = "DMA status register (DMA_STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {}
