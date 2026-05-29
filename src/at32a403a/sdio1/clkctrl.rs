#[doc = "Register `CLKCTRL` reader"]
pub type R = crate::R<CLKCTRL_SPEC>;
#[doc = "Register `CLKCTRL` writer"]
pub type W = crate::W<CLKCTRL_SPEC>;
#[doc = "Field `CLKDIV` reader - 0-7 bits of CLKDIV. SDIO_CK = SDIOCLK / (CLKDIV + 2)"]
pub type CLKDIV_R = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - 0-7 bits of CLKDIV. SDIO_CK = SDIOCLK / (CLKDIV + 2)"]
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
#[doc = "Clock output enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKOEN_A {
    #[doc = "0: SDIO_CK output signal is disabled"]
    Disabled = 0,
    #[doc = "1: SDIO_CK output signal is enabled"]
    Enabled = 1,
}
impl From<CLKOEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKOEN` reader - Clock output enable"]
pub type CLKOEN_R = crate::BitReader<CLKOEN_A>;
impl CLKOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLKOEN_A {
        match self.bits {
            false => CLKOEN_A::Disabled,
            true => CLKOEN_A::Enabled,
        }
    }
    #[doc = "SDIO_CK output signal is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKOEN_A::Disabled
    }
    #[doc = "SDIO_CK output signal is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CLKOEN_A::Enabled
    }
}
#[doc = "Field `CLKOEN` writer - Clock output enable"]
pub type CLKOEN_W<'a, REG> = crate::BitWriter<'a, REG, CLKOEN_A>;
impl<'a, REG> CLKOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SDIO_CK output signal is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOEN_A::Disabled)
    }
    #[doc = "SDIO_CK output signal is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOEN_A::Enabled)
    }
}
#[doc = "Power saving mode enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWRSVEN_A {
    #[doc = "0: SDIO_CK is always output"]
    Normal = 0,
    #[doc = "1: SDIO_CK is only output when the bus is active"]
    PowerSaving = 1,
}
impl From<PWRSVEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWRSVEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSVEN` reader - Power saving mode enable"]
pub type PWRSVEN_R = crate::BitReader<PWRSVEN_A>;
impl PWRSVEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWRSVEN_A {
        match self.bits {
            false => PWRSVEN_A::Normal,
            true => PWRSVEN_A::PowerSaving,
        }
    }
    #[doc = "SDIO_CK is always output"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PWRSVEN_A::Normal
    }
    #[doc = "SDIO_CK is only output when the bus is active"]
    #[inline(always)]
    pub fn is_power_saving(&self) -> bool {
        *self == PWRSVEN_A::PowerSaving
    }
}
#[doc = "Field `PWRSVEN` writer - Power saving mode enable"]
pub type PWRSVEN_W<'a, REG> = crate::BitWriter<'a, REG, PWRSVEN_A>;
impl<'a, REG> PWRSVEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SDIO_CK is always output"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PWRSVEN_A::Normal)
    }
    #[doc = "SDIO_CK is only output when the bus is active"]
    #[inline(always)]
    pub fn power_saving(self) -> &'a mut crate::W<REG> {
        self.variant(PWRSVEN_A::PowerSaving)
    }
}
#[doc = "Clock divider bypass enable bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPSEN_A {
    #[doc = "0: SDIO_CK output signal is driven by the SDIOCLK that is divided according to the CLKDIV value"]
    NotBypassed = 0,
    #[doc = "1: SDIO_CK output signal is directly driven by the SDIOCLK"]
    Bypassed = 1,
}
impl From<BYPSEN_A> for bool {
    #[inline(always)]
    fn from(variant: BYPSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPSEN` reader - Clock divider bypass enable bit"]
pub type BYPSEN_R = crate::BitReader<BYPSEN_A>;
impl BYPSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BYPSEN_A {
        match self.bits {
            false => BYPSEN_A::NotBypassed,
            true => BYPSEN_A::Bypassed,
        }
    }
    #[doc = "SDIO_CK output signal is driven by the SDIOCLK that is divided according to the CLKDIV value"]
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == BYPSEN_A::NotBypassed
    }
    #[doc = "SDIO_CK output signal is directly driven by the SDIOCLK"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == BYPSEN_A::Bypassed
    }
}
#[doc = "Field `BYPSEN` writer - Clock divider bypass enable bit"]
pub type BYPSEN_W<'a, REG> = crate::BitWriter<'a, REG, BYPSEN_A>;
impl<'a, REG> BYPSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SDIO_CK output signal is driven by the SDIOCLK that is divided according to the CLKDIV value"]
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(BYPSEN_A::NotBypassed)
    }
    #[doc = "SDIO_CK output signal is directly driven by the SDIOCLK"]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(BYPSEN_A::Bypassed)
    }
}
#[doc = "Bus width selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BUSWS_A {
    #[doc = "0: 1-bit data transfer"]
    OneBit = 0,
    #[doc = "1: 4-bit data transfer"]
    FourBit = 1,
    #[doc = "2: 8-bit data transfer"]
    EightBit = 2,
}
impl From<BUSWS_A> for u8 {
    #[inline(always)]
    fn from(variant: BUSWS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BUSWS_A {
    type Ux = u8;
}
impl crate::IsEnum for BUSWS_A {}
#[doc = "Field `BUSWS` reader - Bus width selection"]
pub type BUSWS_R = crate::FieldReader<BUSWS_A>;
impl BUSWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BUSWS_A> {
        match self.bits {
            0 => Some(BUSWS_A::OneBit),
            1 => Some(BUSWS_A::FourBit),
            2 => Some(BUSWS_A::EightBit),
            _ => None,
        }
    }
    #[doc = "1-bit data transfer"]
    #[inline(always)]
    pub fn is_one_bit(&self) -> bool {
        *self == BUSWS_A::OneBit
    }
    #[doc = "4-bit data transfer"]
    #[inline(always)]
    pub fn is_four_bit(&self) -> bool {
        *self == BUSWS_A::FourBit
    }
    #[doc = "8-bit data transfer"]
    #[inline(always)]
    pub fn is_eight_bit(&self) -> bool {
        *self == BUSWS_A::EightBit
    }
}
#[doc = "Field `BUSWS` writer - Bus width selection"]
pub type BUSWS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, BUSWS_A>;
impl<'a, REG> BUSWS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1-bit data transfer"]
    #[inline(always)]
    pub fn one_bit(self) -> &'a mut crate::W<REG> {
        self.variant(BUSWS_A::OneBit)
    }
    #[doc = "4-bit data transfer"]
    #[inline(always)]
    pub fn four_bit(self) -> &'a mut crate::W<REG> {
        self.variant(BUSWS_A::FourBit)
    }
    #[doc = "8-bit data transfer"]
    #[inline(always)]
    pub fn eight_bit(self) -> &'a mut crate::W<REG> {
        self.variant(BUSWS_A::EightBit)
    }
}
#[doc = "SDIO_CK edge selection bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKEDS_A {
    #[doc = "0: SDIO_CK is generated on the rising edge of the SDIOCLK"]
    Rising = 0,
    #[doc = "1: SDIO_CK is generated on the falling edge of the SDIOCLK"]
    Falling = 1,
}
impl From<CLKEDS_A> for bool {
    #[inline(always)]
    fn from(variant: CLKEDS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKEDS` reader - SDIO_CK edge selection bit"]
pub type CLKEDS_R = crate::BitReader<CLKEDS_A>;
impl CLKEDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLKEDS_A {
        match self.bits {
            false => CLKEDS_A::Rising,
            true => CLKEDS_A::Falling,
        }
    }
    #[doc = "SDIO_CK is generated on the rising edge of the SDIOCLK"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == CLKEDS_A::Rising
    }
    #[doc = "SDIO_CK is generated on the falling edge of the SDIOCLK"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == CLKEDS_A::Falling
    }
}
#[doc = "Field `CLKEDS` writer - SDIO_CK edge selection bit"]
pub type CLKEDS_W<'a, REG> = crate::BitWriter<'a, REG, CLKEDS_A>;
impl<'a, REG> CLKEDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SDIO_CK is generated on the rising edge of the SDIOCLK"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(CLKEDS_A::Rising)
    }
    #[doc = "SDIO_CK is generated on the falling edge of the SDIOCLK"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(CLKEDS_A::Falling)
    }
}
#[doc = "Hardware flow control enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HFCEN_A {
    #[doc = "0: Hardware flow control disabled"]
    Disabled = 0,
    #[doc = "1: Hardware flow control enabled"]
    Enabled = 1,
}
impl From<HFCEN_A> for bool {
    #[inline(always)]
    fn from(variant: HFCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFCEN` reader - Hardware flow control enable"]
pub type HFCEN_R = crate::BitReader<HFCEN_A>;
impl HFCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HFCEN_A {
        match self.bits {
            false => HFCEN_A::Disabled,
            true => HFCEN_A::Enabled,
        }
    }
    #[doc = "Hardware flow control disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HFCEN_A::Disabled
    }
    #[doc = "Hardware flow control enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HFCEN_A::Enabled
    }
}
#[doc = "Field `HFCEN` writer - Hardware flow control enable"]
pub type HFCEN_W<'a, REG> = crate::BitWriter<'a, REG, HFCEN_A>;
impl<'a, REG> HFCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware flow control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HFCEN_A::Disabled)
    }
    #[doc = "Hardware flow control enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HFCEN_A::Enabled)
    }
}
#[doc = "Field `CLKDIV98` reader - 8-9 bits of CLKDIV. SDIO_CK = SDIOCLK / (CLKDIV + 2)"]
pub type CLKDIV98_R = crate::FieldReader;
#[doc = "Field `CLKDIV98` writer - 8-9 bits of CLKDIV. SDIO_CK = SDIOCLK / (CLKDIV + 2)"]
pub type CLKDIV98_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - 0-7 bits of CLKDIV. SDIO_CK = SDIOCLK / (CLKDIV + 2)"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Clock output enable"]
    #[inline(always)]
    pub fn clkoen(&self) -> CLKOEN_R {
        CLKOEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Power saving mode enable"]
    #[inline(always)]
    pub fn pwrsven(&self) -> PWRSVEN_R {
        PWRSVEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock divider bypass enable bit"]
    #[inline(always)]
    pub fn bypsen(&self) -> BYPSEN_R {
        BYPSEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Bus width selection"]
    #[inline(always)]
    pub fn busws(&self) -> BUSWS_R {
        BUSWS_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - SDIO_CK edge selection bit"]
    #[inline(always)]
    pub fn clkeds(&self) -> CLKEDS_R {
        CLKEDS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Hardware flow control enable"]
    #[inline(always)]
    pub fn hfcen(&self) -> HFCEN_R {
        HFCEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - 8-9 bits of CLKDIV. SDIO_CK = SDIOCLK / (CLKDIV + 2)"]
    #[inline(always)]
    pub fn clkdiv98(&self) -> CLKDIV98_R {
        CLKDIV98_R::new(((self.bits >> 15) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKCTRL")
            .field("clkdiv", &self.clkdiv())
            .field("clkoen", &self.clkoen())
            .field("pwrsven", &self.pwrsven())
            .field("bypsen", &self.bypsen())
            .field("busws", &self.busws())
            .field("clkeds", &self.clkeds())
            .field("hfcen", &self.hfcen())
            .field("clkdiv98", &self.clkdiv98())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - 0-7 bits of CLKDIV. SDIO_CK = SDIOCLK / (CLKDIV + 2)"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W<'_, CLKCTRL_SPEC> {
        CLKDIV_W::new(self, 0)
    }
    #[doc = "Bit 8 - Clock output enable"]
    #[inline(always)]
    pub fn clkoen(&mut self) -> CLKOEN_W<'_, CLKCTRL_SPEC> {
        CLKOEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Power saving mode enable"]
    #[inline(always)]
    pub fn pwrsven(&mut self) -> PWRSVEN_W<'_, CLKCTRL_SPEC> {
        PWRSVEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clock divider bypass enable bit"]
    #[inline(always)]
    pub fn bypsen(&mut self) -> BYPSEN_W<'_, CLKCTRL_SPEC> {
        BYPSEN_W::new(self, 10)
    }
    #[doc = "Bits 11:12 - Bus width selection"]
    #[inline(always)]
    pub fn busws(&mut self) -> BUSWS_W<'_, CLKCTRL_SPEC> {
        BUSWS_W::new(self, 11)
    }
    #[doc = "Bit 13 - SDIO_CK edge selection bit"]
    #[inline(always)]
    pub fn clkeds(&mut self) -> CLKEDS_W<'_, CLKCTRL_SPEC> {
        CLKEDS_W::new(self, 13)
    }
    #[doc = "Bit 14 - Hardware flow control enable"]
    #[inline(always)]
    pub fn hfcen(&mut self) -> HFCEN_W<'_, CLKCTRL_SPEC> {
        HFCEN_W::new(self, 14)
    }
    #[doc = "Bits 15:16 - 8-9 bits of CLKDIV. SDIO_CK = SDIOCLK / (CLKDIV + 2)"]
    #[inline(always)]
    pub fn clkdiv98(&mut self) -> CLKDIV98_W<'_, CLKCTRL_SPEC> {
        CLKDIV98_W::new(self, 15)
    }
}
#[doc = "SD clock control register (SDIO_CLKCTRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKCTRL_SPEC;
impl crate::RegisterSpec for CLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkctrl::R`](R) reader structure"]
impl crate::Readable for CLKCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkctrl::W`](W) writer structure"]
impl crate::Writable for CLKCTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKCTRL to value 0"]
impl crate::Resettable for CLKCTRL_SPEC {}
