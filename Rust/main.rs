use sysinfo::System;
use std::ptr;
use winapi::um::{
    memoryapi::{ReadProcessMemory, VirtualQueryEx},
    processthreadsapi::OpenProcess,
    winnt::{HANDLE, MEMORY_BASIC_INFORMATION, PROCESS_VM_READ, PROCESS_QUERY_INFORMATION},
};
use std::io::{self, Write};

fn get_process_id(PID_Name: &str) -> u32{
    let system = System::new_all();
    let mut PID: u32 = 0;

    for(pid, process) in system.processes() {
        if process.name() == PID_Name {
            //println!("{:?}: {:?}", process.name(), pid.as_u32());
            PID = pid.as_u32()
        }
    }

    PID
}

fn get_address(PID: u32, target_value: i32) -> Vec<String> {
    let process_handle: HANDLE = unsafe { OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, 0, PID) };
    if process_handle.is_null() {
        eprintln!("Failed to open process with PID {}", PID);
        return Vec::new();
    }

    let mut address_list = Vec::new();
    let mut address = 0usize;
    let mut mbi: MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };

    while unsafe {
        VirtualQueryEx(
            process_handle,
            address as *const _,
            &mut mbi,
            std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
        )
    } != 0
    {
        if mbi.State == winapi::um::winnt::MEM_COMMIT
            && (mbi.Protect & winapi::um::winnt::PAGE_READWRITE != 0)
        {
            let region_size = mbi.RegionSize;
            let mut buffer = vec![0u8; region_size];
            let mut bytes_read = 0;

            if unsafe {
                ReadProcessMemory(
                    process_handle,
                    mbi.BaseAddress,
                    buffer.as_mut_ptr() as *mut _,
                    region_size,
                    &mut bytes_read,
                )
            } != 0
            {
                // Search for the target value in the buffer
                let int_values = unsafe {
                    std::slice::from_raw_parts(buffer.as_ptr() as *const i32, bytes_read / std::mem::size_of::<i32>())
                };

                for (offset, &value) in int_values.iter().enumerate() {
                    if value == target_value {
                        let found_address = (mbi.BaseAddress as usize) + offset * std::mem::size_of::<i32>();
                        let formatted_address = format!("0x{:X}", found_address);
                        address_list.push(formatted_address);
                        println!("Found value at address: 0x{:X}, {value}", found_address);
                    }
                }
            }
        }

        address = mbi.BaseAddress as usize + mbi.RegionSize;
    }

    // Close the process handle
    unsafe { winapi::um::handleapi::CloseHandle(process_handle) };

    address_list 
}

fn read_values_at_addresses(PID: u32, addresses: Vec<String>, target_value: i32) {
    let process_handle: HANDLE = unsafe { OpenProcess(PROCESS_VM_READ, 0, PID) };
    if process_handle.is_null() {
        eprintln!("Failed to open process with PID {}", PID);
        return;
    }

    for address_str in &addresses {
        let address = usize::from_str_radix(&address_str[2..], 16).unwrap(); // Convert the string address back to usize
        let mut value = 0i32;
        let mut bytes_read = 0;

        // Read the value at the address
        if unsafe {
            ReadProcessMemory(
                process_handle,
                address as *const _,
                &mut value as *mut _ as *mut _,
                std::mem::size_of::<i32>(),
                &mut bytes_read,
            )
        } != 0 && bytes_read == std::mem::size_of::<i32>()
        {
            println!("Address: {} -> Value: {}", address_str, value);
        } else {
            eprintln!("Failed to read value at address: {}", address_str);
        }
    }

    unsafe { winapi::um::handleapi::CloseHandle(process_handle) };
}



fn main() {
    
    let PID = get_process_id("NGUIdle.exe");
    let address_book = get_address(PID, 868);

    println!("{:?}\nChange your number of idle energy please and enter the new value, click enter once value changed: ", address_book);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("Continuing execution...");

    read_values_at_addresses(PID, address_book, 858);

    //Use the address book and see what values have changed
}
