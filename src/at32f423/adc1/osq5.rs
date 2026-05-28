#[doc = "Register `OSQ5` reader"]
pub type R = crate::R<OSQ5_SPEC>;
#[doc = "Register `OSQ5` writer"]
pub type W = crate::W<OSQ5_SPEC>;
#[doc = "Number of 23st conversion in ordinary sequence\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_CHANNEL_A {
    #[doc = "0: ADC_IN0"]
    AdcIn0 = 0,
    #[doc = "1: ADC_IN1"]
    AdcIn1 = 1,
    #[doc = "2: ADC_IN2"]
    AdcIn2 = 2,
    #[doc = "3: ADC_IN3"]
    AdcIn3 = 3,
    #[doc = "4: ADC_IN4"]
    AdcIn4 = 4,
    #[doc = "5: ADC_IN5"]
    AdcIn5 = 5,
    #[doc = "6: ADC_IN6"]
    AdcIn6 = 6,
    #[doc = "7: ADC_IN7"]
    AdcIn7 = 7,
    #[doc = "8: ADC_IN8"]
    AdcIn8 = 8,
    #[doc = "9: ADC_IN9"]
    AdcIn9 = 9,
    #[doc = "10: ADC_IN10"]
    AdcIn10 = 10,
    #[doc = "11: ADC_IN11"]
    AdcIn11 = 11,
    #[doc = "12: ADC_IN12"]
    AdcIn12 = 12,
    #[doc = "13: ADC_IN13"]
    AdcIn13 = 13,
    #[doc = "14: ADC_IN14"]
    AdcIn14 = 14,
    #[doc = "15: ADC_IN15"]
    AdcIn15 = 15,
    #[doc = "16: ADC_IN16"]
    AdcIn16 = 16,
    #[doc = "17: ADC_IN17"]
    AdcIn17 = 17,
}
impl From<ADC_CHANNEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_CHANNEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC_CHANNEL_A {
    type Ux = u8;
}
impl crate::IsEnum for ADC_CHANNEL_A {}
#[doc = "Field `OSN23` reader - Number of 23st conversion in ordinary sequence"]
pub type OSN23_R = crate::FieldReader<ADC_CHANNEL_A>;
impl OSN23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADC_CHANNEL_A> {
        match self.bits {
            0 => Some(ADC_CHANNEL_A::AdcIn0),
            1 => Some(ADC_CHANNEL_A::AdcIn1),
            2 => Some(ADC_CHANNEL_A::AdcIn2),
            3 => Some(ADC_CHANNEL_A::AdcIn3),
            4 => Some(ADC_CHANNEL_A::AdcIn4),
            5 => Some(ADC_CHANNEL_A::AdcIn5),
            6 => Some(ADC_CHANNEL_A::AdcIn6),
            7 => Some(ADC_CHANNEL_A::AdcIn7),
            8 => Some(ADC_CHANNEL_A::AdcIn8),
            9 => Some(ADC_CHANNEL_A::AdcIn9),
            10 => Some(ADC_CHANNEL_A::AdcIn10),
            11 => Some(ADC_CHANNEL_A::AdcIn11),
            12 => Some(ADC_CHANNEL_A::AdcIn12),
            13 => Some(ADC_CHANNEL_A::AdcIn13),
            14 => Some(ADC_CHANNEL_A::AdcIn14),
            15 => Some(ADC_CHANNEL_A::AdcIn15),
            16 => Some(ADC_CHANNEL_A::AdcIn16),
            17 => Some(ADC_CHANNEL_A::AdcIn17),
            _ => None,
        }
    }
    #[doc = "ADC_IN0"]
    #[inline(always)]
    pub fn is_adc_in0(&self) -> bool {
        *self == ADC_CHANNEL_A::AdcIn0
    }
    #[doc = "ADC_IN1"]
    #[inline(always)]
    pub fn is_adc_in1(&self) -> bool {
        *self == ADC_CHANNEL_A::AdcIn1
    }
    #[doc = "ADC_IN2"]
    #[inline(always)]
    pub fn is_adc_in2(&self) -> bool {
        *self == ADC_CHANNEL_A::AdcIn2
    }
    #[doc = "ADC_IN3"]
    #[inline(always)]
    pub fn is_adc_in3(&self) -> bool {
        *self == ADC_CHANNEL_A::AdcIn3
    }
    #[doc = "ADC_IN4"]
    #[inline(always)]
    pub fn is_adc_in4(&self) -> bool {
        *self == ADC_CHANNEL_A::AdcIn4
    }
    #[doc = "ADC_IN5"]
    #[inline(always)]
    pub fn is_adc_in5(&self) -> bool {
        *self == ADC_CHANNEL_A::AdcIn5
    }
    #[doc = "ADC_IN6"]
    #[inline(always)]
    pub fn is_adc_in6(&self) -> bool {
        *self == ADC_CHANNEL_A::AdcIn6
    }
    #[doc = "ADC_IN7"]
    #[inline(always)]
    pub fn is_adc_in7(&self) -> bool {
        *self == ADC_CHANNEL_A::AdcIn7
    }
    #[doc = "ADC_IN8"]
    #[inline(always)]
    pub fn is_adc_in8(&self) -> bool {
        *self == ADC_CHANNEL_A::AdcIn8
    }
    #[doc = "ADC_IN9"]
    #[inline(always)]
    pub fn is_adc_in9(&self) -> bool {
        *self == ADC_CHANNEL_A::AdcIn9
    }
    #[doc = "ADC_IN10"]
    #[inline(always)]
    pub fn is_adc_in10(&self) -> bool {
        *self == ADC_CHANNEL_A::AdcIn10
    }
    #[doc = "ADC_IN11"]
    #[inline(always)]
    pub fn is_adc_in11(&self) -> bool {
        *self == ADC_CHANNEL_A::AdcIn11
    }
    #[doc = "ADC_IN12"]
    #[inline(always)]
    pub fn is_adc_in12(&self) -> bool {
        *self == ADC_CHANNEL_A::AdcIn12
    }
    #[doc = "ADC_IN13"]
    #[inline(always)]
    pub fn is_adc_in13(&self) -> bool {
        *self == ADC_CHANNEL_A::AdcIn13
    }
    #[doc = "ADC_IN14"]
    #[inline(always)]
    pub fn is_adc_in14(&self) -> bool {
        *self == ADC_CHANNEL_A::AdcIn14
    }
    #[doc = "ADC_IN15"]
    #[inline(always)]
    pub fn is_adc_in15(&self) -> bool {
        *self == ADC_CHANNEL_A::AdcIn15
    }
    #[doc = "ADC_IN16"]
    #[inline(always)]
    pub fn is_adc_in16(&self) -> bool {
        *self == ADC_CHANNEL_A::AdcIn16
    }
    #[doc = "ADC_IN17"]
    #[inline(always)]
    pub fn is_adc_in17(&self) -> bool {
        *self == ADC_CHANNEL_A::AdcIn17
    }
}
#[doc = "Field `OSN23` writer - Number of 23st conversion in ordinary sequence"]
pub type OSN23_W<'a, REG> = crate::FieldWriter<'a, REG, 5, ADC_CHANNEL_A>;
impl<'a, REG> OSN23_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC_IN0"]
    #[inline(always)]
    pub fn adc_in0(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_CHANNEL_A::AdcIn0)
    }
    #[doc = "ADC_IN1"]
    #[inline(always)]
    pub fn adc_in1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_CHANNEL_A::AdcIn1)
    }
    #[doc = "ADC_IN2"]
    #[inline(always)]
    pub fn adc_in2(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_CHANNEL_A::AdcIn2)
    }
    #[doc = "ADC_IN3"]
    #[inline(always)]
    pub fn adc_in3(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_CHANNEL_A::AdcIn3)
    }
    #[doc = "ADC_IN4"]
    #[inline(always)]
    pub fn adc_in4(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_CHANNEL_A::AdcIn4)
    }
    #[doc = "ADC_IN5"]
    #[inline(always)]
    pub fn adc_in5(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_CHANNEL_A::AdcIn5)
    }
    #[doc = "ADC_IN6"]
    #[inline(always)]
    pub fn adc_in6(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_CHANNEL_A::AdcIn6)
    }
    #[doc = "ADC_IN7"]
    #[inline(always)]
    pub fn adc_in7(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_CHANNEL_A::AdcIn7)
    }
    #[doc = "ADC_IN8"]
    #[inline(always)]
    pub fn adc_in8(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_CHANNEL_A::AdcIn8)
    }
    #[doc = "ADC_IN9"]
    #[inline(always)]
    pub fn adc_in9(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_CHANNEL_A::AdcIn9)
    }
    #[doc = "ADC_IN10"]
    #[inline(always)]
    pub fn adc_in10(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_CHANNEL_A::AdcIn10)
    }
    #[doc = "ADC_IN11"]
    #[inline(always)]
    pub fn adc_in11(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_CHANNEL_A::AdcIn11)
    }
    #[doc = "ADC_IN12"]
    #[inline(always)]
    pub fn adc_in12(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_CHANNEL_A::AdcIn12)
    }
    #[doc = "ADC_IN13"]
    #[inline(always)]
    pub fn adc_in13(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_CHANNEL_A::AdcIn13)
    }
    #[doc = "ADC_IN14"]
    #[inline(always)]
    pub fn adc_in14(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_CHANNEL_A::AdcIn14)
    }
    #[doc = "ADC_IN15"]
    #[inline(always)]
    pub fn adc_in15(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_CHANNEL_A::AdcIn15)
    }
    #[doc = "ADC_IN16"]
    #[inline(always)]
    pub fn adc_in16(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_CHANNEL_A::AdcIn16)
    }
    #[doc = "ADC_IN17"]
    #[inline(always)]
    pub fn adc_in17(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_CHANNEL_A::AdcIn17)
    }
}
#[doc = "Field `OSN24` reader - Number of 24nd conversion in ordinary sequence"]
pub use OSN23_R as OSN24_R;
#[doc = "Field `OSN25` reader - number of 25rd conversion in ordinary sequence"]
pub use OSN23_R as OSN25_R;
#[doc = "Field `OSN26` reader - Number of 26th conversion in ordinary sequence"]
pub use OSN23_R as OSN26_R;
#[doc = "Field `OSN27` reader - Number of 27th conversion in ordinary sequence"]
pub use OSN23_R as OSN27_R;
#[doc = "Field `OSN28` reader - Number of 28th conversion in ordinary sequence"]
pub use OSN23_R as OSN28_R;
#[doc = "Field `OSN24` writer - Number of 24nd conversion in ordinary sequence"]
pub use OSN23_W as OSN24_W;
#[doc = "Field `OSN25` writer - number of 25rd conversion in ordinary sequence"]
pub use OSN23_W as OSN25_W;
#[doc = "Field `OSN26` writer - Number of 26th conversion in ordinary sequence"]
pub use OSN23_W as OSN26_W;
#[doc = "Field `OSN27` writer - Number of 27th conversion in ordinary sequence"]
pub use OSN23_W as OSN27_W;
#[doc = "Field `OSN28` writer - Number of 28th conversion in ordinary sequence"]
pub use OSN23_W as OSN28_W;
impl R {
    #[doc = "Bits 0:4 - Number of 23st conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn23(&self) -> OSN23_R {
        OSN23_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Number of 24nd conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn24(&self) -> OSN24_R {
        OSN24_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - number of 25rd conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn25(&self) -> OSN25_R {
        OSN25_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Number of 26th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn26(&self) -> OSN26_R {
        OSN26_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - Number of 27th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn27(&self) -> OSN27_R {
        OSN27_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - Number of 28th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn28(&self) -> OSN28_R {
        OSN28_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSQ5")
            .field("osn23", &self.osn23())
            .field("osn28", &self.osn28())
            .field("osn27", &self.osn27())
            .field("osn26", &self.osn26())
            .field("osn25", &self.osn25())
            .field("osn24", &self.osn24())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of 23st conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn23(&mut self) -> OSN23_W<'_, OSQ5_SPEC> {
        OSN23_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - Number of 24nd conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn24(&mut self) -> OSN24_W<'_, OSQ5_SPEC> {
        OSN24_W::new(self, 5)
    }
    #[doc = "Bits 10:14 - number of 25rd conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn25(&mut self) -> OSN25_W<'_, OSQ5_SPEC> {
        OSN25_W::new(self, 10)
    }
    #[doc = "Bits 15:19 - Number of 26th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn26(&mut self) -> OSN26_W<'_, OSQ5_SPEC> {
        OSN26_W::new(self, 15)
    }
    #[doc = "Bits 20:24 - Number of 27th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn27(&mut self) -> OSN27_W<'_, OSQ5_SPEC> {
        OSN27_W::new(self, 20)
    }
    #[doc = "Bits 25:29 - Number of 28th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn28(&mut self) -> OSN28_W<'_, OSQ5_SPEC> {
        OSN28_W::new(self, 25)
    }
}
#[doc = "Ordinary sequence register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`osq5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osq5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSQ5_SPEC;
impl crate::RegisterSpec for OSQ5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osq5::R`](R) reader structure"]
impl crate::Readable for OSQ5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`osq5::W`](W) writer structure"]
impl crate::Writable for OSQ5_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSQ5 to value 0"]
impl crate::Resettable for OSQ5_SPEC {}
