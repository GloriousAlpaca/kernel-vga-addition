use memory_addresses::{PhysAddr, VirtAddr};
use pci_types::{Bar, CommandRegister};
use x86_64::instructions::port::Port;

use crate::arch::pci::PciConfigRegion;
use crate::arch::x86_64::mm::paging;
use crate::arch::x86_64::mm::paging::{BasePageSize, PageTableEntryFlags, PageTableEntryFlagsExt};
use crate::drivers::pci::PciDevice;

pub fn init_device(adapter: &PciDevice<PciConfigRegion>) {
	//To Do: Detect Resolution automatically
	let width: u16 = 640;
	let height: u16 = 400;
	let bpp: u16 = 32;

	unsafe {
		let mut vbe_dispi_ioport_index: Port<u16> = Port::new(0x01ce);
		let mut vbe_dispi_ioport_data: Port<u16> = Port::new(0x01cf);

		//disable VBE extensions as per docs
		vbe_dispi_ioport_index.write(4);
		vbe_dispi_ioport_data.write(0x00);

		//set width
		vbe_dispi_ioport_index.write(1);
		vbe_dispi_ioport_data.write(width);

		//set height
		vbe_dispi_ioport_index.write(2);
		vbe_dispi_ioport_data.write(height);

		//set bpp
		vbe_dispi_ioport_index.write(3);
		vbe_dispi_ioport_data.write(bpp);

		//enable VBE extensions and linear frame buffer
		vbe_dispi_ioport_index.write(4);
		vbe_dispi_ioport_data.write(0x41);
	}

	//set memory space bit in command register to enable memory access to the framebuffer
	adapter.set_command(CommandRegister::MEMORY_ENABLE);

	//get framebuffer address and size from BAR0
	let (phys, size) = match adapter.get_bar(0) {
		Some(Bar::Memory32 { address, size, .. }) => (u64::from(address), size as usize),
		Some(Bar::Memory64 { address, size, .. }) => (address, size as usize),
		_ => return, //not a memory BAR, can't use
	};

	let page_count = size / 4096;

	let mut flags = PageTableEntryFlags::empty();
	flags.device().writable().execute_disable();
	flags.insert(PageTableEntryFlags::USER_ACCESSIBLE);
	paging::map::<BasePageSize>(VirtAddr::new(phys), PhysAddr::new(phys), page_count, flags);
}
