#[doc = "Register `OADDR1` reader"]
pub type R = crate::R<OADDR1_SPEC>;
#[doc = "Register `OADDR1` writer"]
pub type W = crate::W<OADDR1_SPEC>;
#[doc = "Field `ADDR1` reader - Interface address"]
pub type ADDR1_R = crate::FieldReader<u16>;
#[doc = "Field `ADDR1` writer - Interface address"]
pub type ADDR1_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16, crate::Safe>;
#[doc = "Own Address mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDR1MODE_A {
    #[doc = "0: 7-bit address mode"]
    Bit7 = 0,
    #[doc = "1: 10-bit address mode"]
    Bit10 = 1,
}
impl From<ADDR1MODE_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR1MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR1MODE` reader - Own Address mode"]
pub type ADDR1MODE_R = crate::BitReader<ADDR1MODE_A>;
impl ADDR1MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADDR1MODE_A {
        match self.bits {
            false => ADDR1MODE_A::Bit7,
            true => ADDR1MODE_A::Bit10,
        }
    }
    #[doc = "7-bit address mode"]
    #[inline(always)]
    pub fn is_bit7(&self) -> bool {
        *self == ADDR1MODE_A::Bit7
    }
    #[doc = "10-bit address mode"]
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == ADDR1MODE_A::Bit10
    }
}
#[doc = "Field `ADDR1MODE` writer - Own Address mode"]
pub type ADDR1MODE_W<'a, REG> = crate::BitWriter<'a, REG, ADDR1MODE_A>;
impl<'a, REG> ADDR1MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "7-bit address mode"]
    #[inline(always)]
    pub fn bit7(self) -> &'a mut crate::W<REG> {
        self.variant(ADDR1MODE_A::Bit7)
    }
    #[doc = "10-bit address mode"]
    #[inline(always)]
    pub fn bit10(self) -> &'a mut crate::W<REG> {
        self.variant(ADDR1MODE_A::Bit10)
    }
}
#[doc = "Own address 1 enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDR1EN_A {
    #[doc = "0: Own address 1 disabled"]
    Disabled = 0,
    #[doc = "1: Own address 1 enabled"]
    Enabled = 1,
}
impl From<ADDR1EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR1EN` reader - Own address 1 enable"]
pub type ADDR1EN_R = crate::BitReader<ADDR1EN_A>;
impl ADDR1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADDR1EN_A {
        match self.bits {
            false => ADDR1EN_A::Disabled,
            true => ADDR1EN_A::Enabled,
        }
    }
    #[doc = "Own address 1 disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDR1EN_A::Disabled
    }
    #[doc = "Own address 1 enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDR1EN_A::Enabled
    }
}
#[doc = "Field `ADDR1EN` writer - Own address 1 enable"]
pub type ADDR1EN_W<'a, REG> = crate::BitWriter<'a, REG, ADDR1EN_A>;
impl<'a, REG> ADDR1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Own address 1 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADDR1EN_A::Disabled)
    }
    #[doc = "Own address 1 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADDR1EN_A::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:9 - Interface address"]
    #[inline(always)]
    pub fn addr1(&self) -> ADDR1_R {
        ADDR1_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Own Address mode"]
    #[inline(always)]
    pub fn addr1mode(&self) -> ADDR1MODE_R {
        ADDR1MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Own address 1 enable"]
    #[inline(always)]
    pub fn addr1en(&self) -> ADDR1EN_R {
        ADDR1EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OADDR1")
            .field("addr1", &self.addr1())
            .field("addr1mode", &self.addr1mode())
            .field("addr1en", &self.addr1en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - Interface address"]
    #[inline(always)]
    pub fn addr1(&mut self) -> ADDR1_W<'_, OADDR1_SPEC> {
        ADDR1_W::new(self, 0)
    }
    #[doc = "Bit 10 - Own Address mode"]
    #[inline(always)]
    pub fn addr1mode(&mut self) -> ADDR1MODE_W<'_, OADDR1_SPEC> {
        ADDR1MODE_W::new(self, 10)
    }
    #[doc = "Bit 15 - Own address 1 enable"]
    #[inline(always)]
    pub fn addr1en(&mut self) -> ADDR1EN_W<'_, OADDR1_SPEC> {
        ADDR1EN_W::new(self, 15)
    }
}
#[doc = "Own address register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`oaddr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oaddr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OADDR1_SPEC;
impl crate::RegisterSpec for OADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oaddr1::R`](R) reader structure"]
impl crate::Readable for OADDR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`oaddr1::W`](W) writer structure"]
impl crate::Writable for OADDR1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OADDR1 to value 0"]
impl crate::Resettable for OADDR1_SPEC {}
