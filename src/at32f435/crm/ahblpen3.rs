#[doc = "Register `AHBLPEN3` reader"]
pub type R = crate::R<AHBLPEN3_SPEC>;
#[doc = "Register `AHBLPEN3` writer"]
pub type W = crate::W<AHBLPEN3_SPEC>;
#[doc = "XMC clock enable during sleep mode\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XMCLPEN_A {
    #[doc = "0: Disable peripheral clock"]
    Disable = 0,
    #[doc = "1: Enable peripheral clock"]
    Enable = 1,
}
impl From<XMCLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: XMCLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XMC` reader - XMC clock enable during sleep mode"]
pub type XMC_R = crate::BitReader<XMCLPEN_A>;
impl XMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> XMCLPEN_A {
        match self.bits {
            false => XMCLPEN_A::Disable,
            true => XMCLPEN_A::Enable,
        }
    }
    #[doc = "Disable peripheral clock"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == XMCLPEN_A::Disable
    }
    #[doc = "Enable peripheral clock"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == XMCLPEN_A::Enable
    }
}
#[doc = "Field `XMC` writer - XMC clock enable during sleep mode"]
pub type XMC_W<'a, REG> = crate::BitWriter<'a, REG, XMCLPEN_A>;
impl<'a, REG> XMC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable peripheral clock"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(XMCLPEN_A::Disable)
    }
    #[doc = "Enable peripheral clock"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(XMCLPEN_A::Enable)
    }
}
#[doc = "Field `QSPI1` reader - QSPI1 clock enable during sleep mode"]
pub use XMC_R as QSPI1_R;
#[doc = "Field `QSPI2` reader - QSPI2 clock enable during sleep mode"]
pub use XMC_R as QSPI2_R;
#[doc = "Field `SDIO2` reader - SDIO2 clock enable during sleep mode"]
pub use XMC_R as SDIO2_R;
#[doc = "Field `QSPI1` writer - QSPI1 clock enable during sleep mode"]
pub use XMC_W as QSPI1_W;
#[doc = "Field `QSPI2` writer - QSPI2 clock enable during sleep mode"]
pub use XMC_W as QSPI2_W;
#[doc = "Field `SDIO2` writer - SDIO2 clock enable during sleep mode"]
pub use XMC_W as SDIO2_W;
impl R {
    #[doc = "Bit 0 - XMC clock enable during sleep mode"]
    #[inline(always)]
    pub fn xmc(&self) -> XMC_R {
        XMC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - QSPI1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn qspi1(&self) -> QSPI1_R {
        QSPI1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 14 - QSPI2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn qspi2(&self) -> QSPI2_R {
        QSPI2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SDIO2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn sdio2(&self) -> SDIO2_R {
        SDIO2_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBLPEN3")
            .field("xmc", &self.xmc())
            .field("qspi1", &self.qspi1())
            .field("qspi2", &self.qspi2())
            .field("sdio2", &self.sdio2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - XMC clock enable during sleep mode"]
    #[inline(always)]
    pub fn xmc(&mut self) -> XMC_W<'_, AHBLPEN3_SPEC> {
        XMC_W::new(self, 0)
    }
    #[doc = "Bit 1 - QSPI1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn qspi1(&mut self) -> QSPI1_W<'_, AHBLPEN3_SPEC> {
        QSPI1_W::new(self, 1)
    }
    #[doc = "Bit 14 - QSPI2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn qspi2(&mut self) -> QSPI2_W<'_, AHBLPEN3_SPEC> {
        QSPI2_W::new(self, 14)
    }
    #[doc = "Bit 15 - SDIO2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn sdio2(&mut self) -> SDIO2_W<'_, AHBLPEN3_SPEC> {
        SDIO2_W::new(self, 15)
    }
}
#[doc = "AHB peripheral Low-power clock enable register 3 (CRM_AHBLPEN3)\n\nYou can [`read`](crate::Reg::read) this register and get [`ahblpen3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahblpen3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBLPEN3_SPEC;
impl crate::RegisterSpec for AHBLPEN3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahblpen3::R`](R) reader structure"]
impl crate::Readable for AHBLPEN3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahblpen3::W`](W) writer structure"]
impl crate::Writable for AHBLPEN3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBLPEN3 to value 0xc003"]
impl crate::Resettable for AHBLPEN3_SPEC {
    const RESET_VALUE: u32 = 0xc003;
}
