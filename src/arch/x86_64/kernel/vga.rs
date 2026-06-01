use memory_addresses::{PhysAddr, VirtAddr};
use x86_64::instructions::port::Port;

use crate::arch::x86_64::mm::paging;
use crate::arch::x86_64::mm::paging::{BasePageSize, PageTableEntryFlags, PageTableEntryFlagsExt};
use crate::arch::pci::PciConfigRegion;
use crate::drivers::pci::PciDevice;

const VGA_BUFFER_ADDRESS: PhysAddr = PhysAddr::new(0xE0000000);

pub fn write_byte(_byte: u8) {

}

pub fn init(){
/*    let width: u16 = 640;
    let height: u16 = 480;
    let bpp: u16 = 32;

    unsafe{
        let mut VBE_DISPI_IOPORT_INDEX: Port<u16> = Port::new(0x01CE);
        let mut VBE_DISPI_IOPORT_DATA: Port<u16> = Port::new(0x01CF);
        
        //disable VBE extensions as per docs
        VBE_DISPI_IOPORT_INDEX.write(4);
        VBE_DISPI_IOPORT_DATA.write(0x00);

        //set width
        VBE_DISPI_IOPORT_INDEX.write(1);
        VBE_DISPI_IOPORT_DATA.write(width);

        //set height
        VBE_DISPI_IOPORT_INDEX.write(2);
        VBE_DISPI_IOPORT_DATA.write(height);

        //set bpp
        VBE_DISPI_IOPORT_INDEX.write(3);
        VBE_DISPI_IOPORT_DATA.write(bpp);
        
        //enable VBE extensions and linear frame buffer
        VBE_DISPI_IOPORT_INDEX.write(4);
        VBE_DISPI_IOPORT_DATA.write(0x41);

    }
    
    let num_pages = (width as usize * height as usize * 4) / 4096;

    let mut flags = PageTableEntryFlags::empty();
		flags.device().writable().execute_disable();
        flags.insert(PageTableEntryFlags::USER_ACCESSIBLE);
		paging::map::<BasePageSize>(
			VirtAddr::new(VGA_BUFFER_ADDRESS.as_u64()),
			VGA_BUFFER_ADDRESS,
            1,
			flags,
		);*/
}


pub fn init_device(adapter: &PciDevice<PciConfigRegion>) {
    let width: u16 = 640;
    let height: u16 = 240;
    let bpp: u16 = 32;
    unsafe{
        let mut VBE_DISPI_IOPORT_INDEX: Port<u16> = Port::new(0x01CE);
        let mut VBE_DISPI_IOPORT_DATA: Port<u16> = Port::new(0x01CF);
        
        //disable VBE extensions as per docs
        VBE_DISPI_IOPORT_INDEX.write(4);
        VBE_DISPI_IOPORT_DATA.write(0x00);

        //set width
        VBE_DISPI_IOPORT_INDEX.write(1);
        VBE_DISPI_IOPORT_DATA.write(width);

        //set height
        VBE_DISPI_IOPORT_INDEX.write(2);
        VBE_DISPI_IOPORT_DATA.write(height);

        //set bpp
        VBE_DISPI_IOPORT_INDEX.write(3);
        VBE_DISPI_IOPORT_DATA.write(bpp);
        
        //enable VBE extensions and linear frame buffer
        VBE_DISPI_IOPORT_INDEX.write(4);
        VBE_DISPI_IOPORT_DATA.write(0x41);
    }
}
