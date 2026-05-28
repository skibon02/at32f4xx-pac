#[doc = "Register `DMA_MUXSEL` reader"]
pub type R = crate::R<DMA_MUXSEL_SPEC>;
#[doc = "Register `DMA_MUXSEL` writer"]
pub type W = crate::W<DMA_MUXSEL_SPEC>;
#[doc = "Multiplexer Table Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBL_SEL_A {
    #[doc = "0: Normal table"]
    Normal = 0,
    #[doc = "1: Flexible mapping table"]
    Flexible = 1,
}
impl From<TBL_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: TBL_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBL_SEL` reader - Multiplexer Table Select"]
pub type TBL_SEL_R = crate::BitReader<TBL_SEL_A>;
impl TBL_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TBL_SEL_A {
        match self.bits {
            false => TBL_SEL_A::Normal,
            true => TBL_SEL_A::Flexible,
        }
    }
    #[doc = "Normal table"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TBL_SEL_A::Normal
    }
    #[doc = "Flexible mapping table"]
    #[inline(always)]
    pub fn is_flexible(&self) -> bool {
        *self == TBL_SEL_A::Flexible
    }
}
#[doc = "Field `TBL_SEL` writer - Multiplexer Table Select"]
pub type TBL_SEL_W<'a, REG> = crate::BitWriter<'a, REG, TBL_SEL_A>;
impl<'a, REG> TBL_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal table"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(TBL_SEL_A::Normal)
    }
    #[doc = "Flexible mapping table"]
    #[inline(always)]
    pub fn flexible(self) -> &'a mut crate::W<REG> {
        self.variant(TBL_SEL_A::Flexible)
    }
}
impl R {
    #[doc = "Bit 0 - Multiplexer Table Select"]
    #[inline(always)]
    pub fn tbl_sel(&self) -> TBL_SEL_R {
        TBL_SEL_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_MUXSEL")
            .field("tbl_sel", &self.tbl_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Multiplexer Table Select"]
    #[inline(always)]
    pub fn tbl_sel(&mut self) -> TBL_SEL_W<'_, DMA_MUXSEL_SPEC> {
        TBL_SEL_W::new(self, 0)
    }
}
#[doc = "DMAMUX Table Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_muxsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_muxsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_MUXSEL_SPEC;
impl crate::RegisterSpec for DMA_MUXSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_muxsel::R`](R) reader structure"]
impl crate::Readable for DMA_MUXSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_muxsel::W`](W) writer structure"]
impl crate::Writable for DMA_MUXSEL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_MUXSEL to value 0"]
impl crate::Resettable for DMA_MUXSEL_SPEC {}
