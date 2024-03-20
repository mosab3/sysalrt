use sysinfo::{System, MINIMUM_CPU_UPDATE_INTERVAL};

fn sys_mon() -> System {
    let system = System::new_all();

    return  system;
}

pub fn cpu_usage() -> u64 {
    let mut sysmon = sys_mon();
        
    sysmon.refresh_cpu();
    std::thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
    sysmon.refresh_cpu();

    let global_cpu = sysmon.global_cpu_info();
    
    let cpu_percentage: u64 = global_cpu.cpu_usage().round() as u64;
    return cpu_percentage;
}



pub fn memory_usage() -> u64 {
    let mut sysmon = sys_mon();
    
    sysmon.refresh_memory();

    let memory_percentage: u64 = (sysmon.used_memory() * 100) / sysmon.total_memory();

    return  memory_percentage;

}

#[test]
fn test_cpu_usage() {
    println!("{}%", cpu_usage())
}

#[test]
fn test_memory_usage() {
    println!("{}%", memory_usage())
}
