mod command;
mod context;

use embassy_time::{with_timeout, Duration, Timer};
use heapless::Vec;

use crate::{
    at_command::{
        request::*,
        response::{SignalQuality, SystemInfo},
        unsolicited::{ConnectionMessage, RegistrationStatus},
    },
    drop::{AsyncDrop, DropMessage},
    gnss::Gnss,
    pump::{DropPump, RawIoPump, RxPump, TxPump},
    read::ModemReader,
    tcp::TcpStream,
    voltage::VoltageWarner,
    BuildIo, Error, ModemPower,
};
pub use command::{CommandRunner, CommandRunnerGuard, RawAtCommand};
pub use context::*;

pub struct Uninitialized;
pub struct Disabled;
pub struct Enabled;
pub struct Sleeping;

pub struct Modem<'c, P> {
    context: &'c ModemContext,
    commands: CommandRunner<'c>,
    power: P,
}

impl<'c, P: ModemPower> Modem<'c, P> {
    pub async fn new<I: BuildIo>(
        io: I,
        power: P,
        context: &'c ModemContext,
    ) -> Result<
        (
            Modem<'c, P>,
            RawIoPump<'c, I>,
            TxPump<'c>,
            RxPump<'c>,
            DropPump<'c>,
        ),
        Error,
    > {
        let modem = Modem {
            commands: context.commands(),
            context,
            power,
        };

        let io_pump = RawIoPump {
            io,
            rx: context.rx_pipe.writer(),
            tx: context.tx_pipe.reader(),
            power_state: false,
            power_signal: context.power_signal.dyn_subscriber().unwrap(),
        };

        let rx_pump = RxPump {
            reader: ModemReader::new(context.rx_pipe.reader()),
            generic_response: context.generic_response.sender(),
            registration_events: &context.registration_events,
            tcp: &context.tcp,
            gnss: context.gnss_slot.peek(),
            voltage_warning: context.voltage_slot.peek(),
        };

        let tx_pump = TxPump {
            writer: context.tx_pipe.writer(),
            commands: context.commands.receiver(),
        };

        let drop_pump = DropPump {
            context,
            power_signal: context.power_signal.dyn_subscriber().unwrap(),
            power_state: false,
        };

        Ok((modem, io_pump, tx_pump, rx_pump, drop_pump))
    }

    pub async fn init(&mut self) -> Result<(), Error> {
        self.deactivate().await;
        defmt::info!("S0");
        let publisher = self.context.power_signal.publisher().unwrap();
        publisher.publish(true).await;
        defmt::info!("S1");
        self.power.enable().await;
        defmt::info!("S2");

        let commands = self.commands.lock().await;
        defmt::info!("S3");
        let set_flow_control = SetFlowControl {
            dce_by_dte: FlowControl::Hardware,
            dte_by_dce: FlowControl::Hardware,
        };
        defmt::info!("S4");

        // Turn on hardware flow control, the modem does not save this state on reboot.
        // We need to set it as fast as possible to avoid dropping bytes.
        for _ in 0..5 {
            if let Ok(Ok(_)) = with_timeout(Duration::from_millis(2000), async {
                commands.run(set_flow_control).await
            })
            .await
            {
                break;
            }
        }
        defmt::info!("S5");
        // Modem has been known to get stuck in an unresponsive state until we jiggle it by
        // enabling echo. This is fine.
        for _ in 0..5 {
            if let Ok(Ok(_)) = with_timeout(
                Duration::from_millis(1000),
                commands.run(ate::SetEcho(true)),
            )
            .await
            {
                break;
            }
        }
        defmt::info!("S6");
        commands.run(csclk::SetSlowClock(false)).await?;
        defmt::info!("S7");
        commands.run(At).await?;
        defmt::info!("S8");
        commands.run(GetSignalQuality).await?;
        defmt::info!("S9");
        
        let (iccidvalue,ok)= commands.run(ShowIccid).await?;
        defmt::info!("country code{} {} {}", iccidvalue.country,iccidvalue.issuer,iccidvalue.account);
        defmt::info!("S10");
        commands.run(ipr::SetBaudRate(BaudRate::Hz115200)).await?;
        defmt::info!("S11");
        commands.run(set_flow_control).await?;
        commands
            .run(cmee::ConfigureCMEErrors(CMEErrorMode::Numeric))
            .await?;
        defmt::info!("S12");
        commands.run(Cipmode1).await?;
        defmt::info!("S13");
        commands.run(Csocksetpn).await?;
        //commands.run(cmnb::SetNbMode(NbMode::CatM)).await?;
        defmt::info!("S14");
        commands.run(Netopen).await?;
        //commands.run(cfgri::ConfigureRiPin(RiPinMode::On)).await?;
        defmt::info!("S15");
        //commands.run(cbatchk::EnableVBatCheck(true)).await?;
        defmt::info!("S16");
        /* 
        let configure_edrx = cedrxs::ConfigureEDRX {
            n: EDRXSetting::Enable,
            act_type: AcTType::CatM,
            requested_edrx_value: 0b0000,
        };
         
        defmt::info!("S14");

        for _ in 0..5 {
            match commands.run(configure_edrx).await {
                Ok(_) => break,
                _ => Timer::after(Duration::from_millis(200)).await,
            }
        }
        defmt::info!("S17");
        commands.run(configure_edrx).await?;
        defmt::info!("S18");
        core::mem::drop(commands);
        defmt::info!("S19");
        core::mem::drop(publisher);
        defmt::info!("S20");
       
        self.deactivate().await;
        */
        Ok(())
    }

    pub async fn activate(&mut self) -> Result<(), Error> {
        // unwrap is fine here since the modem is the only code creating publishers, there will always be a free slot.
        let publisher = self.context.power_signal.publisher().unwrap();
        publisher.publish(true).await;
        defmt::info!("A1");
        self.power.enable().await;
        let set_flow_control = ifc::SetFlowControl {
            dce_by_dte: FlowControl::Hardware,
            dte_by_dce: FlowControl::Hardware,
        };
        defmt::info!("A2");

        let commands = self.commands.lock().await;

        for _ in 0..5 {
            if let Ok(Ok(_)) = with_timeout(Duration::from_millis(2000), async {
                commands.run(set_flow_control).await
            })
            .await
            {
                break;
            }
        } 
        defmt::info!("A3");
        commands.run(ate::SetEcho(false)).await?;
        commands
            .run(cgreg::ConfigureRegistrationUrc::EnableRegLocation)
            .await?;

        self.wait_for_registration(&commands).await?;
        defmt::info!("A4");

        commands.run(cipmux::EnableMultiIpConnection(true)).await?;
        defmt::info!("A5");
        commands.run(cipshut::ShutConnections).await?;
        defmt::info!("A6");

        self.authenticate(&commands).await?;
        Ok(())
    }

    pub async fn deactivate(&mut self) {
        let publisher = self.context.power_signal.publisher().unwrap();
        publisher.publish(false).await;
        self.context.tcp.disconnect_all().await;

        self.power.disable().await;
    }

    async fn wait_for_registration(&self, commands: &CommandRunnerGuard<'_>) -> Result<(), Error> {
        loop {
            if with_timeout(Duration::from_millis(2000), async {
                commands.run(cgreg::GetRegistrationStatus).await
            })
            .await
            .is_err()
            {
                continue;
            }

            match self.context.registration_events.wait().await {
                RegistrationStatus::RegisteredHome | RegistrationStatus::RegisteredRoaming => break,
                _ => Timer::after(Duration::from_millis(200)).await,
            }
        }

        Ok(())
    }

    async fn authenticate(&self, commands: &CommandRunnerGuard<'_>) -> Result<(), Error> {
        commands
            .run(cstt::StartTask {
                apn: "iot.1nce.net".into(),
                username: "".into(),
                password: "".into(),
            })
            .await?;

        commands.run(ciicr::StartGprs).await?;

        let (_ip, _) = commands.run(cifsrex::GetLocalIpExt).await?;

        Ok(())
    }

    pub async fn connect_tcp(&mut self, host: &str, port: u16) -> Result<TcpStream<'c>, Error> {
        let tcp_context = self.context.tcp.claim().unwrap();

        self.commands
            .lock()
            .await
            .run(cipstart::Connect {
                mode: ConnectMode::Tcp,
                number: tcp_context.ordinal(),
                destination: host.try_into().map_err(|_| Error::BufferOverflow)?,
                port,
            })
            .await?;

        loop {
            match tcp_context.events().recv().await {
                ConnectionMessage::Connected => break,
                ConnectionMessage::ConnectionFailed => panic!("connection failed"), //TODO
                _ => {}
            }
        }

        Ok(TcpStream {
            _drop: AsyncDrop::new(
                &self.context.drop_channel,
                DropMessage::Connection(tcp_context.ordinal()),
            ),
            token: tcp_context,
            commands: self.commands.clone(),
            closed: false,
            buffer: Vec::new(),
        })
    }

    pub async fn claim_gnss(&mut self) -> Result<Option<Gnss<'c>>, Error> {
        let reports = match self.context.gnss_slot.claim() {
            Some(reports) => reports,
            None => return Ok(None),
        };

        self.commands
            .lock()
            .await
            .run(cgnspwr::SetGnssPower(true))
            .await?;

        self.commands
            .lock()
            .await
            .run(cgnsurc::ConfigureGnssUrc {
                period: 4, // TODO
            })
            .await?;

        Ok(Some(Gnss {
            _drop: AsyncDrop::new(&self.context.drop_channel, DropMessage::Gnss),
            reports,
        }))
    }

    pub async fn claim_voltage_warner(&mut self) -> Option<VoltageWarner<'c>> {
        VoltageWarner::take(&self.context.voltage_slot)
    }

    pub async fn query_system_info(&mut self) -> Result<SystemInfo, Error> {
        let (info, _) = self.commands.lock().await.run(cpsi::GetSystemInfo).await?;
        Ok(info)
    }

    pub async fn query_signal(&mut self) -> Result<SignalQuality, Error> {
        let (signal, _) = self
            .commands
            .lock()
            .await
            .run(csq::GetSignalQuality)
            .await?;
        Ok(signal)
    }
}
