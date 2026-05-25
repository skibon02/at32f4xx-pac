#[doc = "Register `CLR` reader"]
pub type R = crate::R<CLR_SPEC>;
#[doc = "Register `CLR` writer"]
pub type W = crate::W<CLR_SPEC>;
#[doc = "Field `GFC(1-7)` reader - Channel %s global flag clear"]
pub type GFC_R = crate::BitReader;
#[doc = "Field `GFC(1-7)` writer - Channel %s global flag clear"]
pub type GFC_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FDTFC(1-7)` reader - Channel %s data transfer complete flag clear"]
pub type FDTFC_R = crate::BitReader;
#[doc = "Field `FDTFC(1-7)` writer - Channel %s data transfer complete flag clear"]
pub type FDTFC_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `HDTFC(1-7)` reader - Channel %s half transfer flag clear"]
pub type HDTFC_R = crate::BitReader;
#[doc = "Field `HDTFC(1-7)` writer - Channel %s half transfer flag clear"]
pub type HDTFC_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DTERRFC(1-7)` reader - Channel %s data transfer error flag clear"]
pub type DTERRFC_R = crate::BitReader;
#[doc = "Field `DTERRFC(1-7)` writer - Channel %s data transfer error flag clear"]
pub type DTERRFC_W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Channel (1-7) global flag clear"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GFC1` field.</div>"]
    #[inline(always)]
    pub fn gfc(&self, n: u8) -> GFC_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        GFC_R::new(((self.bits >> (n * 4)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-7) global flag clear"]
    #[inline(always)]
    pub fn gfc_iter(&self) -> impl Iterator<Item = GFC_R> + '_ {
        (0..7).map(move |n| GFC_R::new(((self.bits >> (n * 4)) & 1) != 0))
    }
    #[doc = "Bit 0 - Channel 1 global flag clear"]
    #[inline(always)]
    pub fn gfc1(&self) -> GFC_R {
        GFC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 2 global flag clear"]
    #[inline(always)]
    pub fn gfc2(&self) -> GFC_R {
        GFC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 3 global flag clear"]
    #[inline(always)]
    pub fn gfc3(&self) -> GFC_R {
        GFC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 4 global flag clear"]
    #[inline(always)]
    pub fn gfc4(&self) -> GFC_R {
        GFC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 5 global flag clear"]
    #[inline(always)]
    pub fn gfc5(&self) -> GFC_R {
        GFC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Channel 6 global flag clear"]
    #[inline(always)]
    pub fn gfc6(&self) -> GFC_R {
        GFC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Channel 7 global flag clear"]
    #[inline(always)]
    pub fn gfc7(&self) -> GFC_R {
        GFC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Channel (1-7) data transfer complete flag clear"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `FDTFC1` field.</div>"]
    #[inline(always)]
    pub fn fdtfc(&self, n: u8) -> FDTFC_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        FDTFC_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-7) data transfer complete flag clear"]
    #[inline(always)]
    pub fn fdtfc_iter(&self) -> impl Iterator<Item = FDTFC_R> + '_ {
        (0..7).map(move |n| FDTFC_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - Channel 1 data transfer complete flag clear"]
    #[inline(always)]
    pub fn fdtfc1(&self) -> FDTFC_R {
        FDTFC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 2 data transfer complete flag clear"]
    #[inline(always)]
    pub fn fdtfc2(&self) -> FDTFC_R {
        FDTFC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 3 data transfer complete flag clear"]
    #[inline(always)]
    pub fn fdtfc3(&self) -> FDTFC_R {
        FDTFC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 4 data transfer complete flag clear"]
    #[inline(always)]
    pub fn fdtfc4(&self) -> FDTFC_R {
        FDTFC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel 5 data transfer complete flag clear"]
    #[inline(always)]
    pub fn fdtfc5(&self) -> FDTFC_R {
        FDTFC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - Channel 6 data transfer complete flag clear"]
    #[inline(always)]
    pub fn fdtfc6(&self) -> FDTFC_R {
        FDTFC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 25 - Channel 7 data transfer complete flag clear"]
    #[inline(always)]
    pub fn fdtfc7(&self) -> FDTFC_R {
        FDTFC_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Channel (1-7) half transfer flag clear"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `HDTFC1` field.</div>"]
    #[inline(always)]
    pub fn hdtfc(&self, n: u8) -> HDTFC_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        HDTFC_R::new(((self.bits >> (n * 4 + 2)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-7) half transfer flag clear"]
    #[inline(always)]
    pub fn hdtfc_iter(&self) -> impl Iterator<Item = HDTFC_R> + '_ {
        (0..7).map(move |n| HDTFC_R::new(((self.bits >> (n * 4 + 2)) & 1) != 0))
    }
    #[doc = "Bit 2 - Channel 1 half transfer flag clear"]
    #[inline(always)]
    pub fn hdtfc1(&self) -> HDTFC_R {
        HDTFC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 2 half transfer flag clear"]
    #[inline(always)]
    pub fn hdtfc2(&self) -> HDTFC_R {
        HDTFC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 3 half transfer flag clear"]
    #[inline(always)]
    pub fn hdtfc3(&self) -> HDTFC_R {
        HDTFC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 4 half transfer flag clear"]
    #[inline(always)]
    pub fn hdtfc4(&self) -> HDTFC_R {
        HDTFC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel 5 half transfer flag clear"]
    #[inline(always)]
    pub fn hdtfc5(&self) -> HDTFC_R {
        HDTFC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 22 - Channel 6 half transfer flag clear"]
    #[inline(always)]
    pub fn hdtfc6(&self) -> HDTFC_R {
        HDTFC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - Channel 7 half transfer flag clear"]
    #[inline(always)]
    pub fn hdtfc7(&self) -> HDTFC_R {
        HDTFC_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Channel (1-7) data transfer error flag clear"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DTERRFC1` field.</div>"]
    #[inline(always)]
    pub fn dterrfc(&self, n: u8) -> DTERRFC_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        DTERRFC_R::new(((self.bits >> (n * 4 + 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-7) data transfer error flag clear"]
    #[inline(always)]
    pub fn dterrfc_iter(&self) -> impl Iterator<Item = DTERRFC_R> + '_ {
        (0..7).map(move |n| DTERRFC_R::new(((self.bits >> (n * 4 + 3)) & 1) != 0))
    }
    #[doc = "Bit 3 - Channel 1 data transfer error flag clear"]
    #[inline(always)]
    pub fn dterrfc1(&self) -> DTERRFC_R {
        DTERRFC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 2 data transfer error flag clear"]
    #[inline(always)]
    pub fn dterrfc2(&self) -> DTERRFC_R {
        DTERRFC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 data transfer error flag clear"]
    #[inline(always)]
    pub fn dterrfc3(&self) -> DTERRFC_R {
        DTERRFC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 4 data transfer error flag clear"]
    #[inline(always)]
    pub fn dterrfc4(&self) -> DTERRFC_R {
        DTERRFC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel 5 data transfer error flag clear"]
    #[inline(always)]
    pub fn dterrfc5(&self) -> DTERRFC_R {
        DTERRFC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel 6 data transfer error flag clear"]
    #[inline(always)]
    pub fn dterrfc6(&self) -> DTERRFC_R {
        DTERRFC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 27 - Channel 7 data transfer error flag clear"]
    #[inline(always)]
    pub fn dterrfc7(&self) -> DTERRFC_R {
        DTERRFC_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLR")
            .field("gfc1", &self.gfc1())
            .field("gfc2", &self.gfc2())
            .field("gfc3", &self.gfc3())
            .field("gfc4", &self.gfc4())
            .field("gfc5", &self.gfc5())
            .field("gfc6", &self.gfc6())
            .field("gfc7", &self.gfc7())
            .field("fdtfc1", &self.fdtfc1())
            .field("fdtfc2", &self.fdtfc2())
            .field("fdtfc3", &self.fdtfc3())
            .field("fdtfc4", &self.fdtfc4())
            .field("fdtfc5", &self.fdtfc5())
            .field("fdtfc6", &self.fdtfc6())
            .field("fdtfc7", &self.fdtfc7())
            .field("hdtfc1", &self.hdtfc1())
            .field("hdtfc2", &self.hdtfc2())
            .field("hdtfc3", &self.hdtfc3())
            .field("hdtfc4", &self.hdtfc4())
            .field("hdtfc5", &self.hdtfc5())
            .field("hdtfc6", &self.hdtfc6())
            .field("hdtfc7", &self.hdtfc7())
            .field("dterrfc1", &self.dterrfc1())
            .field("dterrfc2", &self.dterrfc2())
            .field("dterrfc3", &self.dterrfc3())
            .field("dterrfc4", &self.dterrfc4())
            .field("dterrfc5", &self.dterrfc5())
            .field("dterrfc6", &self.dterrfc6())
            .field("dterrfc7", &self.dterrfc7())
            .finish()
    }
}
impl W {
    #[doc = "Channel (1-7) global flag clear"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GFC1` field.</div>"]
    #[inline(always)]
    pub fn gfc(&mut self, n: u8) -> GFC_W<'_, CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        GFC_W::new(self, n * 4)
    }
    #[doc = "Bit 0 - Channel 1 global flag clear"]
    #[inline(always)]
    pub fn gfc1(&mut self) -> GFC_W<'_, CLR_SPEC> {
        GFC_W::new(self, 0)
    }
    #[doc = "Bit 4 - Channel 2 global flag clear"]
    #[inline(always)]
    pub fn gfc2(&mut self) -> GFC_W<'_, CLR_SPEC> {
        GFC_W::new(self, 4)
    }
    #[doc = "Bit 8 - Channel 3 global flag clear"]
    #[inline(always)]
    pub fn gfc3(&mut self) -> GFC_W<'_, CLR_SPEC> {
        GFC_W::new(self, 8)
    }
    #[doc = "Bit 12 - Channel 4 global flag clear"]
    #[inline(always)]
    pub fn gfc4(&mut self) -> GFC_W<'_, CLR_SPEC> {
        GFC_W::new(self, 12)
    }
    #[doc = "Bit 16 - Channel 5 global flag clear"]
    #[inline(always)]
    pub fn gfc5(&mut self) -> GFC_W<'_, CLR_SPEC> {
        GFC_W::new(self, 16)
    }
    #[doc = "Bit 20 - Channel 6 global flag clear"]
    #[inline(always)]
    pub fn gfc6(&mut self) -> GFC_W<'_, CLR_SPEC> {
        GFC_W::new(self, 20)
    }
    #[doc = "Bit 24 - Channel 7 global flag clear"]
    #[inline(always)]
    pub fn gfc7(&mut self) -> GFC_W<'_, CLR_SPEC> {
        GFC_W::new(self, 24)
    }
    #[doc = "Channel (1-7) data transfer complete flag clear"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `FDTFC1` field.</div>"]
    #[inline(always)]
    pub fn fdtfc(&mut self, n: u8) -> FDTFC_W<'_, CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        FDTFC_W::new(self, n * 4 + 1)
    }
    #[doc = "Bit 1 - Channel 1 data transfer complete flag clear"]
    #[inline(always)]
    pub fn fdtfc1(&mut self) -> FDTFC_W<'_, CLR_SPEC> {
        FDTFC_W::new(self, 1)
    }
    #[doc = "Bit 5 - Channel 2 data transfer complete flag clear"]
    #[inline(always)]
    pub fn fdtfc2(&mut self) -> FDTFC_W<'_, CLR_SPEC> {
        FDTFC_W::new(self, 5)
    }
    #[doc = "Bit 9 - Channel 3 data transfer complete flag clear"]
    #[inline(always)]
    pub fn fdtfc3(&mut self) -> FDTFC_W<'_, CLR_SPEC> {
        FDTFC_W::new(self, 9)
    }
    #[doc = "Bit 13 - Channel 4 data transfer complete flag clear"]
    #[inline(always)]
    pub fn fdtfc4(&mut self) -> FDTFC_W<'_, CLR_SPEC> {
        FDTFC_W::new(self, 13)
    }
    #[doc = "Bit 17 - Channel 5 data transfer complete flag clear"]
    #[inline(always)]
    pub fn fdtfc5(&mut self) -> FDTFC_W<'_, CLR_SPEC> {
        FDTFC_W::new(self, 17)
    }
    #[doc = "Bit 21 - Channel 6 data transfer complete flag clear"]
    #[inline(always)]
    pub fn fdtfc6(&mut self) -> FDTFC_W<'_, CLR_SPEC> {
        FDTFC_W::new(self, 21)
    }
    #[doc = "Bit 25 - Channel 7 data transfer complete flag clear"]
    #[inline(always)]
    pub fn fdtfc7(&mut self) -> FDTFC_W<'_, CLR_SPEC> {
        FDTFC_W::new(self, 25)
    }
    #[doc = "Channel (1-7) half transfer flag clear"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `HDTFC1` field.</div>"]
    #[inline(always)]
    pub fn hdtfc(&mut self, n: u8) -> HDTFC_W<'_, CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        HDTFC_W::new(self, n * 4 + 2)
    }
    #[doc = "Bit 2 - Channel 1 half transfer flag clear"]
    #[inline(always)]
    pub fn hdtfc1(&mut self) -> HDTFC_W<'_, CLR_SPEC> {
        HDTFC_W::new(self, 2)
    }
    #[doc = "Bit 6 - Channel 2 half transfer flag clear"]
    #[inline(always)]
    pub fn hdtfc2(&mut self) -> HDTFC_W<'_, CLR_SPEC> {
        HDTFC_W::new(self, 6)
    }
    #[doc = "Bit 10 - Channel 3 half transfer flag clear"]
    #[inline(always)]
    pub fn hdtfc3(&mut self) -> HDTFC_W<'_, CLR_SPEC> {
        HDTFC_W::new(self, 10)
    }
    #[doc = "Bit 14 - Channel 4 half transfer flag clear"]
    #[inline(always)]
    pub fn hdtfc4(&mut self) -> HDTFC_W<'_, CLR_SPEC> {
        HDTFC_W::new(self, 14)
    }
    #[doc = "Bit 18 - Channel 5 half transfer flag clear"]
    #[inline(always)]
    pub fn hdtfc5(&mut self) -> HDTFC_W<'_, CLR_SPEC> {
        HDTFC_W::new(self, 18)
    }
    #[doc = "Bit 22 - Channel 6 half transfer flag clear"]
    #[inline(always)]
    pub fn hdtfc6(&mut self) -> HDTFC_W<'_, CLR_SPEC> {
        HDTFC_W::new(self, 22)
    }
    #[doc = "Bit 26 - Channel 7 half transfer flag clear"]
    #[inline(always)]
    pub fn hdtfc7(&mut self) -> HDTFC_W<'_, CLR_SPEC> {
        HDTFC_W::new(self, 26)
    }
    #[doc = "Channel (1-7) data transfer error flag clear"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DTERRFC1` field.</div>"]
    #[inline(always)]
    pub fn dterrfc(&mut self, n: u8) -> DTERRFC_W<'_, CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        DTERRFC_W::new(self, n * 4 + 3)
    }
    #[doc = "Bit 3 - Channel 1 data transfer error flag clear"]
    #[inline(always)]
    pub fn dterrfc1(&mut self) -> DTERRFC_W<'_, CLR_SPEC> {
        DTERRFC_W::new(self, 3)
    }
    #[doc = "Bit 7 - Channel 2 data transfer error flag clear"]
    #[inline(always)]
    pub fn dterrfc2(&mut self) -> DTERRFC_W<'_, CLR_SPEC> {
        DTERRFC_W::new(self, 7)
    }
    #[doc = "Bit 11 - Channel 3 data transfer error flag clear"]
    #[inline(always)]
    pub fn dterrfc3(&mut self) -> DTERRFC_W<'_, CLR_SPEC> {
        DTERRFC_W::new(self, 11)
    }
    #[doc = "Bit 15 - Channel 4 data transfer error flag clear"]
    #[inline(always)]
    pub fn dterrfc4(&mut self) -> DTERRFC_W<'_, CLR_SPEC> {
        DTERRFC_W::new(self, 15)
    }
    #[doc = "Bit 19 - Channel 5 data transfer error flag clear"]
    #[inline(always)]
    pub fn dterrfc5(&mut self) -> DTERRFC_W<'_, CLR_SPEC> {
        DTERRFC_W::new(self, 19)
    }
    #[doc = "Bit 23 - Channel 6 data transfer error flag clear"]
    #[inline(always)]
    pub fn dterrfc6(&mut self) -> DTERRFC_W<'_, CLR_SPEC> {
        DTERRFC_W::new(self, 23)
    }
    #[doc = "Bit 27 - Channel 7 data transfer error flag clear"]
    #[inline(always)]
    pub fn dterrfc7(&mut self) -> DTERRFC_W<'_, CLR_SPEC> {
        DTERRFC_W::new(self, 27)
    }
}
#[doc = "DMA interrupt flag clear register (DMA_CLR)\n\nYou can [`read`](crate::Reg::read) this register and get [`clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLR_SPEC;
impl crate::RegisterSpec for CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr::R`](R) reader structure"]
impl crate::Readable for CLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clr::W`](W) writer structure"]
impl crate::Writable for CLR_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0fff_ffff;
}
#[doc = "`reset()` method sets CLR to value 0"]
impl crate::Resettable for CLR_SPEC {}
