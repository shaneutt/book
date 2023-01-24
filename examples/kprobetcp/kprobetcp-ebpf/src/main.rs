#![no_std]
#![no_main]

#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
mod binding;

use aya_bpf::{macros::uprobe, programs::ProbeContext};
use aya_log_ebpf::info;

#[uprobe(name = "osslreadprobe")]
pub fn openssl_read_probe(ctx: ProbeContext) -> u32 {
    info!(&ctx, "UProbe OpenSSL Read");
    return 0;
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
