<script lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import Button from './basic_components/Button.svelte';

var device_array = new Array<String>();
var last_dev_arry = new Array<String>();
const default_slct_msg = "Refresh Devices";
//init to default state
device_array.push(default_slct_msg);

//global variables to stop bt scanning
var enable_bt_scan = false;
var scheduled_scan_id:number = 0;
//

//which device is selected
var sel_device: String = "";

//enable scan
async function start_bt_scan(){
    enable_bt_scan = true;
    
    //run function which schedules another run
    await push_scan_update(2);
}

//stop the scan from moving
async function stop_bt_scan(){
    //function will stop next itteration
    enable_bt_scan = false;
    clearTimeout(scheduled_scan_id)
}

//push update to the list and re schedule itself
async function push_scan_update(rate:number = 1) {

    //only update array if needed
    last_dev_arry = device_array;
    var temp_array:String[] = await invoke("list_bt_devices").then();

    if (last_dev_arry != temp_array) {
        console.log("devices found: ["+device_array+"]");
        device_array = temp_array;
        console.log("BT Scan Enabled: "+enable_bt_scan)
    }

    //TODO: timeout after a reasonable time (1-5min?)
    //reschedule another run
    if (enable_bt_scan) {
        scheduled_scan_id = setTimeout(push_scan_update, 1/(rate*1000), rate)
    }
}
</script>



<div id="ble_manager">
    <Button on:click = {start_bt_scan} buttonText = "Start Scan" buttonID = "bth_refresh_devices"/>

    <Button on:click = {stop_bt_scan} buttonText = "Stop Scan" buttonID = "bth_refresh_devices"/>

    <!--drop down selector-->
    <select bind:value={sel_device}>
        {#each device_array as device, index (index)}
           <option value={index}>{device}</option>
        {/each}
    </select>

    <Button on:click = {connectDevice} buttonText = "Connect" buttonID = "bth_refresh_devices"/>
</div>