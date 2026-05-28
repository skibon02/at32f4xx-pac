#[doc = "Register `RMP` reader"]
pub type R = crate::R<RMP_SPEC>;
#[doc = "Register `RMP` writer"]
pub type W = crate::W<RMP_SPEC>;
#[doc = "TMR5 channel 4 input remap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TMR5_CH4_IRMP_A {
    #[doc = "0: GPIO"]
    Gpio = 0,
    #[doc = "1: LICK clock"]
    Lick = 1,
    #[doc = "2: LEXT clock"]
    Lext = 2,
    #[doc = "3: ERTC wakeup interrupt"]
    ErtcWakeup = 3,
}
impl From<TMR5_CH4_IRMP_A> for u8 {
    #[inline(always)]
    fn from(variant: TMR5_CH4_IRMP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TMR5_CH4_IRMP_A {
    type Ux = u8;
}
impl crate::IsEnum for TMR5_CH4_IRMP_A {}
#[doc = "Field `TMR5_CH4_IRMP` reader - TMR5 channel 4 input remap"]
pub type TMR5_CH4_IRMP_R = crate::FieldReader<TMR5_CH4_IRMP_A>;
impl TMR5_CH4_IRMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TMR5_CH4_IRMP_A {
        match self.bits {
            0 => TMR5_CH4_IRMP_A::Gpio,
            1 => TMR5_CH4_IRMP_A::Lick,
            2 => TMR5_CH4_IRMP_A::Lext,
            3 => TMR5_CH4_IRMP_A::ErtcWakeup,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == TMR5_CH4_IRMP_A::Gpio
    }
    #[doc = "LICK clock"]
    #[inline(always)]
    pub fn is_lick(&self) -> bool {
        *self == TMR5_CH4_IRMP_A::Lick
    }
    #[doc = "LEXT clock"]
    #[inline(always)]
    pub fn is_lext(&self) -> bool {
        *self == TMR5_CH4_IRMP_A::Lext
    }
    #[doc = "ERTC wakeup interrupt"]
    #[inline(always)]
    pub fn is_ertc_wakeup(&self) -> bool {
        *self == TMR5_CH4_IRMP_A::ErtcWakeup
    }
}
#[doc = "Field `TMR5_CH4_IRMP` writer - TMR5 channel 4 input remap"]
pub type TMR5_CH4_IRMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TMR5_CH4_IRMP_A, crate::Safe>;
impl<'a, REG> TMR5_CH4_IRMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(TMR5_CH4_IRMP_A::Gpio)
    }
    #[doc = "LICK clock"]
    #[inline(always)]
    pub fn lick(self) -> &'a mut crate::W<REG> {
        self.variant(TMR5_CH4_IRMP_A::Lick)
    }
    #[doc = "LEXT clock"]
    #[inline(always)]
    pub fn lext(self) -> &'a mut crate::W<REG> {
        self.variant(TMR5_CH4_IRMP_A::Lext)
    }
    #[doc = "ERTC wakeup interrupt"]
    #[inline(always)]
    pub fn ertc_wakeup(self) -> &'a mut crate::W<REG> {
        self.variant(TMR5_CH4_IRMP_A::ErtcWakeup)
    }
}
impl R {
    #[doc = "Bits 6:7 - TMR5 channel 4 input remap"]
    #[inline(always)]
    pub fn tmr5_ch4_irmp(&self) -> TMR5_CH4_IRMP_R {
        TMR5_CH4_IRMP_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMP")
            .field("tmr5_ch4_irmp", &self.tmr5_ch4_irmp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 6:7 - TMR5 channel 4 input remap"]
    #[inline(always)]
    pub fn tmr5_ch4_irmp(&mut self) -> TMR5_CH4_IRMP_W<'_, RMP_SPEC> {
        TMR5_CH4_IRMP_W::new(self, 6)
    }
}
#[doc = "TMR5 remap\n\nYou can [`read`](crate::Reg::read) this register and get [`rmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RMP_SPEC;
impl crate::RegisterSpec for RMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rmp::R`](R) reader structure"]
impl crate::Readable for RMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rmp::W`](W) writer structure"]
impl crate::Writable for RMP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RMP to value 0"]
impl crate::Resettable for RMP_SPEC {}
