
// standard libraries
#![no_std]

// no mangle
#[no_mangle]
pub extern "C" fn cmain() -> ! {
  //
  let mut vaddr : u64 = 0xb8000;
  let mut uart0 = vaddr as *mut u8;
  let greeting = b"Hello, I am The Anzu OS. Welcome to our World!";

  for c in greeting.iter() {
    unsafe {
      *uart0 = *c as u8;
      vaddr = vaddr + (2 as u64);
      uart0 = vaddr as *mut u8;
    }
  }

  loop {}
}

#[no_mangle]
pub extern "C" fn printf() -> ! {
  //
  loop {}

}


use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
  //
  loop {}
}

//
#[no_mangle]
pub fn abort() -> ! {
  //
  loop {}
}


