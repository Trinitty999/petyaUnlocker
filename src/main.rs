use libparted::*;
use colored::*;
use std::io::*;
use std::process;

fn list() -> Result<()> {
    for (dev_i, mut device) in Device::devices(true).enumerate() {
        let hw_geom = device.hw_geom();
        let bios_geom = device.bios_geom();

        println!("Device {}", dev_i);
        println!("    Model:         {:?}", device.model());
        println!("    Path:          {:?}", device.path());
        println!("    Sectors:       {}", device.length());
        println!("    Sector Size:   {}", device.sector_size());
        println!("    Type:          {:?}", device.type_());
        println!("    Open Count:    {}", device.open_count());
        println!("    Read Only:     {}", device.read_only());
        println!("    External Mode: {}", device.external_mode());
        println!("    Dirty:         {}", device.dirty());
        println!("    Boot Dirty:    {}", device.boot_dirty());
        println!("    Hardware Geometry:");
        println!("        Cylinders: {}", hw_geom.cylinders);
        println!("        Heads:     {}", hw_geom.heads);
        println!("        Sectors:   {}", hw_geom.sectors);
        println!("    BIOS Geometry:");
        println!("        Cylinders: {}", bios_geom.cylinders);
        println!("        Heads:     {}", bios_geom.heads);
        println!("        Sectors:   {}", bios_geom.sectors);
        println!("    Host:          {}", device.host());
        println!("    Did:           {}", device.did());

        let disk = Disk::new(&mut device)?;
        eprintln!("    Disk Type:    {:?}", disk.get_disk_type_name());

        for part in disk.parts() {
            println!("    Part {}", part.num());
            println!("        Type Name: {:?}", part.type_get_name());
            println!("        Name:      {:?}", part.name());
            println!("        Path:      {:?}", part.get_path());
            println!("        Active:    {}", part.is_active());
            println!("        Busy:      {}", part.is_busy());
            println!("        FS:        {:?}", part.fs_type_name());
            println!("        Start:     {}", part.geom_start());
            println!("        End:       {}", part.geom_end());
            println!("        Length:    {}".bold(), part.geom_length());
        }
    }
    

    Ok(())
}

fn main() -> Result<()> {
    list();
    print!("test".red);
    print!("Press Enter to exit.\n");

    let mut inp = String::new();

    stdin().read_line(&mut inp);

    Ok(())
}