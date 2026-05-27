#[doc = "Register `APB2RST` reader"]
pub type R = crate::R<APB2RST_SPEC>;
#[doc = "Register `APB2RST` writer"]
pub type W = crate::W<APB2RST_SPEC>;
#[doc = "Timer1 reset\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMR1W_A {
    #[doc = "1: Reset peripheral"]
    Reset = 1,
}
impl From<TMR1W_A> for bool {
    #[inline(always)]
    fn from(variant: TMR1W_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR1` reader - Timer1 reset"]
pub type TMR1_R = crate::BitReader<TMR1W_A>;
impl TMR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TMR1W_A> {
        match self.bits {
            true => Some(TMR1W_A::Reset),
            _ => None,
        }
    }
    #[doc = "Reset peripheral"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TMR1W_A::Reset
    }
}
#[doc = "Field `TMR1` writer - Timer1 reset"]
pub type TMR1_W<'a, REG> = crate::BitWriter1S<'a, REG, TMR1W_A>;
impl<'a, REG> TMR1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset peripheral"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(TMR1W_A::Reset)
    }
}
#[doc = "Field `USART1` reader - USART1 reset"]
pub use TMR1_R as USART1_R;
#[doc = "Field `USART6` reader - USART6 reset"]
pub use TMR1_R as USART6_R;
#[doc = "Field `ADC` reader - ADC reset"]
pub use TMR1_R as ADC_R;
#[doc = "Field `SPI1` reader - SPI1 reset"]
pub use TMR1_R as SPI1_R;
#[doc = "Field `SCFG` reader - SCFG reset"]
pub use TMR1_R as SCFG_R;
#[doc = "Field `TMR9` reader - Timer9 reset"]
pub use TMR1_R as TMR9_R;
#[doc = "Field `TMR10` reader - Timer10 reset"]
pub use TMR1_R as TMR10_R;
#[doc = "Field `TMR11` reader - Timer 11 reset"]
pub use TMR1_R as TMR11_R;
#[doc = "Field `ACC` reader - ACC reset"]
pub use TMR1_R as ACC_R;
#[doc = "Field `USART1` writer - USART1 reset"]
pub use TMR1_W as USART1_W;
#[doc = "Field `USART6` writer - USART6 reset"]
pub use TMR1_W as USART6_W;
#[doc = "Field `ADC` writer - ADC reset"]
pub use TMR1_W as ADC_W;
#[doc = "Field `SPI1` writer - SPI1 reset"]
pub use TMR1_W as SPI1_W;
#[doc = "Field `SCFG` writer - SCFG reset"]
pub use TMR1_W as SCFG_W;
#[doc = "Field `TMR9` writer - Timer9 reset"]
pub use TMR1_W as TMR9_W;
#[doc = "Field `TMR10` writer - Timer10 reset"]
pub use TMR1_W as TMR10_W;
#[doc = "Field `TMR11` writer - Timer 11 reset"]
pub use TMR1_W as TMR11_W;
#[doc = "Field `ACC` writer - ACC reset"]
pub use TMR1_W as ACC_W;
impl R {
    #[doc = "Bit 0 - Timer1 reset"]
    #[inline(always)]
    pub fn tmr1(&self) -> TMR1_R {
        TMR1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - USART1 reset"]
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART6 reset"]
    #[inline(always)]
    pub fn usart6(&self) -> USART6_R {
        USART6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC reset"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - SCFG reset"]
    #[inline(always)]
    pub fn scfg(&self) -> SCFG_R {
        SCFG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer9 reset"]
    #[inline(always)]
    pub fn tmr9(&self) -> TMR9_R {
        TMR9_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer10 reset"]
    #[inline(always)]
    pub fn tmr10(&self) -> TMR10_R {
        TMR10_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer 11 reset"]
    #[inline(always)]
    pub fn tmr11(&self) -> TMR11_R {
        TMR11_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 29 - ACC reset"]
    #[inline(always)]
    pub fn acc(&self) -> ACC_R {
        ACC_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2RST")
            .field("tmr1", &self.tmr1())
            .field("usart1", &self.usart1())
            .field("usart6", &self.usart6())
            .field("adc", &self.adc())
            .field("spi1", &self.spi1())
            .field("scfg", &self.scfg())
            .field("tmr9", &self.tmr9())
            .field("tmr10", &self.tmr10())
            .field("tmr11", &self.tmr11())
            .field("acc", &self.acc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timer1 reset"]
    #[inline(always)]
    pub fn tmr1(&mut self) -> TMR1_W<'_, APB2RST_SPEC> {
        TMR1_W::new(self, 0)
    }
    #[doc = "Bit 4 - USART1 reset"]
    #[inline(always)]
    pub fn usart1(&mut self) -> USART1_W<'_, APB2RST_SPEC> {
        USART1_W::new(self, 4)
    }
    #[doc = "Bit 5 - USART6 reset"]
    #[inline(always)]
    pub fn usart6(&mut self) -> USART6_W<'_, APB2RST_SPEC> {
        USART6_W::new(self, 5)
    }
    #[doc = "Bit 8 - ADC reset"]
    #[inline(always)]
    pub fn adc(&mut self) -> ADC_W<'_, APB2RST_SPEC> {
        ADC_W::new(self, 8)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1(&mut self) -> SPI1_W<'_, APB2RST_SPEC> {
        SPI1_W::new(self, 12)
    }
    #[doc = "Bit 14 - SCFG reset"]
    #[inline(always)]
    pub fn scfg(&mut self) -> SCFG_W<'_, APB2RST_SPEC> {
        SCFG_W::new(self, 14)
    }
    #[doc = "Bit 16 - Timer9 reset"]
    #[inline(always)]
    pub fn tmr9(&mut self) -> TMR9_W<'_, APB2RST_SPEC> {
        TMR9_W::new(self, 16)
    }
    #[doc = "Bit 17 - Timer10 reset"]
    #[inline(always)]
    pub fn tmr10(&mut self) -> TMR10_W<'_, APB2RST_SPEC> {
        TMR10_W::new(self, 17)
    }
    #[doc = "Bit 18 - Timer 11 reset"]
    #[inline(always)]
    pub fn tmr11(&mut self) -> TMR11_W<'_, APB2RST_SPEC> {
        TMR11_W::new(self, 18)
    }
    #[doc = "Bit 29 - ACC reset"]
    #[inline(always)]
    pub fn acc(&mut self) -> ACC_W<'_, APB2RST_SPEC> {
        ACC_W::new(self, 29)
    }
}
#[doc = "APB2 peripheral reset register (CRM_APB2RST)\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2rst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2RST_SPEC;
impl crate::RegisterSpec for APB2RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2rst::R`](R) reader structure"]
impl crate::Readable for APB2RST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2rst::W`](W) writer structure"]
impl crate::Writable for APB2RST_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x2007_5131;
}
#[doc = "`reset()` method sets APB2RST to value 0"]
impl crate::Resettable for APB2RST_SPEC {}
