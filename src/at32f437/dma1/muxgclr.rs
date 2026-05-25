#[doc = "Register `MUXGCLR` reader"]
pub type R = crate::R<MUXGCLR_SPEC>;
#[doc = "Register `MUXGCLR` writer"]
pub type W = crate::W<MUXGCLR_SPEC>;
#[doc = "Field `TRGOVFC(1-4)` reader - Channel %s trigger overrun flag clear"]
pub type TRGOVFC_R = crate::BitReader;
#[doc = "Field `TRGOVFC(1-4)` writer - Channel %s trigger overrun flag clear"]
pub type TRGOVFC_W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Channel (1-4) trigger overrun flag clear"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TRGOVFC1` field.</div>"]
    #[inline(always)]
    pub fn trgovfc(&self, n: u8) -> TRGOVFC_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        TRGOVFC_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-4) trigger overrun flag clear"]
    #[inline(always)]
    pub fn trgovfc_iter(&self) -> impl Iterator<Item = TRGOVFC_R> + '_ {
        (0..4).map(move |n| TRGOVFC_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Channel 1 trigger overrun flag clear"]
    #[inline(always)]
    pub fn trgovfc1(&self) -> TRGOVFC_R {
        TRGOVFC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 2 trigger overrun flag clear"]
    #[inline(always)]
    pub fn trgovfc2(&self) -> TRGOVFC_R {
        TRGOVFC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 3 trigger overrun flag clear"]
    #[inline(always)]
    pub fn trgovfc3(&self) -> TRGOVFC_R {
        TRGOVFC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 4 trigger overrun flag clear"]
    #[inline(always)]
    pub fn trgovfc4(&self) -> TRGOVFC_R {
        TRGOVFC_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUXGCLR")
            .field("trgovfc1", &self.trgovfc1())
            .field("trgovfc2", &self.trgovfc2())
            .field("trgovfc3", &self.trgovfc3())
            .field("trgovfc4", &self.trgovfc4())
            .finish()
    }
}
impl W {
    #[doc = "Channel (1-4) trigger overrun flag clear"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TRGOVFC1` field.</div>"]
    #[inline(always)]
    pub fn trgovfc(&mut self, n: u8) -> TRGOVFC_W<'_, MUXGCLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        TRGOVFC_W::new(self, n)
    }
    #[doc = "Bit 0 - Channel 1 trigger overrun flag clear"]
    #[inline(always)]
    pub fn trgovfc1(&mut self) -> TRGOVFC_W<'_, MUXGCLR_SPEC> {
        TRGOVFC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 2 trigger overrun flag clear"]
    #[inline(always)]
    pub fn trgovfc2(&mut self) -> TRGOVFC_W<'_, MUXGCLR_SPEC> {
        TRGOVFC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 3 trigger overrun flag clear"]
    #[inline(always)]
    pub fn trgovfc3(&mut self) -> TRGOVFC_W<'_, MUXGCLR_SPEC> {
        TRGOVFC_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 4 trigger overrun flag clear"]
    #[inline(always)]
    pub fn trgovfc4(&mut self) -> TRGOVFC_W<'_, MUXGCLR_SPEC> {
        TRGOVFC_W::new(self, 3)
    }
}
#[doc = "Generator Interrupt Clear Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxgclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxgclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUXGCLR_SPEC;
impl crate::RegisterSpec for MUXGCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxgclr::R`](R) reader structure"]
impl crate::Readable for MUXGCLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`muxgclr::W`](W) writer structure"]
impl crate::Writable for MUXGCLR_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f;
}
#[doc = "`reset()` method sets MUXGCLR to value 0"]
impl crate::Resettable for MUXGCLR_SPEC {}
