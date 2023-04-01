extern crate vulkano;
use std::env;
use std::process::exit;
use vulkano::VulkanLibrary;
use vulkano::instance::{Instance, InstanceCreateInfo};

fn print_help() {
    println!("Usage: lsvkdev <arg>
    no arg  Show all Vulkan devices names 
    -c      Show current Vulkan device name
    -d      Show current Vulkan device driver name
    -ad     Show all Vulkan devices driver names 
    -h      Show this usage info");
}

fn main() {
    let exec_args: Vec<String> = env::args().collect();

    let library = VulkanLibrary::new()
        .unwrap_or_else(|error| {
            eprintln!("No local Vulkan library/DLL: {}", error);
            exit(1);
        });

    let instance = Instance::new(library, InstanceCreateInfo::default())
        .unwrap_or_else(|error| {
            eprintln!("Failed to create Vulkan instance: {}", error);
            exit(1);
        });

    let physical_device = instance
        .enumerate_physical_devices()
        .unwrap_or_else(|error| {
            eprintln!("Could not enumerate Vulkan devices: {}", error);
            exit(1);
        }).next().unwrap_or_else(|| {
            eprintln!("No Vulkan devices available!");
            exit(1);
        });

    let physical_devices = instance
        .enumerate_physical_devices()
        .unwrap_or_else(|error| {
            eprintln!("Could not enumerate Vulkan devices: {}", error);
            exit(1);
        });

    match exec_args.len() {
        1 => {
            for device in physical_devices {
                println!("{}", &device.properties().device_name);
            };
        },
        2 => {
            match exec_args[1].as_str() {
                "-c" => {
                    println!("{}", &physical_device.properties().device_name);
                },
                "-d" => {
                    println!("{}", &physical_device.properties().driver_name.clone().unwrap());
                },
                "-ad" => {
                    for device in physical_devices {
                        println!("{}", &device.properties().driver_name.clone().unwrap());
                    };
                },
                "-h" => {
                    print_help();
                },
                _ => {
                    eprintln!("Error: invalid argument!");
                    print_help();
                },
            }
        },
        _ => {}
    }
}
