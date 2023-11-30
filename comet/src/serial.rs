use serialport::Error;

pub fn list_available_ports() -> Result<Vec<serialport::SerialPortInfo>, Error> {
    let ports = serialport::available_ports()?;
    Ok(ports)
}
