#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(type_name_of_val)]
#![feature(mem_copy_fn)]
use core::mem::copy;

use core::any::type_name_of_val;
extern crate jlink_rtt;
// src/main.rs or src/bin/my-app.rs
use defmt_rtt as _;
use rtt_target::{rtt_init_print, rprintln};



use defmt_rtt as _;
use embedded_io::asynch::Write; // linker shenanigans
use embedded_io::asynch::Read; 
use heapless::Vec;

//use char::to_string;
use core::str;

//use std::mem;
//extern crate std; 
//use std;
//use std::mem; 

mod example;

use core::future::Future;
use embassy_executor::Spawner;


use embassy_nrf::{
    buffered_uarte::{BufferedUarte, BufferedUarteRx, BufferedUarteTx, State},
    gpio::{AnyPin, Input, Level, Output, OutputDrive, Pin, Pull},
    interrupt::{self, UARTE0_UART0},
    peripherals::{PPI_CH1, PPI_CH2, PPI_CH3, PPI_CH4,TIMER0, UARTE0,TIMER1,UARTE1},
    uarte,
};


struct UarteComponents {
    pub uarte: UARTE1,
    pub timer: TIMER1,
    pub ppi_ch1: PPI_CH1,
    pub ppi_ch2: PPI_CH2,
    pub irq: UARTE0_UART0,
    pub rxd: AnyPin,
    pub txd: AnyPin,
    pub rts: AnyPin,
    pub cts: AnyPin,
    pub config: uarte::Config,
    pub state: State<'static, UARTE0, TIMER0>,
    pub tx_buffer: [u8; 256],
    pub rx_buffer: [u8; 256],
}


struct PinOut {
    lc79_pen: AnyPin,
    lc79_rx: AnyPin,
    lc79_tx: AnyPin,
    lc79_rts: AnyPin,
    lc79_cts: AnyPin,
    lc79_ap_req: AnyPin,

}






use embassy_time::{with_timeout, Duration, Timer};
use sim7000_async::{spawn_modem, BuildIo, ModemPower, PowerState, SplitIo};

use defmt_rtt as _; // linker shenanigans

//#[cfg(debug_assertions)]
//extern crate panic_rtt_target;
extern crate panic_probe;

type Modem = sim7000_async::modem::Modem<'static, ModemPowerPins>;

/* 
use std::ascii::escape_default;
use std::str;

fn show(bs: &[u8]) -> String {
    let mut visible = String::new();
    for &b in bs {
        let part: Vec<u8> = escape_default(b).collect();
        visible.push_str(str::from_utf8(&part).unwrap());
    }
    visible
}*/

#[embassy_executor::main]
async fn main(spawner: Spawner) {





    //sim7600 init
    let p1 = embassy_nrf::init(Default::default());
    defmt::error!("log-level: error");
    defmt::warn!("log-level: warn");
    defmt::info!("log-level: info");
    defmt::debug!("log-level: debug");
    defmt::trace!("log-level: trace");
    let irquart1 = interrupt::take!(UARTE1);
    let mut uart1_conf = uarte::Config::default();
    uart1_conf.baudrate = embassy_nrf::uarte::Baudrate::BAUD115200;
    uart1_conf.parity = embassy_nrf::uarte::Parity::EXCLUDED;
    let vcu_pinout = PinOut {
        lc79_pen: p1.P0_12.degrade(),
        lc79_rx: p1.P0_15.degrade(),
        lc79_tx: p1.P0_14.degrade(),
        lc79_cts: p1.P0_16.degrade(),
        lc79_rts: p1.P1_02.degrade(),
        lc79_ap_req: p1.P1_06.degrade(),
    };

    let mut uart1 = embassy_nrf::uarte::Uarte::new_with_rtscts(
        p1.UARTE1,
        irquart1,
        vcu_pinout.lc79_rx,
        vcu_pinout.lc79_tx,
        vcu_pinout.lc79_cts,
        vcu_pinout.lc79_rts,
        uart1_conf,
    );

    let mut lc79_pen = Output::new(
        vcu_pinout.lc79_pen,
        embassy_nrf::gpio::Level::High,
        embassy_nrf::gpio::OutputDrive::Standard,
    );

    // LC79_BOOT and LC79_STANDBY are controlled by the extender MCP23008-E_SS
    lc79_pen.set_low();
    Timer::after(Duration::from_millis(1500)).await;
    lc79_pen.set_high(); 
    defmt::info!("Enable LC79D channel");

    
     

/* 
    let mut readbuf = [0u8,1]; 
    loop {
        let mut read= uart1.blocking_read(&mut readbuf).unwrap();
        let strreadbuf:&str = core::str::from_utf8(&readbuf).unwrap();
        defmt::info!("Read{}",&strreadbuf);
    }

*/

    let mut i:u8=0;
    let mut readmiddlebuf = [0u8;1];
    //let mut readbuf = [0u8;256];
    let mut readbuf: [char;250];
    loop {
        //let mut read= uart1.blocking_read(&mut readbuf[i]).unwrap();
        //let strreadbuf:&str = core::str::from_utf8(&readbuf[i]).unwrap();



        let read= uart1.blocking_read(&mut readmiddlebuf[..]).unwrap();
        let strreadbuf = core::str::from_utf8(&readmiddlebuf).unwrap(); //:&str
        //let char_vec: Vec<char,1> = strreadbuf.chars().collect();
        defmt::info!("{}",&strreadbuf);
        defmt::info!("{}",readmiddlebuf[0]);
        //defmt::info!("{}",encode_utf8(readmiddlebuf[0]));
        //readbuf[0]= core::mem::take(&mut strreadbuf[0]);
        //readbuf[0]= char(strreadbuf[0]);
       // defmt::info!("{}",&strreadbuf);

        //defmt::info!("{}",strreadbuf);
        //defmt::info!("{}",type_name_of_val(&char_vec[0]));


 

       
        if((strreadbuf!="\n")&&(i<250))
        {
           // use std::mem; 
           //vec_new[i]=char_vec[0];
           //let got = core::mem::replace(&vec_new[i], char_vec[0]);
           //vec_new[0]= mem::take(&mut v2[0]);
           
           //defmt::info!("{}",vec_new[i]);
            i += 1;
        }
        else{
           // let strreadbuf_line:&str = core::str::from_utf8(&readbuf[0..i]).unwrap();
            //defmt::info!("Read{}",&strreadbuf_line);
            defmt::info!("got line");
            i = 0;
        }
 

     
    }








    


    
    let mut irq = interrupt::take!(UARTE0_UART0);
    //let irq = irq_lc79d;
    let mut config = uarte::Config::default();
    config.parity = uarte::Parity::EXCLUDED;
    config.baudrate = uarte::Baudrate::BAUD115200;
   


    let sim7600_power_pins = ModemPowerPins {
        status: Input::new(p1.P1_12.degrade(), Pull::None),
        power_key: Output::new(p1.P1_05.degrade(), Level::Low, OutputDrive::Standard),
        dtr: Output::new(p1.P0_13.degrade(), Level::Low, OutputDrive::Standard),
        reset: Output::new(p1.P1_04.degrade(), Level::Low, OutputDrive::Standard),
        ri: Input::new(p1.P1_07.degrade(), Pull::Up),
    };
    
    let mut SIM7600_PEN = Output::new(
        p1.P1_00.degrade(),
        embassy_nrf::gpio::Level::High,
        embassy_nrf::gpio::OutputDrive::Standard,
    );
    


    /*  
    let mut uart_lc79d= BufferedUarte::new(
        state,
        &mut self.uarte,
        &mut self.timer,
        &mut self.ppi_ch3,
        &mut self.ppi_ch4,
        &mut self.irq,
        &mut self.p1.P0_15,
        &mut self. p1.P0_14,
        &mut self.p1.P0_16,
        &mut self.p1.P1_02,
        self.config.clone(),
        &mut self.rx_buffer,
        &mut self.tx_buffer,
    );
*/
  /*
    let mut irq_lc79d = interrupt::take!(UARTE1);

    let mut uarte= Uarte::new_with_rtscts(

        p1.UARTE0,
        p1.TIMER0,
        p1.PPI_CH0,
        p1.PPI_CH1,
        irq_lc79d,
        p1.P0_20,
        p1.P0_24,
        p1.P0_08,
        p1.P0_11,
        config,
    );
*/






    


    SIM7600_PEN.set_low();
    Timer::after(Duration::from_millis(1500)).await;
    SIM7600_PEN.set_high();
    defmt::info!("Enable SIM7600 channel");

    let mut modem = spawn_modem!(
        &spawner,
        UarteComponents_1 as UarteComponents_1 { uarte: p1.UARTE0, timer: p1.TIMER0, ppi_ch1: p1.PPI_CH1, ppi_ch2: p1.PPI_CH2, irq, rxd: p1.P0_06.degrade(), txd: p1.P0_08.degrade(), rts: p1.P0_07.degrade(), cts: p1.P1_10.degrade(), config, state: State::new(), tx_buffer: [0; 64], rx_buffer: [0; 64] },
        sim7600_power_pins
    );
    defmt::info!("T0");
    defmt::info!("Initializing 4G modem");
    modem.init().await.unwrap();


    defmt::info!("T1");
    defmt::info!("Activating modem");
    modem.activate().await.unwrap();


    //defmt::info!("T2");
    //defmt::info!("sleeping 1s");
    //Timer::after(Duration::from_millis(1000)).await;

    
    //no voltage checking 
    /*
    match modem.claim_voltage_warner().await {
        Some(warner) => spawner.must_spawn(example::voltage_warn(warner)),
        None => defmt::error!("Failed to take VoltageWarner handle"),
    }
    */
    /* no internal GNSS
    match modem.claim_gnss().await {
     
        Ok(Some(gnss)) => spawner.must_spawn(example::gnss(gnss)),
        Ok(None) => defmt::error!("Failed to take GNSS handle"),
        Err(e) => defmt::error!("Failed to subscribe to GNSS: {:?}", e),
    }
  */
    //defmt::info!("sleeping 5s");
    //Timer::after(Duration::from_millis(5000)).await;
    //defmt::info!("T3");
    //defmt::info!("Signal quality: {:?}", modem.query_signal().await);
    //defmt::info!("System info: {:?}", modem.query_system_info().await);
   // defmt::info!("T4");


    for _ in 0..100 {
        defmt::info!("Z1");
        defmt::info!("sleeping 1s");
        Timer::after(Duration::from_millis(1000)).await;

        defmt::info!("spawning tasks");
        let tcpbin_handle = example::ping_tcpbin(&spawner, &mut modem)
            .await
            .map_err(|e| defmt::error!("Failed to spawn ping_tcpbin: {:?}", e))
            .ok();
        
       
        let qotd_handle = example::get_quote_of_the_day(&spawner, &mut modem)
            .await
            .map_err(|e| defmt::error!("Failed to spawn Quote of the Day: {:?}", e))
            .ok();
        

        defmt::info!("await tcpbin");
        if let Some(handle) = tcpbin_handle {
            if let Err(e) = handle.await {
                defmt::error!("ping_tcpbin failed: {:?}", e);
            }
        }

        defmt::info!("await QotD");
        if let Some(handle) = qotd_handle {
            if let Err(e) = handle.await {
                defmt::error!("get QotD failed: {:?}", e);
            }
        }
        /*  */
    }
    defmt::info!("T6");
    defmt::info!("main() finished");
    loop {
        Timer::after(Duration::from_millis(1000)).await;
    }
}

struct UarteComponents_1 {
    pub uarte: UARTE0,
    pub timer: TIMER0,
    pub ppi_ch1: PPI_CH1,
    pub ppi_ch2: PPI_CH2,
    pub irq: UARTE0_UART0,
    pub rxd: AnyPin,
    pub txd: AnyPin,
    pub rts: AnyPin,
    pub cts: AnyPin,
    pub config: uarte::Config,
    pub state: State<'static, UARTE0, TIMER0>,
    pub tx_buffer: [u8; 64],
    pub rx_buffer: [u8; 64],
}

impl BuildIo for UarteComponents_1 {
    type IO<'d> = AppUarte<'d>
    where
    Self: 'd;

    fn build<'d>(&'d mut self) -> Self::IO<'d> {
        let state = unsafe {
            core::mem::transmute::<
                &'d mut State<'static, UARTE0, TIMER0>,
                &'d mut State<'d, UARTE0, TIMER0>,
            >(&mut self.state)
        };
        AppUarte(BufferedUarte::new(
            state,
            &mut self.uarte,
            &mut self.timer,
            &mut self.ppi_ch1,
            &mut self.ppi_ch2,
            &mut self.irq,
            &mut self.rxd,
            &mut self.txd,
            &mut self.cts,
            &mut self.rts,
            self.config.clone(),
            &mut self.rx_buffer,
            &mut self.tx_buffer,
        ))
    }
}

/* 
struct UarteComponents_2 {
    pub uarte: UARTE0,
    pub timer: TIMER0,
    pub ppi_ch1: PPI_CH1,
    pub ppi_ch2: PPI_CH2,
    pub irq_lc79d: UARTE0_UART0,
    pub rxd: AnyPin,
    pub txd: AnyPin,
    pub rts: AnyPin,
    pub cts: AnyPin,
    pub config_lc79d: uarte::Config,
    pub state: State<'static, UARTE0, TIMER0>,
    pub tx_buffer: [u8; 64],
    pub rx_buffer: [u8; 64],
}

impl BuildIo for UarteComponents_2 {
    type IO<'d> = AppUarte<'d>
    where
    Self: 'd;

    fn build<'d>(&'d mut self) -> Self::IO<'d> {
        let state = unsafe {
            core::mem::transmute::<
                &'d mut State<'static, UARTE0, TIMER0>,
                &'d mut State<'d, UARTE0, TIMER0>,
            >(&mut self.state)
        };
        AppUarte(BufferedUarte::new(
            state,
            &mut self.uarte,
            &mut self.timer,
            &mut self.ppi_ch3,
            &mut self.ppi_ch4,
            &mut self.irq_lc79d,
            &mut self.rxd,
            &mut self.txd,
            &mut self.cts,
            &mut self.rts,
            self.config_lc79d.clone(),
            &mut self.rx_buffer,
            &mut self.tx_buffer,
        ))
    }
}



struct UarteComponents_3 {
    pub uarte: UARTE1,
    pub timer: TIMER1,
    pub ppi_ch1: PPI_CH3,
    pub ppi_ch2: PPI_CH4,
    pub irq_lc79d: UARTE1,
    pub rxd: AnyPin,
    pub txd: AnyPin,
    pub rts: AnyPin,
    pub cts: AnyPin,
    pub config_lc79d: uarte::Config,
    pub state: State<'static, UARTE1, TIMER1>,
    pub tx_buffer: [u8; 64],
    pub rx_buffer: [u8; 64],
}

impl BuildIo for UarteComponents_3 {
    type IO<'d> = AppUarte<'d>
    where
    Self: 'd;

    fn build<'d>(&'d mut self) -> Self::IO<'d> {
        let state = unsafe {
            core::mem::transmute::<
                &'d mut State<'static, UARTE1, TIMER1>,
                &'d mut State<'d, UARTE1, TIMER1>,
            >(&mut self.state)
        };
        AppUarte(BufferedUarte::new(
            state,
            &mut self.uarte,
            &mut self.timer,
            &mut self.ppi_ch3,
            &mut self.ppi_ch4,
            &mut self.irq_lc79d,
            &mut self.rxd,
            &mut self.txd,
            &mut self.cts,
            &mut self.rts,
            self.config_lc79d.clone(),
            &mut self.rx_buffer,
            &mut self.tx_buffer,
        ))
    }
}
*/

/*

*/

struct AppUarte<'d>(
    embassy_nrf::buffered_uarte::BufferedUarte<
        'd,
        embassy_nrf::peripherals::UARTE0,
        embassy_nrf::peripherals::TIMER0,
    >,
);

impl<'d> SplitIo for AppUarte<'d> {
    type Reader<'u> = BufferedUarteRx<'u, 'd, UARTE0, TIMER0>
    where
    Self: 'u;

    type Writer<'u> = BufferedUarteTx<'u, 'd, UARTE0, TIMER0>
    where
    Self: 'u;

    fn split<'u>(&'u mut self) -> (Self::Reader<'u>, Self::Writer<'u>) {
        self.0.split()
    }
}

#[repr(transparent)]
struct AppUarteRead<'d>(
    embassy_nrf::uarte::UarteRxWithIdle<
        'd,
        embassy_nrf::peripherals::UARTE0,
        embassy_nrf::peripherals::TIMER0,
    >,
);

impl<'d> embedded_io::Io for AppUarteRead<'d> {
    type Error = sim7000_async::Error;
}

impl<'d> embedded_io::asynch::Read for AppUarteRead<'d> {
    type ReadFuture<'a> = impl Future<Output = Result<usize, Self::Error>> + 'a
    where
    Self: 'a;

    fn read<'a>(&'a mut self, read: &'a mut [u8]) -> Self::ReadFuture<'a> {
        async move {
            defmt::trace!("Read until idle");
            let n = match with_timeout(Duration::from_millis(1000), self.0.read_until_idle(read))
                .await
            {
                Ok(Ok(result)) => result,
                Ok(Err(_err)) => return Err(sim7000_async::Error::Serial),
                Err(_) => 0,
            };

            if n > 0 {
                defmt::debug!("Read {} bytes from modem uarte", n);
            }

            Ok(n)
        }
    }
}

struct AppUarteWrite<'d>(embassy_nrf::uarte::UarteTx<'d, embassy_nrf::peripherals::UARTE0>);

impl<'d> embedded_io::Io for AppUarteWrite<'d> {
    type Error = sim7000_async::Error;
}

impl<'d> embedded_io::asynch::Write for AppUarteWrite<'d> {
    type WriteFuture<'a> = impl Future<Output = Result<usize, Self::Error>> + 'a
    where
        Self: 'a;

    type FlushFuture<'a> = impl Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    fn write<'a>(&'a mut self, words: &'a [u8]) -> Self::WriteFuture<'a> {
        async {
            self.0
                .write(words)
                .await
                .map_err(|_| sim7000_async::Error::Serial)?;
            Ok(words.len())
        }
    }

    fn flush(&mut self) -> Self::FlushFuture<'_> {
        async { Ok(()) }
    }
}

pub struct ModemPowerPins {
    pub status: Input<'static, AnyPin>,
    pub power_key: Output<'static, AnyPin>,
    pub dtr: Output<'static, AnyPin>,
    pub reset: Output<'static, AnyPin>,
    pub ri: Input<'static, AnyPin>,
}

impl ModemPowerPins {
    async fn press_power_key(&mut self, millis: u32) {
        self.power_key.set_low();
        Timer::after(Duration::from_millis(100)).await;

        //based on schematics the power key is active low on MCU side
        self.power_key.set_high();
        Timer::after(Duration::from_millis(millis as u64)).await;
        self.power_key.set_low();
        defmt::info!("power key pressed for {}ms", millis);
    }

    fn is_enabled(&self) -> bool {
        let status = self.status.is_high();
        defmt::info!(
            "modem is currently {}",
            if status { "enabled" } else { "disabled" }
        );
        status
    }
}

impl ModemPower for ModemPowerPins {
    type EnableFuture<'a> = impl Future<Output = ()> + 'a
    where
        Self: 'a;
    type DisableFuture<'a> = impl Future<Output = ()> + 'a
    where
        Self: 'a;
    type SleepFuture<'a> = impl Future<Output = ()> + 'a
    where
        Self: 'a;
    type WakeFuture<'a> = impl Future<Output = ()> + 'a
    where
        Self: 'a;
    type ResetFuture<'a> = impl Future<Output = ()> + 'a
    where
        Self: 'a;

    fn enable(&mut self) -> Self::EnableFuture<'_> {
        async {
            defmt::info!("enabling modem");
            //poor datasheet gives only min, not max timeout
            if self.is_enabled() {
                defmt::info!("modem was enabled already");
                return;
            }
            self.press_power_key(1100).await;
            while self.status.is_low() {
                Timer::after(Duration::from_millis(100)).await;
            }
            defmt::info!("modem enabled");
        }
    }

    fn disable(&mut self) -> Self::DisableFuture<'_> {
        async {
            defmt::info!("disabling modem");
            //poor datasheet gives only min, not max timeout
            if !self.is_enabled() {
                defmt::info!("modem was disabled already");
                return;
            }
            self.press_power_key(1300).await;
            while self.status.is_high() {
                Timer::after(Duration::from_millis(100)).await;
            }
            defmt::info!("modem disabled");
        }
    }

    fn sleep(&mut self) -> Self::SleepFuture<'_> {
        async {
            self.dtr.set_high();
        }
    }

    fn wake(&mut self) -> Self::WakeFuture<'_> {
        async {
            self.dtr.set_low();
        }
    }

    fn reset(&mut self) -> Self::ResetFuture<'_> {
        async {
            self.reset.set_high();
            // Reset pin needs to be held low for 252ms. Wait for 300ms to ensure it works.
            Timer::after(Duration::from_millis(300)).await;
            self.reset.set_low();
        }
    }

    fn state(&mut self) -> sim7000_async::PowerState {
        match self.status.is_high() {
            true => PowerState::On,
            false => PowerState::Off,
        }
    }
}
