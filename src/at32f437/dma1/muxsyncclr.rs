#[doc = "Register `MUXSYNCCLR` reader"]
pub type R = crate::R<MUXSYNCCLR_SPEC>;
#[doc = "Register `MUXSYNCCLR` writer"]
pub type W = crate::W<MUXSYNCCLR_SPEC>;
#[doc = "Field `SYNCOVFC(1-7)` reader - Channel %s synchronization overrun flag clear"]
pub type SYNCOVFC_R = crate::BitReader;
#[doc = "Field `SYNCOVFC(1-7)` writer - Channel %s synchronization overrun flag clear"]
pub type SYNCOVFC_W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Channel (1-7) synchronization overrun flag clear"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `SYNCOVFC1` field.</div>"]
    #[inline(always)]
    pub fn syncovfc(&self, n: u8) -> SYNCOVFC_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        SYNCOVFC_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-7) synchronization overrun flag clear"]
    #[inline(always)]
    pub fn syncovfc_iter(&self) -> impl Iterator<Item = SYNCOVFC_R> + '_ {
        (0..7).map(move |n| SYNCOVFC_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Channel 1 synchronization overrun flag clear"]
    #[inline(always)]
    pub fn syncovfc1(&self) -> SYNCOVFC_R {
        SYNCOVFC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 2 synchronization overrun flag clear"]
    #[inline(always)]
    pub fn syncovfc2(&self) -> SYNCOVFC_R {
        SYNCOVFC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 3 synchronization overrun flag clear"]
    #[inline(always)]
    pub fn syncovfc3(&self) -> SYNCOVFC_R {
        SYNCOVFC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 4 synchronization overrun flag clear"]
    #[inline(always)]
    pub fn syncovfc4(&self) -> SYNCOVFC_R {
        SYNCOVFC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 5 synchronization overrun flag clear"]
    #[inline(always)]
    pub fn syncovfc5(&self) -> SYNCOVFC_R {
        SYNCOVFC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 6 synchronization overrun flag clear"]
    #[inline(always)]
    pub fn syncovfc6(&self) -> SYNCOVFC_R {
        SYNCOVFC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 7 synchronization overrun flag clear"]
    #[inline(always)]
    pub fn syncovfc7(&self) -> SYNCOVFC_R {
        SYNCOVFC_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUXSYNCCLR")
            .field("syncovfc1", &self.syncovfc1())
            .field("syncovfc2", &self.syncovfc2())
            .field("syncovfc3", &self.syncovfc3())
            .field("syncovfc4", &self.syncovfc4())
            .field("syncovfc5", &self.syncovfc5())
            .field("syncovfc6", &self.syncovfc6())
            .field("syncovfc7", &self.syncovfc7())
            .finish()
    }
}
impl W {
    #[doc = "Channel (1-7) synchronization overrun flag clear"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `SYNCOVFC1` field.</div>"]
    #[inline(always)]
    pub fn syncovfc(&mut self, n: u8) -> SYNCOVFC_W<'_, MUXSYNCCLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        SYNCOVFC_W::new(self, n)
    }
    #[doc = "Bit 0 - Channel 1 synchronization overrun flag clear"]
    #[inline(always)]
    pub fn syncovfc1(&mut self) -> SYNCOVFC_W<'_, MUXSYNCCLR_SPEC> {
        SYNCOVFC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 2 synchronization overrun flag clear"]
    #[inline(always)]
    pub fn syncovfc2(&mut self) -> SYNCOVFC_W<'_, MUXSYNCCLR_SPEC> {
        SYNCOVFC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 3 synchronization overrun flag clear"]
    #[inline(always)]
    pub fn syncovfc3(&mut self) -> SYNCOVFC_W<'_, MUXSYNCCLR_SPEC> {
        SYNCOVFC_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 4 synchronization overrun flag clear"]
    #[inline(always)]
    pub fn syncovfc4(&mut self) -> SYNCOVFC_W<'_, MUXSYNCCLR_SPEC> {
        SYNCOVFC_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 5 synchronization overrun flag clear"]
    #[inline(always)]
    pub fn syncovfc5(&mut self) -> SYNCOVFC_W<'_, MUXSYNCCLR_SPEC> {
        SYNCOVFC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 6 synchronization overrun flag clear"]
    #[inline(always)]
    pub fn syncovfc6(&mut self) -> SYNCOVFC_W<'_, MUXSYNCCLR_SPEC> {
        SYNCOVFC_W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 7 synchronization overrun flag clear"]
    #[inline(always)]
    pub fn syncovfc7(&mut self) -> SYNCOVFC_W<'_, MUXSYNCCLR_SPEC> {
        SYNCOVFC_W::new(self, 6)
    }
}
#[doc = "Channel Interrupt Clear Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxsyncclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxsyncclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUXSYNCCLR_SPEC;
impl crate::RegisterSpec for MUXSYNCCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxsyncclr::R`](R) reader structure"]
impl crate::Readable for MUXSYNCCLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`muxsyncclr::W`](W) writer structure"]
impl crate::Writable for MUXSYNCCLR_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x7f;
}
#[doc = "`reset()` method sets MUXSYNCCLR to value 0"]
impl crate::Resettable for MUXSYNCCLR_SPEC {}
