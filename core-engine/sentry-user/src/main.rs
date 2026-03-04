mod clbd; // Imports the clbd.rs file you created

use aya::maps::perf::AsyncPerfEventArray;
use aya::programs::TracePoint;
use aya::util::online_cpus;
use aya::Bpf;
use bytes::BytesMut;
use tokio::task;

// Imports the data structure from Member 1's crate
use latent_sentry_common::HardwareEvent; 

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    println!("🛡️ Starting Latent-Sentry User-Space Agent...");

    // -------------------------------------------------------------------
    // STEP 1: LOAD MEMBER 2'S RULEBOOK (THE BYTECODE)
    // -------------------------------------------------------------------
    // Note: The path might change depending on where you run the cargo command
    let mut bpf = Bpf::load_file("../target/bpfel-unknown-none/release/latent-sentry-ebpf")?;
    println!("✅ eBPF Bytecode successfully loaded into the Kernel.");

    // -------------------------------------------------------------------
    // STEP 2: ATTACH TO THE TRACEPOINT (THE DOOR)
    // -------------------------------------------------------------------
    let program: &mut TracePoint = bpf.program_mut("latent_sentry").unwrap().try_into()?;
    program.load()?;
    program.attach("syscalls", "sys_enter_read")?;
    println!("✅ eBPF attached to tracepoint: sys_enter_read.");

    // -------------------------------------------------------------------
    // STEP 3: SET UP THE RING BUFFER LISTENER
    // -------------------------------------------------------------------
    let mut perf_array = AsyncPerfEventArray::try_from(bpf.map_mut("EVENTS").unwrap())?;

    // Listen to all CPU cores
    for cpu_id in online_cpus()? {
        let mut buf = perf_array.open(cpu_id, None)?;

        task::spawn(async move {
            let mut buffers = (0..10)
                .map(|_| BytesMut::with_capacity(1024))
                .collect::<Vec<_>>();

            loop {
                // Wait for Member 2 to drop a HardwareEvent into the buffer
                let events = buf.read_events(&mut buffers).await.unwrap();
                for i in 0..events.read {
                    let buf = &mut buffers[i];
                    
                    // Safely convert the raw bytes back into the HardwareEvent struct
                    let event = unsafe { std::ptr::read(buf.as_ptr() as *const HardwareEvent) };
                    
                    // PASS THE EVENT TO YOUR CLBD ENGINE IN THE OTHER FILE
                    clbd::run_clbd_engine(event).await;
                }
            }
        });
    }

    println!("📡 Listening for Hardware I/O anomalies... Press Ctrl+C to stop.");
    tokio::signal::ctrl_c().await?;
    println!("Exiting Latent-Sentry...");
    Ok(())
}
