#[doc = "Register `MUXGSTS` reader"]
pub type R = crate::R<MUXGSTS_SPEC>;
#[doc = "Register `MUXGSTS` writer"]
pub type W = crate::W<MUXGSTS_SPEC>;
#[doc = "Field `TRGOVF(1-4)` reader - Channel %s trigger overrun flag"]
pub type TRGOVF_R = crate::BitReader;
impl R {
    #[doc = "Channel (1-4) trigger overrun flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TRGOVF1` field.</div>"]
    #[inline(always)]
    pub fn trgovf(&self, n: u8) -> TRGOVF_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        TRGOVF_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-4) trigger overrun flag"]
    #[inline(always)]
    pub fn trgovf_iter(&self) -> impl Iterator<Item = TRGOVF_R> + '_ {
        (0..4).map(move |n| TRGOVF_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Channel 1 trigger overrun flag"]
    #[inline(always)]
    pub fn trgovf1(&self) -> TRGOVF_R {
        TRGOVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 2 trigger overrun flag"]
    #[inline(always)]
    pub fn trgovf2(&self) -> TRGOVF_R {
        TRGOVF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 3 trigger overrun flag"]
    #[inline(always)]
    pub fn trgovf3(&self) -> TRGOVF_R {
        TRGOVF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 4 trigger overrun flag"]
    #[inline(always)]
    pub fn trgovf4(&self) -> TRGOVF_R {
        TRGOVF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUXGSTS")
            .field("trgovf1", &self.trgovf1())
            .field("trgovf2", &self.trgovf2())
            .field("trgovf3", &self.trgovf3())
            .field("trgovf4", &self.trgovf4())
            .finish()
    }
}
impl W {}
#[doc = "Generator Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxgsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxgsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUXGSTS_SPEC;
impl crate::RegisterSpec for MUXGSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxgsts::R`](R) reader structure"]
impl crate::Readable for MUXGSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`muxgsts::W`](W) writer structure"]
impl crate::Writable for MUXGSTS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MUXGSTS to value 0"]
impl crate::Resettable for MUXGSTS_SPEC {}
