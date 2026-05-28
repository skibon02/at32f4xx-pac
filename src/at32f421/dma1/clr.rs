#[doc = "Register `CLR` reader"]
pub type R = crate::R<CLR_SPEC>;
#[doc = "Register `CLR` writer"]
pub type W = crate::W<CLR_SPEC>;
#[doc = "Channel %s global flag clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GFC1_A {
    #[doc = "1: Clear all flags DTERRFx, HDTFx, FDTFx and GFx"]
    Clear = 1,
}
impl From<GFC1_A> for bool {
    #[inline(always)]
    fn from(variant: GFC1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GFC(1-5)` reader - Channel %s global flag clear"]
pub type GFC_R = crate::BitReader<GFC1_A>;
impl GFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GFC1_A> {
        match self.bits {
            true => Some(GFC1_A::Clear),
            _ => None,
        }
    }
    #[doc = "Clear all flags DTERRFx, HDTFx, FDTFx and GFx"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GFC1_A::Clear
    }
}
#[doc = "Field `GFC(1-5)` writer - Channel %s global flag clear"]
pub type GFC_W<'a, REG> = crate::BitWriter1C<'a, REG, GFC1_A>;
impl<'a, REG> GFC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear all flags DTERRFx, HDTFx, FDTFx and GFx"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(GFC1_A::Clear)
    }
}
#[doc = "Channel %s data transfer complete flag clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FDTFC1_A {
    #[doc = "1: Clear the transfer complete flag"]
    Clear = 1,
}
impl From<FDTFC1_A> for bool {
    #[inline(always)]
    fn from(variant: FDTFC1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDTFC(1-5)` reader - Channel %s data transfer complete flag clear"]
pub type FDTFC_R = crate::BitReader<FDTFC1_A>;
impl FDTFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FDTFC1_A> {
        match self.bits {
            true => Some(FDTFC1_A::Clear),
            _ => None,
        }
    }
    #[doc = "Clear the transfer complete flag"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == FDTFC1_A::Clear
    }
}
#[doc = "Field `FDTFC(1-5)` writer - Channel %s data transfer complete flag clear"]
pub type FDTFC_W<'a, REG> = crate::BitWriter1C<'a, REG, FDTFC1_A>;
impl<'a, REG> FDTFC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the transfer complete flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FDTFC1_A::Clear)
    }
}
#[doc = "Channel %s half transfer flag clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDTFC1_A {
    #[doc = "1: Clear the half transfer flag"]
    Clear = 1,
}
impl From<HDTFC1_A> for bool {
    #[inline(always)]
    fn from(variant: HDTFC1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDTFC(1-5)` reader - Channel %s half transfer flag clear"]
pub type HDTFC_R = crate::BitReader<HDTFC1_A>;
impl HDTFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HDTFC1_A> {
        match self.bits {
            true => Some(HDTFC1_A::Clear),
            _ => None,
        }
    }
    #[doc = "Clear the half transfer flag"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == HDTFC1_A::Clear
    }
}
#[doc = "Field `HDTFC(1-5)` writer - Channel %s half transfer flag clear"]
pub type HDTFC_W<'a, REG> = crate::BitWriter1C<'a, REG, HDTFC1_A>;
impl<'a, REG> HDTFC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the half transfer flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(HDTFC1_A::Clear)
    }
}
#[doc = "Channel %s data transfer error flag clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTERRFC1_A {
    #[doc = "1: Clear the transfer error flag"]
    Clear = 1,
}
impl From<DTERRFC1_A> for bool {
    #[inline(always)]
    fn from(variant: DTERRFC1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTERRFC(1-5)` reader - Channel %s data transfer error flag clear"]
pub type DTERRFC_R = crate::BitReader<DTERRFC1_A>;
impl DTERRFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DTERRFC1_A> {
        match self.bits {
            true => Some(DTERRFC1_A::Clear),
            _ => None,
        }
    }
    #[doc = "Clear the transfer error flag"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == DTERRFC1_A::Clear
    }
}
#[doc = "Field `DTERRFC(1-5)` writer - Channel %s data transfer error flag clear"]
pub type DTERRFC_W<'a, REG> = crate::BitWriter1C<'a, REG, DTERRFC1_A>;
impl<'a, REG> DTERRFC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the transfer error flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(DTERRFC1_A::Clear)
    }
}
impl R {
    #[doc = "Channel (1-5) global flag clear"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GFC1` field.</div>"]
    #[inline(always)]
    pub fn gfc(&self, n: u8) -> GFC_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        GFC_R::new(((self.bits >> (n * 4)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-5) global flag clear"]
    #[inline(always)]
    pub fn gfc_iter(&self) -> impl Iterator<Item = GFC_R> + '_ {
        (0..5).map(move |n| GFC_R::new(((self.bits >> (n * 4)) & 1) != 0))
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
    #[doc = "Channel (1-5) data transfer complete flag clear"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `FDTFC1` field.</div>"]
    #[inline(always)]
    pub fn fdtfc(&self, n: u8) -> FDTFC_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        FDTFC_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-5) data transfer complete flag clear"]
    #[inline(always)]
    pub fn fdtfc_iter(&self) -> impl Iterator<Item = FDTFC_R> + '_ {
        (0..5).map(move |n| FDTFC_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0))
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
    #[doc = "Channel (1-5) half transfer flag clear"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `HDTFC1` field.</div>"]
    #[inline(always)]
    pub fn hdtfc(&self, n: u8) -> HDTFC_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        HDTFC_R::new(((self.bits >> (n * 4 + 2)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-5) half transfer flag clear"]
    #[inline(always)]
    pub fn hdtfc_iter(&self) -> impl Iterator<Item = HDTFC_R> + '_ {
        (0..5).map(move |n| HDTFC_R::new(((self.bits >> (n * 4 + 2)) & 1) != 0))
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
    #[doc = "Channel (1-5) data transfer error flag clear"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DTERRFC1` field.</div>"]
    #[inline(always)]
    pub fn dterrfc(&self, n: u8) -> DTERRFC_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        DTERRFC_R::new(((self.bits >> (n * 4 + 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-5) data transfer error flag clear"]
    #[inline(always)]
    pub fn dterrfc_iter(&self) -> impl Iterator<Item = DTERRFC_R> + '_ {
        (0..5).map(move |n| DTERRFC_R::new(((self.bits >> (n * 4 + 3)) & 1) != 0))
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLR")
            .field("gfc1", &self.gfc1())
            .field("gfc2", &self.gfc2())
            .field("gfc3", &self.gfc3())
            .field("gfc4", &self.gfc4())
            .field("gfc5", &self.gfc5())
            .field("fdtfc1", &self.fdtfc1())
            .field("fdtfc2", &self.fdtfc2())
            .field("fdtfc3", &self.fdtfc3())
            .field("fdtfc4", &self.fdtfc4())
            .field("fdtfc5", &self.fdtfc5())
            .field("hdtfc1", &self.hdtfc1())
            .field("hdtfc2", &self.hdtfc2())
            .field("hdtfc3", &self.hdtfc3())
            .field("hdtfc4", &self.hdtfc4())
            .field("hdtfc5", &self.hdtfc5())
            .field("dterrfc1", &self.dterrfc1())
            .field("dterrfc2", &self.dterrfc2())
            .field("dterrfc3", &self.dterrfc3())
            .field("dterrfc4", &self.dterrfc4())
            .field("dterrfc5", &self.dterrfc5())
            .finish()
    }
}
impl W {
    #[doc = "Channel (1-5) global flag clear"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GFC1` field.</div>"]
    #[inline(always)]
    pub fn gfc(&mut self, n: u8) -> GFC_W<'_, CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
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
    #[doc = "Channel (1-5) data transfer complete flag clear"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `FDTFC1` field.</div>"]
    #[inline(always)]
    pub fn fdtfc(&mut self, n: u8) -> FDTFC_W<'_, CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
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
    #[doc = "Channel (1-5) half transfer flag clear"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `HDTFC1` field.</div>"]
    #[inline(always)]
    pub fn hdtfc(&mut self, n: u8) -> HDTFC_W<'_, CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
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
    #[doc = "Channel (1-5) data transfer error flag clear"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DTERRFC1` field.</div>"]
    #[inline(always)]
    pub fn dterrfc(&mut self, n: u8) -> DTERRFC_W<'_, CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
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
}
#[doc = "DMA flag clear register (DMA_CLR)\n\nYou can [`read`](crate::Reg::read) this register and get [`clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLR_SPEC;
impl crate::RegisterSpec for CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr::R`](R) reader structure"]
impl crate::Readable for CLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clr::W`](W) writer structure"]
impl crate::Writable for CLR_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x000f_ffff;
}
#[doc = "`reset()` method sets CLR to value 0"]
impl crate::Resettable for CLR_SPEC {}
