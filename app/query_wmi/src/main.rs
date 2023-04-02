use query_wmi::{COMLibrary, Variant, WMIConnection};
use query_wmi::computer_hardware::{
    get_Win32_CDROMDrive, get_Win32_ComputerSystem,
    get_Win32_PCMCIAController, get_Win32_PnPEntity, get_Win32_Processor,
    get_Win32_SystemEnclosure, get_Win32_TapeDrive, get_Win32_USBHub,
};
use query_wmi::operating_systems::get_Win32_OperatingSystem;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new()?;
    dbg!(get_Win32_OperatingSystem(com_con)?);
    dbg!(get_Win32_CDROMDrive(com_con)?);
    dbg!(get_Win32_ComputerSystem(com_con)?);
    dbg!(get_Win32_PCMCIAController(com_con)?);
    dbg!(get_Win32_PnPEntity(com_con)?);
    dbg!(get_Win32_Processor(com_con)?);
    dbg!(get_Win32_SystemEnclosure(com_con)?);
    dbg!(get_Win32_USBHub(com_con)?);
    dbg!(get_Win32_TapeDrive(com_con)?);
    Ok(())
}