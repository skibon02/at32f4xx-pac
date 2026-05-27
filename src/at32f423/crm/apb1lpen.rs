#[doc = "Register `APB1LPEN` reader"]
pub type R = crate::R<APB1LPEN_SPEC>;
#[doc = "Register `APB1LPEN` writer"]
pub type W = crate::W<APB1LPEN_SPEC>;
#[doc = "Timer2 clock enable during sleep mode\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMR2LPEN_A {
    #[doc = "0: Disable peripheral clock"]
    Disable = 0,
    #[doc = "1: Enable peripheral clock"]
    Enable = 1,
}
impl From<TMR2LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TMR2LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR2` reader - Timer2 clock enable during sleep mode"]
pub type TMR2_R = crate::BitReader<TMR2LPEN_A>;
impl TMR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TMR2LPEN_A {
        match self.bits {
            false => TMR2LPEN_A::Disable,
            true => TMR2LPEN_A::Enable,
        }
    }
    #[doc = "Disable peripheral clock"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TMR2LPEN_A::Disable
    }
    #[doc = "Enable peripheral clock"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TMR2LPEN_A::Enable
    }
}
#[doc = "Field `TMR2` writer - Timer2 clock enable during sleep mode"]
pub type TMR2_W<'a, REG> = crate::BitWriter<'a, REG, TMR2LPEN_A>;
impl<'a, REG> TMR2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable peripheral clock"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TMR2LPEN_A::Disable)
    }
    #[doc = "Enable peripheral clock"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TMR2LPEN_A::Enable)
    }
}
#[doc = "Field `TMR3` reader - Timer3 clock enable during sleep mode"]
pub use TMR2_R as TMR3_R;
#[doc = "Field `TMR4` reader - Timer4 clock enable during sleep mode"]
pub use TMR2_R as TMR4_R;
#[doc = "Field `TMR6` reader - Timer6 clock enable during sleep mode"]
pub use TMR2_R as TMR6_R;
#[doc = "Field `TMR7` reader - Timer7 clock enable during sleep mode"]
pub use TMR2_R as TMR7_R;
#[doc = "Field `TMR12` reader - Timer12 clock enable during sleep mode"]
pub use TMR2_R as TMR12_R;
#[doc = "Field `TMR13` reader - Timer13 clock enable during sleep mode"]
pub use TMR2_R as TMR13_R;
#[doc = "Field `TMR14` reader - Timer14 clock enable during sleep mode"]
pub use TMR2_R as TMR14_R;
#[doc = "Field `WWDT` reader - WWDT clock enable during sleep mode"]
pub use TMR2_R as WWDT_R;
#[doc = "Field `SPI2` reader - SPI2 clock enable during sleep mode"]
pub use TMR2_R as SPI2_R;
#[doc = "Field `SPI3` reader - SPI3 clock enable during sleep mode"]
pub use TMR2_R as SPI3_R;
#[doc = "Field `USART2` reader - USART2 clock enable during sleep mode"]
pub use TMR2_R as USART2_R;
#[doc = "Field `USART3` reader - USART3 clock enable during sleep mode"]
pub use TMR2_R as USART3_R;
#[doc = "Field `USART4` reader - USART4 clock enable during sleep mode"]
pub use TMR2_R as USART4_R;
#[doc = "Field `USART5` reader - USART5 clock enable during sleep mode"]
pub use TMR2_R as USART5_R;
#[doc = "Field `I2C1CPEN` reader - I2C1 clock enable during sleep mode"]
pub use TMR2_R as I2C1CPEN_R;
#[doc = "Field `I2C2CPEN` reader - I2C2 clock enable during sleep mode"]
pub use TMR2_R as I2C2CPEN_R;
#[doc = "Field `I2C3CPEN` reader - I2C3 clock enable during sleep mode"]
pub use TMR2_R as I2C3CPEN_R;
#[doc = "Field `CAN1` reader - CAN1 clock enable during sleep mode"]
pub use TMR2_R as CAN1_R;
#[doc = "Field `CAN2` reader - CAN2 clock enable during sleep mode"]
pub use TMR2_R as CAN2_R;
#[doc = "Field `PWC` reader - PWC clock enable during sleep mode"]
pub use TMR2_R as PWC_R;
#[doc = "Field `DAC` reader - DAC clock enable during sleep mode"]
pub use TMR2_R as DAC_R;
#[doc = "Field `USART7` reader - USART7 clock enable during sleep mode"]
pub use TMR2_R as USART7_R;
#[doc = "Field `USART8` reader - USART8 clock enable during sleep mode"]
pub use TMR2_R as USART8_R;
#[doc = "Field `TMR3` writer - Timer3 clock enable during sleep mode"]
pub use TMR2_W as TMR3_W;
#[doc = "Field `TMR4` writer - Timer4 clock enable during sleep mode"]
pub use TMR2_W as TMR4_W;
#[doc = "Field `TMR6` writer - Timer6 clock enable during sleep mode"]
pub use TMR2_W as TMR6_W;
#[doc = "Field `TMR7` writer - Timer7 clock enable during sleep mode"]
pub use TMR2_W as TMR7_W;
#[doc = "Field `TMR12` writer - Timer12 clock enable during sleep mode"]
pub use TMR2_W as TMR12_W;
#[doc = "Field `TMR13` writer - Timer13 clock enable during sleep mode"]
pub use TMR2_W as TMR13_W;
#[doc = "Field `TMR14` writer - Timer14 clock enable during sleep mode"]
pub use TMR2_W as TMR14_W;
#[doc = "Field `WWDT` writer - WWDT clock enable during sleep mode"]
pub use TMR2_W as WWDT_W;
#[doc = "Field `SPI2` writer - SPI2 clock enable during sleep mode"]
pub use TMR2_W as SPI2_W;
#[doc = "Field `SPI3` writer - SPI3 clock enable during sleep mode"]
pub use TMR2_W as SPI3_W;
#[doc = "Field `USART2` writer - USART2 clock enable during sleep mode"]
pub use TMR2_W as USART2_W;
#[doc = "Field `USART3` writer - USART3 clock enable during sleep mode"]
pub use TMR2_W as USART3_W;
#[doc = "Field `USART4` writer - USART4 clock enable during sleep mode"]
pub use TMR2_W as USART4_W;
#[doc = "Field `USART5` writer - USART5 clock enable during sleep mode"]
pub use TMR2_W as USART5_W;
#[doc = "Field `I2C1CPEN` writer - I2C1 clock enable during sleep mode"]
pub use TMR2_W as I2C1CPEN_W;
#[doc = "Field `I2C2CPEN` writer - I2C2 clock enable during sleep mode"]
pub use TMR2_W as I2C2CPEN_W;
#[doc = "Field `I2C3CPEN` writer - I2C3 clock enable during sleep mode"]
pub use TMR2_W as I2C3CPEN_W;
#[doc = "Field `CAN1` writer - CAN1 clock enable during sleep mode"]
pub use TMR2_W as CAN1_W;
#[doc = "Field `CAN2` writer - CAN2 clock enable during sleep mode"]
pub use TMR2_W as CAN2_W;
#[doc = "Field `PWC` writer - PWC clock enable during sleep mode"]
pub use TMR2_W as PWC_W;
#[doc = "Field `DAC` writer - DAC clock enable during sleep mode"]
pub use TMR2_W as DAC_W;
#[doc = "Field `USART7` writer - USART7 clock enable during sleep mode"]
pub use TMR2_W as USART7_W;
#[doc = "Field `USART8` writer - USART8 clock enable during sleep mode"]
pub use TMR2_W as USART8_W;
impl R {
    #[doc = "Bit 0 - Timer2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr2(&self) -> TMR2_R {
        TMR2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer3 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr3(&self) -> TMR3_R {
        TMR3_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer4 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr4(&self) -> TMR4_R {
        TMR4_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer6 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr6(&self) -> TMR6_R {
        TMR6_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer7 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr7(&self) -> TMR7_R {
        TMR7_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer12 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr12(&self) -> TMR12_R {
        TMR12_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer13 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr13(&self) -> TMR13_R {
        TMR13_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer14 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr14(&self) -> TMR14_R {
        TMR14_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDT clock enable during sleep mode"]
    #[inline(always)]
    pub fn wwdt(&self) -> WWDT_R {
        WWDT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn spi2(&self) -> SPI2_R {
        SPI2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable during sleep mode"]
    #[inline(always)]
    pub fn spi3(&self) -> SPI3_R {
        SPI3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn usart2(&self) -> USART2_R {
        USART2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clock enable during sleep mode"]
    #[inline(always)]
    pub fn usart3(&self) -> USART3_R {
        USART3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USART4 clock enable during sleep mode"]
    #[inline(always)]
    pub fn usart4(&self) -> USART4_R {
        USART4_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - USART5 clock enable during sleep mode"]
    #[inline(always)]
    pub fn usart5(&self) -> USART5_R {
        USART5_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn i2c1cpen(&self) -> I2C1CPEN_R {
        I2C1CPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn i2c2cpen(&self) -> I2C2CPEN_R {
        I2C2CPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 clock enable during sleep mode"]
    #[inline(always)]
    pub fn i2c3cpen(&self) -> I2C3CPEN_R {
        I2C3CPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn can1(&self) -> CAN1_R {
        CAN1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CAN2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn can2(&self) -> CAN2_R {
        CAN2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - PWC clock enable during sleep mode"]
    #[inline(always)]
    pub fn pwc(&self) -> PWC_R {
        PWC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC clock enable during sleep mode"]
    #[inline(always)]
    pub fn dac(&self) -> DAC_R {
        DAC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - USART7 clock enable during sleep mode"]
    #[inline(always)]
    pub fn usart7(&self) -> USART7_R {
        USART7_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - USART8 clock enable during sleep mode"]
    #[inline(always)]
    pub fn usart8(&self) -> USART8_R {
        USART8_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1LPEN")
            .field("tmr2", &self.tmr2())
            .field("tmr3", &self.tmr3())
            .field("tmr4", &self.tmr4())
            .field("tmr6", &self.tmr6())
            .field("tmr7", &self.tmr7())
            .field("tmr12", &self.tmr12())
            .field("tmr13", &self.tmr13())
            .field("tmr14", &self.tmr14())
            .field("wwdt", &self.wwdt())
            .field("spi2", &self.spi2())
            .field("spi3", &self.spi3())
            .field("usart2", &self.usart2())
            .field("usart3", &self.usart3())
            .field("usart4", &self.usart4())
            .field("usart5", &self.usart5())
            .field("i2c1cpen", &self.i2c1cpen())
            .field("i2c2cpen", &self.i2c2cpen())
            .field("i2c3cpen", &self.i2c3cpen())
            .field("can1", &self.can1())
            .field("can2", &self.can2())
            .field("pwc", &self.pwc())
            .field("dac", &self.dac())
            .field("usart7", &self.usart7())
            .field("usart8", &self.usart8())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timer2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr2(&mut self) -> TMR2_W<'_, APB1LPEN_SPEC> {
        TMR2_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer3 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr3(&mut self) -> TMR3_W<'_, APB1LPEN_SPEC> {
        TMR3_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer4 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr4(&mut self) -> TMR4_W<'_, APB1LPEN_SPEC> {
        TMR4_W::new(self, 2)
    }
    #[doc = "Bit 4 - Timer6 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr6(&mut self) -> TMR6_W<'_, APB1LPEN_SPEC> {
        TMR6_W::new(self, 4)
    }
    #[doc = "Bit 5 - Timer7 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr7(&mut self) -> TMR7_W<'_, APB1LPEN_SPEC> {
        TMR7_W::new(self, 5)
    }
    #[doc = "Bit 6 - Timer12 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr12(&mut self) -> TMR12_W<'_, APB1LPEN_SPEC> {
        TMR12_W::new(self, 6)
    }
    #[doc = "Bit 7 - Timer13 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr13(&mut self) -> TMR13_W<'_, APB1LPEN_SPEC> {
        TMR13_W::new(self, 7)
    }
    #[doc = "Bit 8 - Timer14 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr14(&mut self) -> TMR14_W<'_, APB1LPEN_SPEC> {
        TMR14_W::new(self, 8)
    }
    #[doc = "Bit 11 - WWDT clock enable during sleep mode"]
    #[inline(always)]
    pub fn wwdt(&mut self) -> WWDT_W<'_, APB1LPEN_SPEC> {
        WWDT_W::new(self, 11)
    }
    #[doc = "Bit 14 - SPI2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn spi2(&mut self) -> SPI2_W<'_, APB1LPEN_SPEC> {
        SPI2_W::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 clock enable during sleep mode"]
    #[inline(always)]
    pub fn spi3(&mut self) -> SPI3_W<'_, APB1LPEN_SPEC> {
        SPI3_W::new(self, 15)
    }
    #[doc = "Bit 17 - USART2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn usart2(&mut self) -> USART2_W<'_, APB1LPEN_SPEC> {
        USART2_W::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 clock enable during sleep mode"]
    #[inline(always)]
    pub fn usart3(&mut self) -> USART3_W<'_, APB1LPEN_SPEC> {
        USART3_W::new(self, 18)
    }
    #[doc = "Bit 19 - USART4 clock enable during sleep mode"]
    #[inline(always)]
    pub fn usart4(&mut self) -> USART4_W<'_, APB1LPEN_SPEC> {
        USART4_W::new(self, 19)
    }
    #[doc = "Bit 20 - USART5 clock enable during sleep mode"]
    #[inline(always)]
    pub fn usart5(&mut self) -> USART5_W<'_, APB1LPEN_SPEC> {
        USART5_W::new(self, 20)
    }
    #[doc = "Bit 21 - I2C1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn i2c1cpen(&mut self) -> I2C1CPEN_W<'_, APB1LPEN_SPEC> {
        I2C1CPEN_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn i2c2cpen(&mut self) -> I2C2CPEN_W<'_, APB1LPEN_SPEC> {
        I2C2CPEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - I2C3 clock enable during sleep mode"]
    #[inline(always)]
    pub fn i2c3cpen(&mut self) -> I2C3CPEN_W<'_, APB1LPEN_SPEC> {
        I2C3CPEN_W::new(self, 23)
    }
    #[doc = "Bit 25 - CAN1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn can1(&mut self) -> CAN1_W<'_, APB1LPEN_SPEC> {
        CAN1_W::new(self, 25)
    }
    #[doc = "Bit 26 - CAN2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn can2(&mut self) -> CAN2_W<'_, APB1LPEN_SPEC> {
        CAN2_W::new(self, 26)
    }
    #[doc = "Bit 28 - PWC clock enable during sleep mode"]
    #[inline(always)]
    pub fn pwc(&mut self) -> PWC_W<'_, APB1LPEN_SPEC> {
        PWC_W::new(self, 28)
    }
    #[doc = "Bit 29 - DAC clock enable during sleep mode"]
    #[inline(always)]
    pub fn dac(&mut self) -> DAC_W<'_, APB1LPEN_SPEC> {
        DAC_W::new(self, 29)
    }
    #[doc = "Bit 30 - USART7 clock enable during sleep mode"]
    #[inline(always)]
    pub fn usart7(&mut self) -> USART7_W<'_, APB1LPEN_SPEC> {
        USART7_W::new(self, 30)
    }
    #[doc = "Bit 31 - USART8 clock enable during sleep mode"]
    #[inline(always)]
    pub fn usart8(&mut self) -> USART8_W<'_, APB1LPEN_SPEC> {
        USART8_W::new(self, 31)
    }
}
#[doc = "APB1 peripheral Low-power clock enable register (CRM_APB1LPEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1lpen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lpen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1LPEN_SPEC;
impl crate::RegisterSpec for APB1LPEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1lpen::R`](R) reader structure"]
impl crate::Readable for APB1LPEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb1lpen::W`](W) writer structure"]
impl crate::Writable for APB1LPEN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB1LPEN to value 0xf6fe_e9ff"]
impl crate::Resettable for APB1LPEN_SPEC {
    const RESET_VALUE: u32 = 0xf6fe_e9ff;
}
