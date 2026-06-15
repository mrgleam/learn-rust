use std::time::Instant;

fn main() {
    let customer_ids: Vec<u64> = (1..=50000).collect();
    println!("================================================================================");
    println!(
        "Benchmarking Customer Report Strategies ({} Customers)",
        customer_ids.len()
    );
    println!("================================================================================");

    // 1. Base Case: Naive Algo + Slow Logging (O(N^2) total)
    println!("\n[1/4] Running Base Case: Naive Algo + Slow Logging...");
    let start_base = Instant::now();
    let report_base = process_customer_report_base(&customer_ids);
    let dur_base = start_base.elapsed();
    println!(
        "      Report length: {} bytes, Time: {:?}",
        report_base.len(),
        dur_base
    );

    // 2. Strategy A: Improved Algo + Slow Logging
    println!("\n[2/4] Running Strategy A: Improved Algo + Slow Logging...");
    let start_strat_a = Instant::now();
    let report_strat_a = process_customer_report_strat_a(&customer_ids);
    let dur_strat_a = start_strat_a.elapsed();
    println!(
        "      Report length: {} bytes, Time: {:?}",
        report_strat_a.len(),
        dur_strat_a
    );

    // 3. Strategy B: Naive Algo + Fast Logging
    println!("\n[3/4] Running Strategy B: Naive Algo + Fast Logging...");
    let start_strat_b = Instant::now();
    let report_strat_b = process_customer_report_strat_b(&customer_ids);
    let dur_strat_b = start_strat_b.elapsed();
    println!(
        "      Report length: {} bytes, Time: {:?}",
        report_strat_b.len(),
        dur_strat_b
    );

    // 4. Strategy C: Improved Algo + Fast Logging (Fully Optimized)
    println!("\n[4/4] Running Strategy C: Improved Algo + Fast Logging...");
    let start_strat_c = Instant::now();
    let report_strat_c = process_customer_report_strat_c(&customer_ids);
    let dur_strat_c = start_strat_c.elapsed();
    println!(
        "      Report length: {} bytes, Time: {:?}",
        report_strat_c.len(),
        dur_strat_c
    );

    // --- Performance Comparison Table ---
    println!("\n================================================================================");
    println!("                               PERFORMANCE SUMMARY                              ");
    println!("================================================================================");
    println!(
        "{:<35} | {:<12} | {:<12} | {:<15} | {:<10}",
        "Strategy", "Algo Comp.", "Log Comp.", "Elapsed Time", "Speedup"
    );
    println!("--------------------------------------------------------------------------------");

    let format_speedup = |dur: std::time::Duration| -> String {
        if dur.as_nanos() == 0 {
            "N/A".to_string()
        } else {
            let speedup = dur_base.as_secs_f64() / dur.as_secs_f64();
            format!("{:.2}x", speedup)
        }
    };

    println!(
        "{:<35} | {:<12} | {:<12} | {:<15?} | {:<10}",
        "1. Naive Algo + Slow Log (Base)", "O(N)", "O(M^2)", dur_base, "1.00x"
    );
    println!(
        "{:<35} | {:<12} | {:<12} | {:<15?} | {:<10}",
        "2. Strategy A (Improve Algo)",
        "O(sqrt(N))",
        "O(M^2)",
        dur_strat_a,
        format_speedup(dur_strat_a)
    );
    println!(
        "{:<35} | {:<12} | {:<12} | {:<15?} | {:<10}",
        "3. Strategy B (Improve Logging)",
        "O(N)",
        "O(M)",
        dur_strat_b,
        format_speedup(dur_strat_b)
    );
    println!(
        "{:<35} | {:<12} | {:<12} | {:<15?} | {:<10}",
        "4. Strategy C (Improve Both)",
        "O(sqrt(N))",
        "O(M)",
        dur_strat_c,
        format_speedup(dur_strat_c)
    );
    println!("================================================================================");
    println!("Note: N is the customer ID range (value), M is the number of customer records.");
    println!("");
    println!("Analysis:");
    println!("- Strategy A (Improving the algorithm to O(sqrt(N)) but keeping slow logging)");
    println!("  makes prime checks fast, but still has quadratic string-copy overhead.");
    println!("- Strategy B (Improving logging to O(M) append but keeping O(N) naive checks)");
    println!(
        "  avoids the copy reallocation overhead but still performs slow loops for prime checking."
    );
    if dur_strat_a.as_nanos() > 0 {
        println!(
            "- Direct Comparison: Strategy A is {:.2}x faster than Strategy B!",
            dur_strat_b.as_secs_f64() / dur_strat_a.as_secs_f64()
        );
    }
    println!("================================================================================");
}

// ==========================================
// 1. Prime Checker Implementations
// ==========================================

/// Naive prime check: checks every number from 2 up to id - 1 (O(N))
fn is_ultra_vip_id_naive(id: u64) -> bool {
    if id <= 1 {
        return false;
    }
    for i in 2..id {
        if id % i == 0 {
            return false;
        }
    }
    true
}

/// Improved prime check: checks numbers from 2 up to sqrt(id) (O(sqrt(N)))
fn is_ultra_vip_id_improved(id: u64) -> bool {
    if id <= 1 {
        return false;
    }
    let limit = (id as f64).sqrt() as u64;
    for i in 2..=limit {
        if id % i == 0 {
            return false;
        }
    }
    true
}

// ==========================================
// 2. Log Concatenation Strategies
// ==========================================

/// Slow O(M^2) concatenation (forces copy-reallocation)
fn append_log_slow(report: &mut String, id: u64, is_vip: bool) {
    let timestamp = "2026-06-15 23:30:00";
    let status = if is_vip { "ULTRA-VIP" } else { "NORMAL" };
    let log_line = format!(
        "[{}] Customer ID: {} -> Status: {}\n",
        timestamp, id, status
    );
    *report = format!("{}{}", report, log_line);
}

/// Fast O(M) concatenation (in-place append)
fn append_log_fast(report: &mut String, id: u64, is_vip: bool) {
    let timestamp = "2026-06-15 23:30:00";
    let status = if is_vip { "ULTRA-VIP" } else { "NORMAL" };
    let log_line = format!(
        "[{}] Customer ID: {} -> Status: {}\n",
        timestamp, id, status
    );
    report.push_str(&log_line);
}

// ==========================================
// 3. Process Report Functions
// ==========================================

// 1. Naive Algo + Slow Logging (Base Case)
pub fn process_customer_report_base(customer_ids: &[u64]) -> String {
    let mut final_report = String::new();
    for &id in customer_ids {
        let is_vip = is_ultra_vip_id_naive(id);
        append_log_slow(&mut final_report, id, is_vip);
    }
    final_report
}

// 2. Improved Algo + Slow Logging (Strategy A)
pub fn process_customer_report_strat_a(customer_ids: &[u64]) -> String {
    let mut final_report = String::new();
    for &id in customer_ids {
        let is_vip = is_ultra_vip_id_improved(id);
        append_log_slow(&mut final_report, id, is_vip);
    }
    final_report
}

// 3. Naive Algo + Fast Logging (Strategy B)
pub fn process_customer_report_strat_b(customer_ids: &[u64]) -> String {
    let mut final_report = String::new();
    for &id in customer_ids {
        let is_vip = is_ultra_vip_id_naive(id);
        append_log_fast(&mut final_report, id, is_vip);
    }
    final_report
}

// 4. Improved Algo + Fast Logging (Strategy C)
pub fn process_customer_report_strat_c(customer_ids: &[u64]) -> String {
    let mut final_report = String::new();
    for &id in customer_ids {
        let is_vip = is_ultra_vip_id_improved(id);
        append_log_fast(&mut final_report, id, is_vip);
    }
    final_report
}
