use core::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

use x86_64::instructions::port::Port;

use crate::arch::x86_64::kernel::interrupts;

const BUFFER_SIZE: usize = 256;
const ATOMIC_ZERO: AtomicU8 = AtomicU8::new(0);

static KEYBOARD_BUFFER: [AtomicU8; BUFFER_SIZE] = [ATOMIC_ZERO; BUFFER_SIZE];
static WRITE_INDEX: AtomicUsize = AtomicUsize::new(0);
static READ_INDEX: AtomicUsize = AtomicUsize::new(0);

#[allow(dead_code)]
pub(crate) fn get_keyboard_handler() -> (u8, fn()) {
	println!(">>> TRACE: get_keyboard_handler() wurde vom Kernel aufgerufen!");
	unsafe {
		let mut cmd_port = Port::<u8>::new(0x64);
		let mut data_port = Port::<u8>::new(0x60);

		// 1. Briefkasten freispülen (Alten Müll vom BIOS löschen)
		while (cmd_port.read() & 0x01) != 0 {
			let _ = data_port.read();
		}

		// 2. Konfiguration vom PS/2-Controller auslesen (Befehl 0x20)
		cmd_port.write(0x20);
		let mut config = data_port.read();

		// 3. Bit 0 aktivieren (= Hardware-Interrupts für die Tastatur einschalten!)
		config |= 0b0000_0001;

		// 4. Neue Konfiguration an den Controller zurückschreiben (Befehl 0x60)
		cmd_port.write(0x60);
		data_port.write(config);
	}
	fn keyboard_handler() {
		let mut port = Port::<u8>::new(0x60);
		let scancode = unsafe { port.read() };

		let write_idx = WRITE_INDEX.load(Ordering::Relaxed);
		let next_write_idx = write_idx.wrapping_add(1) % BUFFER_SIZE;
		let read_idx = READ_INDEX.load(Ordering::Acquire);
		println!("*** KEYBOARD INTERRUPT! Scancode: {:#04x} ***", scancode);
		//Check if the buffer is full (next write index would overwrite the read index)
		if next_write_idx != read_idx {
			KEYBOARD_BUFFER[write_idx].store(scancode, Ordering::Release);
			WRITE_INDEX.store(next_write_idx, Ordering::Release);
		} else {
			// Buffer is full, you might want to handle this case (e.g., drop the scancode or log
			// an error)
		}
	}

	interrupts::add_irq_name(1, "PS/2 Keyboard");

	(1, keyboard_handler)
}

// Pops a scancode from the keyboard buffer, returning `None` if the buffer is empty.
pub fn pop_scancode() -> Option<u8> {
	let read_idx = READ_INDEX.load(Ordering::Relaxed);
	let write_idx = WRITE_INDEX.load(Ordering::Acquire);

	if read_idx != write_idx {
		let scancode = KEYBOARD_BUFFER[read_idx].load(Ordering::Acquire);
		READ_INDEX.store(read_idx.wrapping_add(1) % BUFFER_SIZE, Ordering::Release);
		Some(scancode)
	} else {
		None
	}
}
