use latent_sentry_common::HardwareEvent;
use std::process::Command;

// -------------------------------------------------------------------
// THE CLBD ENGINE & THREAT RATING SYSTEM
// -------------------------------------------------------------------
pub async fn run_clbd_engine(event: HardwareEvent) {
    // 1. Check if the OS knows about this PID (The Divergence Check)
    let is_hidden_from_os = check_if_hidden(event.pid);

    let mut threat_rating = 0;

    // 2. The Rating Logic
    if !is_hidden_from_os {
        threat_rating = 10; 
        // Normal behavior. Do nothing.
        return; 
    }

    // IF WE REACH HERE, THE PROCESS IS HIDDEN.
    if event.is_kernel_thread == 1 {
        // Rating 21-50: It's an Async Kernel Thread doing background work.
        threat_rating = 45;
        println!(
            "⚠️ [RATING 45] Suspicious Async Kernel Task (PID: {}) read {} bytes. Monitoring...",
            event.pid, event.bytes_read
        );
    } else {
        // Rating 81-100: It is NOT a kernel thread. It is a user-space Rootkit!
        threat_rating = 95;

        // Elevate to 100 if it's stealing massive AI weights (e.g., > 500 MB)
        if event.bytes_read > 500_000_000 {
            threat_rating = 100;
        }

        println!(
            "🚨 [RATING {}] CRITICAL THREAT: Killer Rootkit Detected (PID: {}) stealing {} bytes!",
            threat_rating, event.pid, event.bytes_read
        );

        // 3. ENFORCEMENT: The Kill Switch
        execute_kill_switch(event.pid);
        
        // 4. ALERTS: Send to Dashboard (Member 4's job)
        send_dashboard_alert(event.pid, threat_rating, event.bytes_read).await;
    }
}

// -------------------------------------------------------------------
// HELPER FUNCTIONS (Private to this module)
// -------------------------------------------------------------------

/// Checks the Linux /proc/ directory to see if the process actually exists to the OS.
fn check_if_hidden(pid: u32) -> bool {
    let proc_path = format!("/proc/{}", pid);
    let os_sees_it = std::path::Path::new(&proc_path).exists();
    
    // If the OS does NOT see it, it is hidden.
    !os_sees_it 
}

/// Uses the OS kill command to instantly terminate the malicious process.
fn execute_kill_switch(pid: u32) {
    println!("🛑 ENFORCEMENT: Issuing SIGKILL to PID {}...", pid);
    let output = Command::new("kill")
        .arg("-9") // -9 forces an immediate, unblockable kill
        .arg(pid.to_string())
        .output()
        .expect("Failed to execute kill command");

    if output.status.success() {
        println!("✅ ENFORCEMENT SUCCESS: Attacker process destroyed.");
    } else {
        println!("❌ ENFORCEMENT FAILED: Process might already be dead or requires higher privileges.");
    }
}

/// Mock function: Member 3 would use WebSockets to send this to Next.js
async fn send_dashboard_alert(pid: u32, rating: u32, bytes: u64) {
    let json_alert = format!(
        r#"{{"event": "divergence", "pid": {}, "rating": {}, "bytes_stolen": {}}}"#,
        pid, rating, bytes
    );
    // TODO: Send `json_alert` over WebSocket to Next.js
}
