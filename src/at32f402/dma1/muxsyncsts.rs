#[doc = "Register `MUXSYNCSTS` reader"]
pub type R = crate::R<MUXSYNCSTS_SPEC>;
#[doc = "Register `MUXSYNCSTS` writer"]
pub type W = crate::W<MUXSYNCSTS_SPEC>;
#[doc = "Channel %s synchronization overrun flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCOVF1_A {
    #[doc = "0: No synchronization overrun"]
    NoOverrun = 0,
    #[doc = "1: A synchronization overrun occurred (DMA request count < REQCNT)"]
    Overrun = 1,
}
impl From<SYNCOVF1_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCOVF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCOVF(1-7)` reader - Channel %s synchronization overrun flag"]
pub type SYNCOVF_R = crate::BitReader<SYNCOVF1_A>;
impl SYNCOVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYNCOVF1_A {
        match self.bits {
            false => SYNCOVF1_A::NoOverrun,
            true => SYNCOVF1_A::Overrun,
        }
    }
    #[doc = "No synchronization overrun"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == SYNCOVF1_A::NoOverrun
    }
    #[doc = "A synchronization overrun occurred (DMA request count < REQCNT)"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == SYNCOVF1_A::Overrun
    }
}
impl R {
    #[doc = "Channel (1-7) synchronization overrun flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `SYNCOVF1` field.</div>"]
    #[inline(always)]
    pub fn syncovf(&self, n: u8) -> SYNCOVF_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        SYNCOVF_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-7) synchronization overrun flag"]
    #[inline(always)]
    pub fn syncovf_iter(&self) -> impl Iterator<Item = SYNCOVF_R> + '_ {
        (0..7).map(move |n| SYNCOVF_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Channel 1 synchronization overrun flag"]
    #[inline(always)]
    pub fn syncovf1(&self) -> SYNCOVF_R {
        SYNCOVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 2 synchronization overrun flag"]
    #[inline(always)]
    pub fn syncovf2(&self) -> SYNCOVF_R {
        SYNCOVF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 3 synchronization overrun flag"]
    #[inline(always)]
    pub fn syncovf3(&self) -> SYNCOVF_R {
        SYNCOVF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 4 synchronization overrun flag"]
    #[inline(always)]
    pub fn syncovf4(&self) -> SYNCOVF_R {
        SYNCOVF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 5 synchronization overrun flag"]
    #[inline(always)]
    pub fn syncovf5(&self) -> SYNCOVF_R {
        SYNCOVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 6 synchronization overrun flag"]
    #[inline(always)]
    pub fn syncovf6(&self) -> SYNCOVF_R {
        SYNCOVF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 7 synchronization overrun flag"]
    #[inline(always)]
    pub fn syncovf7(&self) -> SYNCOVF_R {
        SYNCOVF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUXSYNCSTS")
            .field("syncovf1", &self.syncovf1())
            .field("syncovf2", &self.syncovf2())
            .field("syncovf3", &self.syncovf3())
            .field("syncovf4", &self.syncovf4())
            .field("syncovf5", &self.syncovf5())
            .field("syncovf6", &self.syncovf6())
            .field("syncovf7", &self.syncovf7())
            .finish()
    }
}
impl W {}
#[doc = "Channel Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxsyncsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxsyncsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUXSYNCSTS_SPEC;
impl crate::RegisterSpec for MUXSYNCSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxsyncsts::R`](R) reader structure"]
impl crate::Readable for MUXSYNCSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`muxsyncsts::W`](W) writer structure"]
impl crate::Writable for MUXSYNCSTS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MUXSYNCSTS to value 0"]
impl crate::Resettable for MUXSYNCSTS_SPEC {}
