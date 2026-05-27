#[doc = "Register `AHBLPEN1` reader"]
pub type R = crate::R<AHBLPEN1_SPEC>;
#[doc = "Register `AHBLPEN1` writer"]
pub type W = crate::W<AHBLPEN1_SPEC>;
#[doc = "IO A clock enable during sleep mode\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOALPEN_A {
    #[doc = "0: Disable peripheral clock"]
    Disable = 0,
    #[doc = "1: Enable peripheral clock"]
    Enable = 1,
}
impl From<GPIOALPEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOALPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOA` reader - IO A clock enable during sleep mode"]
pub type GPIOA_R = crate::BitReader<GPIOALPEN_A>;
impl GPIOA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPIOALPEN_A {
        match self.bits {
            false => GPIOALPEN_A::Disable,
            true => GPIOALPEN_A::Enable,
        }
    }
    #[doc = "Disable peripheral clock"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIOALPEN_A::Disable
    }
    #[doc = "Enable peripheral clock"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIOALPEN_A::Enable
    }
}
#[doc = "Field `GPIOA` writer - IO A clock enable during sleep mode"]
pub type GPIOA_W<'a, REG> = crate::BitWriter<'a, REG, GPIOALPEN_A>;
impl<'a, REG> GPIOA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable peripheral clock"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOALPEN_A::Disable)
    }
    #[doc = "Enable peripheral clock"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOALPEN_A::Enable)
    }
}
#[doc = "Field `GPIOB` reader - IO B clock enable during sleep mode"]
pub use GPIOA_R as GPIOB_R;
#[doc = "Field `GPIOC` reader - IO C clock enable during sleep mode"]
pub use GPIOA_R as GPIOC_R;
#[doc = "Field `GPIOD` reader - IO D clock enable during sleep mode"]
pub use GPIOA_R as GPIOD_R;
#[doc = "Field `GPIOE` reader - IO E clock enable during sleep mode"]
pub use GPIOA_R as GPIOE_R;
#[doc = "Field `GPIOF` reader - IO F clock enable during sleep mode"]
pub use GPIOA_R as GPIOF_R;
#[doc = "Field `GPIOG` reader - IO G clock enable during sleep mode"]
pub use GPIOA_R as GPIOG_R;
#[doc = "Field `GPIOH` reader - IO H clock enable during sleep mode"]
pub use GPIOA_R as GPIOH_R;
#[doc = "Field `CRC` reader - CRC clock enable during sleep mode"]
pub use GPIOA_R as CRC_R;
#[doc = "Field `FLASH` reader - Flash clock enable during sleep mode"]
pub use GPIOA_R as FLASH_R;
#[doc = "Field `SRAM1` reader - SRAM1 clock enable during sleep mode"]
pub use GPIOA_R as SRAM1_R;
#[doc = "Field `SRAM2` reader - SRAM2 clock enable during sleep mode"]
pub use GPIOA_R as SRAM2_R;
#[doc = "Field `EDMA` reader - EDMA clock enable during sleep mode"]
pub use GPIOA_R as EDMA_R;
#[doc = "Field `DMA1` reader - DMA1 clock enable during sleep mode"]
pub use GPIOA_R as DMA1_R;
#[doc = "Field `DMA2` reader - DMA2 clock enable during sleep mode"]
pub use GPIOA_R as DMA2_R;
#[doc = "Field `EMAC` reader - EMAC clock enable during sleep mode"]
pub use GPIOA_R as EMAC_R;
#[doc = "Field `EMACTX` reader - EMAC Tx clock enable during sleep mode"]
pub use GPIOA_R as EMACTX_R;
#[doc = "Field `EMACRX` reader - EMAC Rx clock enable during sleep mode"]
pub use GPIOA_R as EMACRX_R;
#[doc = "Field `EMACPTP` reader - EMAC PTP clock enable during sleep mode"]
pub use GPIOA_R as EMACPTP_R;
#[doc = "Field `OTGFS2` reader - OTGFS2 clock enable during sleep mode"]
pub use GPIOA_R as OTGFS2_R;
#[doc = "Field `GPIOB` writer - IO B clock enable during sleep mode"]
pub use GPIOA_W as GPIOB_W;
#[doc = "Field `GPIOC` writer - IO C clock enable during sleep mode"]
pub use GPIOA_W as GPIOC_W;
#[doc = "Field `GPIOD` writer - IO D clock enable during sleep mode"]
pub use GPIOA_W as GPIOD_W;
#[doc = "Field `GPIOE` writer - IO E clock enable during sleep mode"]
pub use GPIOA_W as GPIOE_W;
#[doc = "Field `GPIOF` writer - IO F clock enable during sleep mode"]
pub use GPIOA_W as GPIOF_W;
#[doc = "Field `GPIOG` writer - IO G clock enable during sleep mode"]
pub use GPIOA_W as GPIOG_W;
#[doc = "Field `GPIOH` writer - IO H clock enable during sleep mode"]
pub use GPIOA_W as GPIOH_W;
#[doc = "Field `CRC` writer - CRC clock enable during sleep mode"]
pub use GPIOA_W as CRC_W;
#[doc = "Field `FLASH` writer - Flash clock enable during sleep mode"]
pub use GPIOA_W as FLASH_W;
#[doc = "Field `SRAM1` writer - SRAM1 clock enable during sleep mode"]
pub use GPIOA_W as SRAM1_W;
#[doc = "Field `SRAM2` writer - SRAM2 clock enable during sleep mode"]
pub use GPIOA_W as SRAM2_W;
#[doc = "Field `EDMA` writer - EDMA clock enable during sleep mode"]
pub use GPIOA_W as EDMA_W;
#[doc = "Field `DMA1` writer - DMA1 clock enable during sleep mode"]
pub use GPIOA_W as DMA1_W;
#[doc = "Field `DMA2` writer - DMA2 clock enable during sleep mode"]
pub use GPIOA_W as DMA2_W;
#[doc = "Field `EMAC` writer - EMAC clock enable during sleep mode"]
pub use GPIOA_W as EMAC_W;
#[doc = "Field `EMACTX` writer - EMAC Tx clock enable during sleep mode"]
pub use GPIOA_W as EMACTX_W;
#[doc = "Field `EMACRX` writer - EMAC Rx clock enable during sleep mode"]
pub use GPIOA_W as EMACRX_W;
#[doc = "Field `EMACPTP` writer - EMAC PTP clock enable during sleep mode"]
pub use GPIOA_W as EMACPTP_W;
#[doc = "Field `OTGFS2` writer - OTGFS2 clock enable during sleep mode"]
pub use GPIOA_W as OTGFS2_W;
impl R {
    #[doc = "Bit 0 - IO A clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioa(&self) -> GPIOA_R {
        GPIOA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO B clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpiob(&self) -> GPIOB_R {
        GPIOB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO C clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioc(&self) -> GPIOC_R {
        GPIOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO D clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpiod(&self) -> GPIOD_R {
        GPIOD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO E clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioe(&self) -> GPIOE_R {
        GPIOE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO F clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpiof(&self) -> GPIOF_R {
        GPIOF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO G clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpiog(&self) -> GPIOG_R {
        GPIOG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO H clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioh(&self) -> GPIOH_R {
        GPIOH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable during sleep mode"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Flash clock enable during sleep mode"]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SRAM1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn sram1(&self) -> SRAM1_R {
        SRAM1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SRAM2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn sram2(&self) -> SRAM2_R {
        SRAM2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - EDMA clock enable during sleep mode"]
    #[inline(always)]
    pub fn edma(&self) -> EDMA_R {
        EDMA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn dma1(&self) -> DMA1_R {
        DMA1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - DMA2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn dma2(&self) -> DMA2_R {
        DMA2_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - EMAC clock enable during sleep mode"]
    #[inline(always)]
    pub fn emac(&self) -> EMAC_R {
        EMAC_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - EMAC Tx clock enable during sleep mode"]
    #[inline(always)]
    pub fn emactx(&self) -> EMACTX_R {
        EMACTX_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - EMAC Rx clock enable during sleep mode"]
    #[inline(always)]
    pub fn emacrx(&self) -> EMACRX_R {
        EMACRX_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - EMAC PTP clock enable during sleep mode"]
    #[inline(always)]
    pub fn emacptp(&self) -> EMACPTP_R {
        EMACPTP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - OTGFS2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn otgfs2(&self) -> OTGFS2_R {
        OTGFS2_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBLPEN1")
            .field("gpioa", &self.gpioa())
            .field("gpiob", &self.gpiob())
            .field("gpioc", &self.gpioc())
            .field("gpiod", &self.gpiod())
            .field("gpioe", &self.gpioe())
            .field("gpiof", &self.gpiof())
            .field("gpiog", &self.gpiog())
            .field("gpioh", &self.gpioh())
            .field("crc", &self.crc())
            .field("flash", &self.flash())
            .field("sram1", &self.sram1())
            .field("sram2", &self.sram2())
            .field("edma", &self.edma())
            .field("dma1", &self.dma1())
            .field("dma2", &self.dma2())
            .field("emac", &self.emac())
            .field("emactx", &self.emactx())
            .field("emacrx", &self.emacrx())
            .field("emacptp", &self.emacptp())
            .field("otgfs2", &self.otgfs2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - IO A clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioa(&mut self) -> GPIOA_W<'_, AHBLPEN1_SPEC> {
        GPIOA_W::new(self, 0)
    }
    #[doc = "Bit 1 - IO B clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpiob(&mut self) -> GPIOB_W<'_, AHBLPEN1_SPEC> {
        GPIOB_W::new(self, 1)
    }
    #[doc = "Bit 2 - IO C clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioc(&mut self) -> GPIOC_W<'_, AHBLPEN1_SPEC> {
        GPIOC_W::new(self, 2)
    }
    #[doc = "Bit 3 - IO D clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpiod(&mut self) -> GPIOD_W<'_, AHBLPEN1_SPEC> {
        GPIOD_W::new(self, 3)
    }
    #[doc = "Bit 4 - IO E clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioe(&mut self) -> GPIOE_W<'_, AHBLPEN1_SPEC> {
        GPIOE_W::new(self, 4)
    }
    #[doc = "Bit 5 - IO F clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpiof(&mut self) -> GPIOF_W<'_, AHBLPEN1_SPEC> {
        GPIOF_W::new(self, 5)
    }
    #[doc = "Bit 6 - IO G clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpiog(&mut self) -> GPIOG_W<'_, AHBLPEN1_SPEC> {
        GPIOG_W::new(self, 6)
    }
    #[doc = "Bit 7 - IO H clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioh(&mut self) -> GPIOH_W<'_, AHBLPEN1_SPEC> {
        GPIOH_W::new(self, 7)
    }
    #[doc = "Bit 12 - CRC clock enable during sleep mode"]
    #[inline(always)]
    pub fn crc(&mut self) -> CRC_W<'_, AHBLPEN1_SPEC> {
        CRC_W::new(self, 12)
    }
    #[doc = "Bit 15 - Flash clock enable during sleep mode"]
    #[inline(always)]
    pub fn flash(&mut self) -> FLASH_W<'_, AHBLPEN1_SPEC> {
        FLASH_W::new(self, 15)
    }
    #[doc = "Bit 16 - SRAM1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn sram1(&mut self) -> SRAM1_W<'_, AHBLPEN1_SPEC> {
        SRAM1_W::new(self, 16)
    }
    #[doc = "Bit 17 - SRAM2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn sram2(&mut self) -> SRAM2_W<'_, AHBLPEN1_SPEC> {
        SRAM2_W::new(self, 17)
    }
    #[doc = "Bit 21 - EDMA clock enable during sleep mode"]
    #[inline(always)]
    pub fn edma(&mut self) -> EDMA_W<'_, AHBLPEN1_SPEC> {
        EDMA_W::new(self, 21)
    }
    #[doc = "Bit 22 - DMA1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn dma1(&mut self) -> DMA1_W<'_, AHBLPEN1_SPEC> {
        DMA1_W::new(self, 22)
    }
    #[doc = "Bit 24 - DMA2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn dma2(&mut self) -> DMA2_W<'_, AHBLPEN1_SPEC> {
        DMA2_W::new(self, 24)
    }
    #[doc = "Bit 25 - EMAC clock enable during sleep mode"]
    #[inline(always)]
    pub fn emac(&mut self) -> EMAC_W<'_, AHBLPEN1_SPEC> {
        EMAC_W::new(self, 25)
    }
    #[doc = "Bit 26 - EMAC Tx clock enable during sleep mode"]
    #[inline(always)]
    pub fn emactx(&mut self) -> EMACTX_W<'_, AHBLPEN1_SPEC> {
        EMACTX_W::new(self, 26)
    }
    #[doc = "Bit 27 - EMAC Rx clock enable during sleep mode"]
    #[inline(always)]
    pub fn emacrx(&mut self) -> EMACRX_W<'_, AHBLPEN1_SPEC> {
        EMACRX_W::new(self, 27)
    }
    #[doc = "Bit 28 - EMAC PTP clock enable during sleep mode"]
    #[inline(always)]
    pub fn emacptp(&mut self) -> EMACPTP_W<'_, AHBLPEN1_SPEC> {
        EMACPTP_W::new(self, 28)
    }
    #[doc = "Bit 29 - OTGFS2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn otgfs2(&mut self) -> OTGFS2_W<'_, AHBLPEN1_SPEC> {
        OTGFS2_W::new(self, 29)
    }
}
#[doc = "AHB Low-power Peripheral Clock enable register 1 (CRM_AHBLPEN1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ahblpen1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahblpen1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBLPEN1_SPEC;
impl crate::RegisterSpec for AHBLPEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahblpen1::R`](R) reader structure"]
impl crate::Readable for AHBLPEN1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahblpen1::W`](W) writer structure"]
impl crate::Writable for AHBLPEN1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBLPEN1 to value 0x3e63_90ff"]
impl crate::Resettable for AHBLPEN1_SPEC {
    const RESET_VALUE: u32 = 0x3e63_90ff;
}
