extern crate libloading;
use std::os::raw::{c_void, c_ulong};
use libloading::{Library, Symbol};

pub type pVirtualAlloc = unsafe extern "system" fn(
    lpAddress: *mut c_void,
    dwSize: usize,
    flAllocationType: c_ulong,
    flProtect: c_ulong,
) -> *mut c_void;

fn main() {
    let lib = unsafe {
        Library::new("kernel32.dll").expect("Couldn't load the kernel32.dll")
    };

    let virtual_alloc: Symbol<pVirtualAlloc> = unsafe {
        lib.get(b"VirtualAlloc\0").expect("Couldn't find the address of VirtualAlloc")

    };
    let size: usize = 1024;

    let return_address = unsafe {
        virtual_alloc(
            std::ptr::null_mut(),
            size,
            0x1000,
            0x04
        )
    };

    if return_address.is_null() {
        println!("Allocation failed");
    }
    else{
        println!("Allocation succeeded. The address allocated: {:?}", return_address);
    }

}
