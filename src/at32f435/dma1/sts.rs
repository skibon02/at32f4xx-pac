#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Field `GF(1-7)` reader - Channel %s global event flag"]
pub type GF_R = crate::BitReader;
#[doc = "Field `FDTF(1-7)` reader - Channel %s data transfer complete event flag"]
pub type FDTF_R = crate::BitReader;
#[doc = "Field `HDTF(1-7)` reader - Channel %s half transfer event flag"]
pub type HDTF_R = crate::BitReader;
#[doc = "Field `DTERRF(1-7)` reader - Channel %s data transfer error event flag"]
pub type DTERRF_R = crate::BitReader;
impl R {
    #[doc = "Channel (1-7) global event flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GF1` field.</div>"]
    #[inline(always)]
    pub fn gf(&self, n: u8) -> GF_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        GF_R::new(((self.bits >> (n * 4)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-7) global event flag"]
    #[inline(always)]
    pub fn gf_iter(&self) -> impl Iterator<Item = GF_R> + '_ {
        (0..7).map(move |n| GF_R::new(((self.bits >> (n * 4)) & 1) != 0))
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
    #[doc = "Bit 20 - Channel 6 global event flag"]
    #[inline(always)]
    pub fn gf6(&self) -> GF_R {
        GF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Channel 7 global event flag"]
    #[inline(always)]
    pub fn gf7(&self) -> GF_R {
        GF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Channel (1-7) data transfer complete event flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `FDTF1` field.</div>"]
    #[inline(always)]
    pub fn fdtf(&self, n: u8) -> FDTF_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        FDTF_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-7) data transfer complete event flag"]
    #[inline(always)]
    pub fn fdtf_iter(&self) -> impl Iterator<Item = FDTF_R> + '_ {
        (0..7).map(move |n| FDTF_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0))
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
    #[doc = "Bit 21 - Channel 6 data transfer complete event flag"]
    #[inline(always)]
    pub fn fdtf6(&self) -> FDTF_R {
        FDTF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 25 - Channel 7 data transfer complete event flag"]
    #[inline(always)]
    pub fn fdtf7(&self) -> FDTF_R {
        FDTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Channel (1-7) half transfer event flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `HDTF1` field.</div>"]
    #[inline(always)]
    pub fn hdtf(&self, n: u8) -> HDTF_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        HDTF_R::new(((self.bits >> (n * 4 + 2)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-7) half transfer event flag"]
    #[inline(always)]
    pub fn hdtf_iter(&self) -> impl Iterator<Item = HDTF_R> + '_ {
        (0..7).map(move |n| HDTF_R::new(((self.bits >> (n * 4 + 2)) & 1) != 0))
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
    #[doc = "Bit 22 - Channel 6 half transfer event flag"]
    #[inline(always)]
    pub fn hdtf6(&self) -> HDTF_R {
        HDTF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - Channel 7 half transfer event flag"]
    #[inline(always)]
    pub fn hdtf7(&self) -> HDTF_R {
        HDTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Channel (1-7) data transfer error event flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DTERRF1` field.</div>"]
    #[inline(always)]
    pub fn dterrf(&self, n: u8) -> DTERRF_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        DTERRF_R::new(((self.bits >> (n * 4 + 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-7) data transfer error event flag"]
    #[inline(always)]
    pub fn dterrf_iter(&self) -> impl Iterator<Item = DTERRF_R> + '_ {
        (0..7).map(move |n| DTERRF_R::new(((self.bits >> (n * 4 + 3)) & 1) != 0))
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
    #[doc = "Bit 23 - Channel 6 data transfer error event flag"]
    #[inline(always)]
    pub fn dterrf6(&self) -> DTERRF_R {
        DTERRF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 27 - Channel 7 data transfer error event flag"]
    #[inline(always)]
    pub fn dterrf7(&self) -> DTERRF_R {
        DTERRF_R::new(((self.bits >> 27) & 1) != 0)
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
            .field("gf6", &self.gf6())
            .field("gf7", &self.gf7())
            .field("fdtf1", &self.fdtf1())
            .field("fdtf2", &self.fdtf2())
            .field("fdtf3", &self.fdtf3())
            .field("fdtf4", &self.fdtf4())
            .field("fdtf5", &self.fdtf5())
            .field("fdtf6", &self.fdtf6())
            .field("fdtf7", &self.fdtf7())
            .field("hdtf1", &self.hdtf1())
            .field("hdtf2", &self.hdtf2())
            .field("hdtf3", &self.hdtf3())
            .field("hdtf4", &self.hdtf4())
            .field("hdtf5", &self.hdtf5())
            .field("hdtf6", &self.hdtf6())
            .field("hdtf7", &self.hdtf7())
            .field("dterrf1", &self.dterrf1())
            .field("dterrf2", &self.dterrf2())
            .field("dterrf3", &self.dterrf3())
            .field("dterrf4", &self.dterrf4())
            .field("dterrf5", &self.dterrf5())
            .field("dterrf6", &self.dterrf6())
            .field("dterrf7", &self.dterrf7())
            .finish()
    }
}
#[doc = "DMA interrupt status register (DMA_STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {}
