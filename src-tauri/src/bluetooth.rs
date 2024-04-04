use std::{string, thread, time};


use btleplug::api::{bleuuid, PeripheralProperties};
use btleplug::api::{bleuuid::uuid_from_u16, Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral};


//setup shared structs
#[derive(Clone, serde::Serialize)]
struct BTStat {
    scanning: String,
    uncon_devices: String,
    conn_devices: String,
    bt_adapter: String,

}

pub async fn connect_device (app_handle: tauri::AppHandle)-> Result<Vec<String>, String> {

    let manager:Manager = Manager::new().await.unwrap();

    // get the first bluetooth adapter
    let adapters:Vec<Adapter> = manager.adapters().await.unwrap();
    let central: Adapter = adapters.into_iter().nth(0).unwrap();

    // start scanning for devices
    let _ = central.start_scan(ScanFilter::default()).await;
    thread::sleep(std::time::Duration::from_secs(2));
    let _ = central.stop_scan().await;

    //construct output vector
    let mut output_vec: Vec<String> = Vec::new();

    //if no peripherials connected, print error
    if central.peripherals().await.unwrap().len() == 0 {
        let adapter_info = central.adapter_info().await.unwrap();
        let _ = Err::<Vec<String>, String>(format!("No devices found on Adapter: [{}]", adapter_info));
    }

    for p in central.peripherals().await.unwrap() {
        let p_prop:PeripheralProperties = p.properties()
                                    .await
                                    .unwrap()
                                    .unwrap();

        let p_name = p_prop.local_name;
                                
        if let Some(p_name) = p_name {
            if !p_name.is_empty() {
                output_vec.push(p_name);
            }
        }
    }

    return Ok(output_vec);

}

pub async fn list_bt_devices() -> Result<Vec<String>, String> {

    let manager:Manager = Manager::new().await.unwrap();

    // get the first bluetooth adapter
    let adapters:Vec<Adapter> = manager.adapters().await.unwrap();
    let central: Adapter = adapters.into_iter().nth(0).unwrap();

    // start scanning for devices
    let _ = central.start_scan(ScanFilter::default()).await;
    thread::sleep(std::time::Duration::from_secs(2));
    let _ = central.stop_scan().await;

    //construct output vector
    let mut output_vec: Vec<String> = Vec::new();

    //if no peripherials connected, print error
    if central.peripherals().await.unwrap().len() == 0 {
        let adapter_info = central.adapter_info().await.unwrap();
        let _ = Err::<Vec<String>, String>(format!("No devices found on Adapter: [{}]", adapter_info));
    }

    for p in central.peripherals().await.unwrap() {
        let p_prop:PeripheralProperties = p.properties()
                                    .await
                                    .unwrap()
                                    .unwrap();

        let p_name = p_prop.local_name;
                                
        if let Some(p_name) = p_name {
            if !p_name.is_empty() {
                output_vec.push(p_name);
            }
        }
    }

    return Ok(output_vec);

}

