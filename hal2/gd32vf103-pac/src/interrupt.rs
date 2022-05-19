/// Interrupt number
pub unsafe trait Nr {
    /// Returns the number associated with an interrupt
    fn nr(&self) -> u8;
}


#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "3 - Software interrupt"]
    INT_SFT = 3,
    #[doc = "7 - Timer interrupt"]
    INT_TMR = 7,
    #[doc = "17 - Bus Error interrupt"]
    INT_BWEI = 17,
    #[doc = "18 - Performance Monitor interrupt"]
    INT_PMOVI = 18,
    #[doc = "19 - WWDGT"]
    WWDGT = 19,
    #[doc = "20 - EXTI_LVD"]
    EXTI_LVD = 20,
    #[doc = "21 - Tamper"]
    TAMPER = 21,
    #[doc = "22 - RTC"]
    RTC = 22,
    #[doc = "23 - FMC"]
    FMC = 23,
    #[doc = "24 - RCU"]
    RCU = 24,
    #[doc = "25 - EXTI_Line0"]
    EXTI_LINE0 = 25,
    #[doc = "26 - EXTI_Line1"]
    EXTI_LINE1 = 26,
    #[doc = "27 - EXTI_Line2"]
    EXTI_LINE2 = 27,
    #[doc = "28 - EXTI_Line3"]
    EXTI_LINE3 = 28,
    #[doc = "29 - EXTI_Line4"]
    EXTI_LINE4 = 29,
    #[doc = "30 - DMA0_Channel0"]
    DMA0_CHANNEL0 = 30,
    #[doc = "31 - DMA0_Channel1"]
    DMA0_CHANNEL1 = 31,
    #[doc = "32 - DMA0_Channel2"]
    DMA0_CHANNEL2 = 32,
    #[doc = "33 - DMA0_Channel3"]
    DMA0_CHANNEL3 = 33,
    #[doc = "34 - DMA0_Channel4"]
    DMA0_CHANNEL4 = 34,
    #[doc = "35 - DMA0_Channel5"]
    DMA0_CHANNEL5 = 35,
    #[doc = "36 - DMA0_Channel6"]
    DMA0_CHANNEL6 = 36,
    #[doc = "37 - ADC0_1"]
    ADC0_1 = 37,
    #[doc = "38 - CAN0_TX"]
    CAN0_TX = 38,
    #[doc = "39 - CAN0_RX0"]
    CAN0_RX0 = 39,
    #[doc = "40 - CAN0_RX1"]
    CAN0_RX1 = 40,
    #[doc = "41 - CAN0_EWMC"]
    CAN0_EWMC = 41,
    #[doc = "42 - EXTI_line9_5"]
    EXTI_LINE9_5 = 42,
    #[doc = "43 - TIMER0_BRK"]
    TIMER0_BRK = 43,
    #[doc = "44 - TIMER0_UP"]
    TIMER0_UP = 44,
    #[doc = "45 - TIMER0_TRG_CMT"]
    TIMER0_TRG_CMT = 45,
    #[doc = "46 - TIMER0_Channel"]
    TIMER0_CHANNEL = 46,
    #[doc = "47 - TIMER1"]
    TIMER1 = 47,
    #[doc = "48 - TIMER2"]
    TIMER2 = 48,
    #[doc = "49 - TIMER3"]
    TIMER3 = 49,
    #[doc = "50 - I2C0_EV"]
    I2C0_EV = 50,
    #[doc = "51 - I2C0_ER"]
    I2C0_ER = 51,
    #[doc = "52 - I2C1_EV"]
    I2C1_EV = 52,
    #[doc = "53 - I2C1_ER"]
    I2C1_ER = 53,
    #[doc = "54 - SPI0"]
    SPI0 = 54,
    #[doc = "55 - SPI1"]
    SPI1 = 55,
    #[doc = "56 - USART0"]
    USART0 = 56,
    #[doc = "57 - USART1"]
    USART1 = 57,
    #[doc = "58 - USART2"]
    USART2 = 58,
    #[doc = "59 - EXTI_line15_10"]
    EXTI_LINE15_10 = 59,
    #[doc = "60 - RTC_Alarm"]
    RTC_ALARM = 60,
    #[doc = "61 - USBFS_WKUP"]
    USBFS_WKUP = 61,
    #[doc = "69 - TIMER4"]
    TIMER4 = 69,
    #[doc = "70 - SPI2"]
    SPI2 = 70,
    #[doc = "71 - UART3"]
    UART3 = 71,
    #[doc = "72 - UART4"]
    UART4 = 72,
    #[doc = "73 - TIMER5"]
    TIMER5 = 73,
    #[doc = "74 - TIMER6"]
    TIMER6 = 74,
    #[doc = "75 - DMA1_Channel0"]
    DMA1_CHANNEL0 = 75,
    #[doc = "76 - DMA1_Channel1"]
    DMA1_CHANNEL1 = 76,
    #[doc = "77 - DMA1_Channel2"]
    DMA1_CHANNEL2 = 77,
    #[doc = "78 - DMA1_Channel3"]
    DMA1_CHANNEL3 = 78,
    #[doc = "79 - DMA1_Channel4"]
    DMA1_CHANNEL4 = 79,
    #[doc = "82 - CAN1_TX"]
    CAN1_TX = 82,
    #[doc = "83 - CAN1_RX0"]
    CAN1_RX0 = 83,
    #[doc = "84 - CAN1_RX1"]
    CAN1_RX1 = 84,
    #[doc = "85 - CAN1_EWMC"]
    CAN1_EWMC = 85,
    #[doc = "86 - USBFS"]
    USBFS = 86,
}
unsafe impl Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            3 => Ok(Interrupt::INT_SFT),
            7 => Ok(Interrupt::INT_TMR),
            17 => Ok(Interrupt::INT_BWEI),
            18 => Ok(Interrupt::INT_PMOVI),
            19 => Ok(Interrupt::WWDGT),
            20 => Ok(Interrupt::EXTI_LVD),
            21 => Ok(Interrupt::TAMPER),
            22 => Ok(Interrupt::RTC),
            23 => Ok(Interrupt::FMC),
            24 => Ok(Interrupt::RCU),
            25 => Ok(Interrupt::EXTI_LINE0),
            26 => Ok(Interrupt::EXTI_LINE1),
            27 => Ok(Interrupt::EXTI_LINE2),
            28 => Ok(Interrupt::EXTI_LINE3),
            29 => Ok(Interrupt::EXTI_LINE4),
            30 => Ok(Interrupt::DMA0_CHANNEL0),
            31 => Ok(Interrupt::DMA0_CHANNEL1),
            32 => Ok(Interrupt::DMA0_CHANNEL2),
            33 => Ok(Interrupt::DMA0_CHANNEL3),
            34 => Ok(Interrupt::DMA0_CHANNEL4),
            35 => Ok(Interrupt::DMA0_CHANNEL5),
            36 => Ok(Interrupt::DMA0_CHANNEL6),
            37 => Ok(Interrupt::ADC0_1),
            38 => Ok(Interrupt::CAN0_TX),
            39 => Ok(Interrupt::CAN0_RX0),
            40 => Ok(Interrupt::CAN0_RX1),
            41 => Ok(Interrupt::CAN0_EWMC),
            42 => Ok(Interrupt::EXTI_LINE9_5),
            43 => Ok(Interrupt::TIMER0_BRK),
            44 => Ok(Interrupt::TIMER0_UP),
            45 => Ok(Interrupt::TIMER0_TRG_CMT),
            46 => Ok(Interrupt::TIMER0_CHANNEL),
            47 => Ok(Interrupt::TIMER1),
            48 => Ok(Interrupt::TIMER2),
            49 => Ok(Interrupt::TIMER3),
            50 => Ok(Interrupt::I2C0_EV),
            51 => Ok(Interrupt::I2C0_ER),
            52 => Ok(Interrupt::I2C1_EV),
            53 => Ok(Interrupt::I2C1_ER),
            54 => Ok(Interrupt::SPI0),
            55 => Ok(Interrupt::SPI1),
            56 => Ok(Interrupt::USART0),
            57 => Ok(Interrupt::USART1),
            58 => Ok(Interrupt::USART2),
            59 => Ok(Interrupt::EXTI_LINE15_10),
            60 => Ok(Interrupt::RTC_ALARM),
            61 => Ok(Interrupt::USBFS_WKUP),
            69 => Ok(Interrupt::TIMER4),
            70 => Ok(Interrupt::SPI2),
            71 => Ok(Interrupt::UART3),
            72 => Ok(Interrupt::UART4),
            73 => Ok(Interrupt::TIMER5),
            74 => Ok(Interrupt::TIMER6),
            75 => Ok(Interrupt::DMA1_CHANNEL0),
            76 => Ok(Interrupt::DMA1_CHANNEL1),
            77 => Ok(Interrupt::DMA1_CHANNEL2),
            78 => Ok(Interrupt::DMA1_CHANNEL3),
            79 => Ok(Interrupt::DMA1_CHANNEL4),
            82 => Ok(Interrupt::CAN1_TX),
            83 => Ok(Interrupt::CAN1_RX0),
            84 => Ok(Interrupt::CAN1_RX1),
            85 => Ok(Interrupt::CAN1_EWMC),
            86 => Ok(Interrupt::USBFS),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
#[doc = r" Assigns a handler to an interrupt"]
#[doc = r""]
#[doc = r" This macro takes two arguments: the name of an interrupt and the path to the"]
#[doc = r" function that will be used as the handler of that interrupt. That function"]
#[doc = r" must have signature `fn()`."]
#[doc = r""]
#[doc = r" Optionally, a third argument may be used to declare interrupt local data."]
#[doc = r" The handler will have exclusive access to these *local* variables on each"]
#[doc = r" invocation. If the third argument is used then the signature of the handler"]
#[doc = r" function must be `fn(&mut $NAME::Locals)` where `$NAME` is the first argument"]
#[doc = r" passed to the macro."]
#[doc = r""]
#[doc = r" # Example"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!(TIM2, periodic);"]
#[doc = r""]
#[doc = r" fn periodic() {"]
#[doc = r#"     print!(".");"#]
#[doc = r" }"]
#[doc = r""]
#[doc = r" interrupt!(TIM3, tick, locals: {"]
#[doc = r"     tick: bool = false;"]
#[doc = r" });"]
#[doc = r""]
#[doc = r" fn tick(locals: &mut TIM3::Locals) {"]
#[doc = r"     locals.tick = !locals.tick;"]
#[doc = r""]
#[doc = r"     if locals.tick {"]
#[doc = r#"         println!("Tick");"#]
#[doc = r"     } else {"]
#[doc = r#"         println!("Tock");"#]
#[doc = r"     }"]
#[doc = r" }"]
#[doc = r" ```"]
macro_rules ! interrupt { ( $ NAME : ident , $ path : path , locals : { $ ( $ lvar : ident : $ lty : ty = $ lval : expr ; ) * } ) => { # [ allow ( non_snake_case ) ] mod $ NAME { pub struct Locals { $ ( pub $ lvar : $ lty , ) * } } # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ( $ lvar : $ lval , ) * } ; let f : fn ( & mut self :: $ NAME :: Locals ) = $ path ; f ( unsafe { & mut LOCALS } ) ; } } ; ( $ NAME : ident , $ path : path ) => { # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn ( ) = $ path ; f ( ) ; } } }
