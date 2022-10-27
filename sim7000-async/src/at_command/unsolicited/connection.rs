use crate::at_command::{ATParseErr, ATParseLine};

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Connection {
    pub index: usize,
    pub message: ConnectionMessage,
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConnectionMessage {
    /// The connection was successfully established
    Connected,

    /// Failed to establish connection
    ConnectionFailed,

    /// A connection already exists on this index
    AlreadyConnected,

    /// A message was successfully sent
    SendSuccess,

    /// Failed to send message
    SendFail,

    /// The connection was closed
    Closed,
}

impl ATParseLine for Connection {
    fn from_line(line: &str) -> Result<Self, ATParseErr> {
        //defmt::info!("Mes0");

        //let (index, message) = line.split_once(", ").ok_or("Missing ', '")?;
        //let index = index.parse()?;

        let message = line.strip_prefix("+CIPOPEN: ").ok_or("Missing '+CIPOPEN: '")?;
        defmt::info!("Me0={}",line );
        defmt::info!("Me1={}",message );
        let (index, message) = message.split_once(",").ok_or("Missing ','")?;
        defmt::info!("Me2={},{}",index, message );
        let index = index.parse()?;

        defmt::info!("Me3={}",index );
        defmt::info!("Me4={}",message );

        use ConnectionMessage::*;
        let message = match message {
            /*
            "CLOSED" => Closed, 
            "SEND OK" => SendSuccess,
            "SEND FAIL" => SendFail,
            "CONNECT OK" => Connected,
            //"+CIPOPEN: 0,0" => Connected, //Niklas 2022
            "CONNECT FAIL" => ConnectionFailed,
            "ALREADY CONNECT" => AlreadyConnected,
            */
            "0" => Connected, //Niklas 2022
            "1" => ConnectionFailed, //Niklas 2022
            "2" => SendFail, //Niklas 2022
            _ => {
                return Err("Invalid connection message".into());
            }
        };
        //Ok(Connection { message }) //niklas
        Ok(Connection { index, message })
    }
}
