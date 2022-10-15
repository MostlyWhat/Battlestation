use sysinfo::{System, SystemExt};

pub fn main() -> String {
    // Please note that we use "new_all" to ensure that all list of
    // components, network interfaces, disks and users are already
    // filled!
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    // Collect System Information
    let system_name = sys.name().unwrap();
    let system_version = sys.os_version().unwrap();
    let system_kernel_version = sys.kernel_version().unwrap();
    let system_host_name = sys.host_name().unwrap();

    // Memory Information in GB
    let system_memory_total = sys.total_memory() / 1024 / 1024 / 1024;
    let system_memory_used = sys.used_memory() / 1024 / 1024 / 1024;
    let system_memory_free = sys.free_memory() / 1024 / 1024 / 1024;

    // All Information Collected into a String
    let system_info = format!(
        "Loading System Information\n\
                                                    \n\
                                                    Battlestation Information:\n\
                                                    \n\
                                                    Firmware: MostlyWhat's Battlestation\n\
                                                    Version: 0.0.1 (October 2022 Build)\n\
                                                    Architecture: x86_64\n\
                                                    Language: Rust\n\
                                                    \n\
                                                    System Information:\n\
                                                    \n\
                                                    System Name: {}\n\
                                                    System Version: {}\n\
                                                    System Kernel Version: {}\n\
                                                    System Hostname: {}\n\
                                                    \n\
                                                    Memory Information:\n\
                                                    \n\
                                                    Total Memory: {} GB\n\
                                                    Used Memory: {} GB\n\
                                                    Free Memory: {} GB",
        system_name,
        system_version,
        system_kernel_version,
        system_host_name,
        system_memory_total,
        system_memory_used,
        system_memory_free
    );

    return system_info.to_string();
}
