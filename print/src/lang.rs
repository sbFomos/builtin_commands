use core::panic::PanicInfo;

#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
    loop {  }
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {    }