#![no_std]
#![no_main]

use aya_ebpf::{
    macros::tracepoint,
    programs::TracePointContext,
};
use aya_log_ebpf::info;

// =========================================================
// SENSOR 1: DISK MONITOR
// =========================================================
#[tracepoint(category = "syscalls", name = "sys_enter_read")]
pub fn latent_sentry_read(ctx: TracePointContext) -> u32 {
    match try_latent_sentry_read(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_latent_sentry_read(ctx: TracePointContext) -> Result<u32, u32> {
    let pid = aya_ebpf::helpers::bpf_get_current_pid_tgid() as u32;
    info!(&ctx, "[DISK] File read executed by PID: {}", pid);
    Ok(0)
}

// =========================================================
// SENSOR 2: GPU MONITOR
// =========================================================
#[tracepoint(category = "syscalls", name = "sys_enter_ioctl")]
pub fn latent_sentry_ioctl(ctx: TracePointContext) -> u32 {
    match try_latent_sentry_ioctl(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_latent_sentry_ioctl(ctx: TracePointContext) -> Result<u32, u32> {
    let pid = aya_ebpf::helpers::bpf_get_current_pid_tgid() as u32;
    info!(&ctx, "[GPU] Hardware control (ioctl) requested by PID: {}", pid);
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}