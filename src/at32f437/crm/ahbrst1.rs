#[doc = "Register `AHBRST1` reader"]
pub type R = crate::R<AHBRST1_SPEC>;
#[doc = "Register `AHBRST1` writer"]
pub type W = crate::W<AHBRST1_SPEC>;
#[doc = "IO port A reset\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOAW_A {
    #[doc = "1: Reset peripheral"]
    Reset = 1,
}
impl From<GPIOAW_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOAW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOA` reader - IO port A reset"]
pub type GPIOA_R = crate::BitReader<GPIOAW_A>;
impl GPIOA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GPIOAW_A> {
        match self.bits {
            true => Some(GPIOAW_A::Reset),
            _ => None,
        }
    }
    #[doc = "Reset peripheral"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == GPIOAW_A::Reset
    }
}
#[doc = "Field `GPIOA` writer - IO port A reset"]
pub type GPIOA_W<'a, REG> = crate::BitWriter1S<'a, REG, GPIOAW_A>;
impl<'a, REG> GPIOA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset peripheral"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAW_A::Reset)
    }
}
#[doc = "Field `GPIOB` reader - IO port B reset"]
pub use GPIOA_R as GPIOB_R;
#[doc = "Field `GPIOC` reader - IO port C reset"]
pub use GPIOA_R as GPIOC_R;
#[doc = "Field `GPIOD` reader - IO port D reset"]
pub use GPIOA_R as GPIOD_R;
#[doc = "Field `GPIOE` reader - IO port E reset"]
pub use GPIOA_R as GPIOE_R;
#[doc = "Field `GPIOF` reader - IO port F reset"]
pub use GPIOA_R as GPIOF_R;
#[doc = "Field `GPIOG` reader - IO port G reset"]
pub use GPIOA_R as GPIOG_R;
#[doc = "Field `GPIOH` reader - IO port H reset"]
pub use GPIOA_R as GPIOH_R;
#[doc = "Field `CRC` reader - CRC reset"]
pub use GPIOA_R as CRC_R;
#[doc = "Field `EDMA` reader - EDMA reset"]
pub use GPIOA_R as EDMA_R;
#[doc = "Field `DMA1` reader - DMA1 reset"]
pub use GPIOA_R as DMA1_R;
#[doc = "Field `DMA2` reader - DMA2 reset"]
pub use GPIOA_R as DMA2_R;
#[doc = "Field `EMAC` reader - EMAC reset"]
pub use GPIOA_R as EMAC_R;
#[doc = "Field `OTGFS2` reader - OTGFS2 interface reset"]
pub use GPIOA_R as OTGFS2_R;
#[doc = "Field `GPIOB` writer - IO port B reset"]
pub use GPIOA_W as GPIOB_W;
#[doc = "Field `GPIOC` writer - IO port C reset"]
pub use GPIOA_W as GPIOC_W;
#[doc = "Field `GPIOD` writer - IO port D reset"]
pub use GPIOA_W as GPIOD_W;
#[doc = "Field `GPIOE` writer - IO port E reset"]
pub use GPIOA_W as GPIOE_W;
#[doc = "Field `GPIOF` writer - IO port F reset"]
pub use GPIOA_W as GPIOF_W;
#[doc = "Field `GPIOG` writer - IO port G reset"]
pub use GPIOA_W as GPIOG_W;
#[doc = "Field `GPIOH` writer - IO port H reset"]
pub use GPIOA_W as GPIOH_W;
#[doc = "Field `CRC` writer - CRC reset"]
pub use GPIOA_W as CRC_W;
#[doc = "Field `EDMA` writer - EDMA reset"]
pub use GPIOA_W as EDMA_W;
#[doc = "Field `DMA1` writer - DMA1 reset"]
pub use GPIOA_W as DMA1_W;
#[doc = "Field `DMA2` writer - DMA2 reset"]
pub use GPIOA_W as DMA2_W;
#[doc = "Field `EMAC` writer - EMAC reset"]
pub use GPIOA_W as EMAC_W;
#[doc = "Field `OTGFS2` writer - OTGFS2 interface reset"]
pub use GPIOA_W as OTGFS2_W;
impl R {
    #[doc = "Bit 0 - IO port A reset"]
    #[inline(always)]
    pub fn gpioa(&self) -> GPIOA_R {
        GPIOA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B reset"]
    #[inline(always)]
    pub fn gpiob(&self) -> GPIOB_R {
        GPIOB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C reset"]
    #[inline(always)]
    pub fn gpioc(&self) -> GPIOC_R {
        GPIOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D reset"]
    #[inline(always)]
    pub fn gpiod(&self) -> GPIOD_R {
        GPIOD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E reset"]
    #[inline(always)]
    pub fn gpioe(&self) -> GPIOE_R {
        GPIOE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port F reset"]
    #[inline(always)]
    pub fn gpiof(&self) -> GPIOF_R {
        GPIOF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port G reset"]
    #[inline(always)]
    pub fn gpiog(&self) -> GPIOG_R {
        GPIOG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port H reset"]
    #[inline(always)]
    pub fn gpioh(&self) -> GPIOH_R {
        GPIOH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - EDMA reset"]
    #[inline(always)]
    pub fn edma(&self) -> EDMA_R {
        EDMA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA1 reset"]
    #[inline(always)]
    pub fn dma1(&self) -> DMA1_R {
        DMA1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - DMA2 reset"]
    #[inline(always)]
    pub fn dma2(&self) -> DMA2_R {
        DMA2_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - EMAC reset"]
    #[inline(always)]
    pub fn emac(&self) -> EMAC_R {
        EMAC_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - OTGFS2 interface reset"]
    #[inline(always)]
    pub fn otgfs2(&self) -> OTGFS2_R {
        OTGFS2_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBRST1")
            .field("gpioa", &self.gpioa())
            .field("gpiob", &self.gpiob())
            .field("gpioc", &self.gpioc())
            .field("gpiod", &self.gpiod())
            .field("gpioe", &self.gpioe())
            .field("gpiof", &self.gpiof())
            .field("gpiog", &self.gpiog())
            .field("gpioh", &self.gpioh())
            .field("crc", &self.crc())
            .field("edma", &self.edma())
            .field("dma1", &self.dma1())
            .field("dma2", &self.dma2())
            .field("emac", &self.emac())
            .field("otgfs2", &self.otgfs2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - IO port A reset"]
    #[inline(always)]
    pub fn gpioa(&mut self) -> GPIOA_W<'_, AHBRST1_SPEC> {
        GPIOA_W::new(self, 0)
    }
    #[doc = "Bit 1 - IO port B reset"]
    #[inline(always)]
    pub fn gpiob(&mut self) -> GPIOB_W<'_, AHBRST1_SPEC> {
        GPIOB_W::new(self, 1)
    }
    #[doc = "Bit 2 - IO port C reset"]
    #[inline(always)]
    pub fn gpioc(&mut self) -> GPIOC_W<'_, AHBRST1_SPEC> {
        GPIOC_W::new(self, 2)
    }
    #[doc = "Bit 3 - IO port D reset"]
    #[inline(always)]
    pub fn gpiod(&mut self) -> GPIOD_W<'_, AHBRST1_SPEC> {
        GPIOD_W::new(self, 3)
    }
    #[doc = "Bit 4 - IO port E reset"]
    #[inline(always)]
    pub fn gpioe(&mut self) -> GPIOE_W<'_, AHBRST1_SPEC> {
        GPIOE_W::new(self, 4)
    }
    #[doc = "Bit 5 - IO port F reset"]
    #[inline(always)]
    pub fn gpiof(&mut self) -> GPIOF_W<'_, AHBRST1_SPEC> {
        GPIOF_W::new(self, 5)
    }
    #[doc = "Bit 6 - IO port G reset"]
    #[inline(always)]
    pub fn gpiog(&mut self) -> GPIOG_W<'_, AHBRST1_SPEC> {
        GPIOG_W::new(self, 6)
    }
    #[doc = "Bit 7 - IO port H reset"]
    #[inline(always)]
    pub fn gpioh(&mut self) -> GPIOH_W<'_, AHBRST1_SPEC> {
        GPIOH_W::new(self, 7)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    pub fn crc(&mut self) -> CRC_W<'_, AHBRST1_SPEC> {
        CRC_W::new(self, 12)
    }
    #[doc = "Bit 21 - EDMA reset"]
    #[inline(always)]
    pub fn edma(&mut self) -> EDMA_W<'_, AHBRST1_SPEC> {
        EDMA_W::new(self, 21)
    }
    #[doc = "Bit 22 - DMA1 reset"]
    #[inline(always)]
    pub fn dma1(&mut self) -> DMA1_W<'_, AHBRST1_SPEC> {
        DMA1_W::new(self, 22)
    }
    #[doc = "Bit 24 - DMA2 reset"]
    #[inline(always)]
    pub fn dma2(&mut self) -> DMA2_W<'_, AHBRST1_SPEC> {
        DMA2_W::new(self, 24)
    }
    #[doc = "Bit 25 - EMAC reset"]
    #[inline(always)]
    pub fn emac(&mut self) -> EMAC_W<'_, AHBRST1_SPEC> {
        EMAC_W::new(self, 25)
    }
    #[doc = "Bit 29 - OTGFS2 interface reset"]
    #[inline(always)]
    pub fn otgfs2(&mut self) -> OTGFS2_W<'_, AHBRST1_SPEC> {
        OTGFS2_W::new(self, 29)
    }
}
#[doc = "AHB peripheral reset register1 (CRM_AHBRST1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrst1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrst1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBRST1_SPEC;
impl crate::RegisterSpec for AHBRST1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrst1::R`](R) reader structure"]
impl crate::Readable for AHBRST1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahbrst1::W`](W) writer structure"]
impl crate::Writable for AHBRST1_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x2360_10ff;
}
#[doc = "`reset()` method sets AHBRST1 to value 0"]
impl crate::Resettable for AHBRST1_SPEC {}
