use crate::{
    graph_view::MmapState,
    telemetry::{i8_array32_to_string, i8_array64_to_string, update_telemetry, TelemetryState},
};
use std::sync::Mutex;
use tauri::State;

#[derive(Debug, serde::Serialize, Clone)]
pub struct ChampDriverData {
    name: String,
    car: String,
    car_class: String,
    id: i32,
}

#[tauri::command]
pub async fn get_champ_drivers(
    telemetry_state: State<'_, Mutex<TelemetryState>>,
    mmap: State<'_, MmapState>,
) -> Result<Vec<ChampDriverData>, String> {
    if !telemetry_state.lock().unwrap().full_mode {
        return Err("Telemetry is not in full mode!".into());
    }

    let telemetry = update_telemetry(&mmap.mmap).unwrap();
    let mut drivers: Vec<ChampDriverData> = Vec::new();
    for i in 0..103 {
        let name = i8_array32_to_string(&telemetry.scoring.veh_scoring_info[i].m_driver_name);
        let car = i8_array64_to_string(&telemetry.scoring.veh_scoring_info[i].m_vehicle_name);
        let car_class =
            i8_array32_to_string(&telemetry.scoring.veh_scoring_info[i].m_vehicle_class);
        if name != "" {
            drivers.push(ChampDriverData {
                name,
                car,
                car_class,
                id: i as i32,
            });
        }
    }
    Ok(drivers)
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct StaleDriverInfo {
    name: String,
    car: String,
    car_class: String,
}

#[tauri::command]
pub async fn get_stale_driver_info(
    telemetry_state: State<'_, Mutex<TelemetryState>>,
    mmap: State<'_, MmapState>,
    cur_driver_id: usize,
) -> Result<StaleDriverInfo, String> {
    if !telemetry_state.lock().unwrap().full_mode {
        return Err("Telemetry is not in full mode!".into());
    }

    let telemetry = update_telemetry(&mmap.mmap).unwrap();

    let name =
        i8_array32_to_string(&telemetry.scoring.veh_scoring_info[cur_driver_id].m_driver_name);
    let car =
        i8_array64_to_string(&telemetry.scoring.veh_scoring_info[cur_driver_id].m_vehicle_name);
    let car_class =
        i8_array32_to_string(&telemetry.scoring.veh_scoring_info[cur_driver_id].m_vehicle_class);

    Ok(StaleDriverInfo {
        name,
        car,
        car_class,
    })
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct DynDriverInfo {
    pub damages: Vec<DamageData>,
    pub tires: Tires,
    pub fuel: FuelData,
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct DamageData {
    pub damage_msg: String,
    pub severity: Severity,
}

#[derive(Debug, serde::Serialize, Clone)]
pub enum Severity {
    Minor,
    Moderate,
    Major,
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct Tires {
    pub fl: Tire,
    pub fr: Tire,
    pub rl: Tire,
    pub rr: Tire,
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct Tire {
    pub health: f64, //0-1 percentage
    pub inside_temp: f64,
    pub outside_temp: f64,
    pub brake_temp: f64,
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct FuelData {
    pub fuel: f64,
    pub max_fuel: f64,
    pub ve: f32,
}

#[tauri::command]
pub async fn get_dyn_driver_info(
    telemetry_state: State<'_, Mutex<TelemetryState>>,
    mmap: State<'_, MmapState>,
    cur_driver_id: usize,
) -> Result<DynDriverInfo, String> {
    if !telemetry_state.lock().unwrap().full_mode {
        return Err("Telemetry is not in full mode!".into());
    }

    let telemetry = update_telemetry(&mmap.mmap).unwrap();

    let wheels = &telemetry.telemetry.telemetry_info[cur_driver_id].m_wheel;
    let wheel0 = wheels[0];
    let wheel1 = wheels[1];
    let wheel2 = wheels[2];
    let wheel3 = wheels[3];

    let inner_temp0 = wheel0.m_tire_inner_layer_temperature;
    let outside_temp0 = wheel0.m_temperature;
    let inner_temp1 = wheel1.m_tire_inner_layer_temperature;
    let outside_temp1 = wheel1.m_temperature;
    let inner_temp2 = wheel2.m_tire_inner_layer_temperature;
    let outside_temp2 = wheel2.m_temperature;
    let inner_temp3 = wheel3.m_tire_inner_layer_temperature;
    let outside_temp3 = wheel3.m_temperature;

    let damages: Vec<DamageData> = Vec::new();
    let tires = Tires {
        fl: Tire {
            health: wheel0.m_wear,
            inside_temp: inner_temp0.iter().sum::<f64>() / 3.0 - 273.15,
            outside_temp: outside_temp0.iter().sum::<f64>() / 3.0 - 273.15,
            brake_temp: wheel0.m_brake_temp - 273.15,
        },
        fr: Tire {
            health: wheel1.m_wear,
            inside_temp: inner_temp1.iter().sum::<f64>() / 3.0 - 273.15,
            outside_temp: outside_temp1.iter().sum::<f64>() / 3.0 - 273.15,
            brake_temp: wheel1.m_brake_temp - 273.15,
        },
        rl: Tire {
            health: wheel2.m_wear,
            inside_temp: inner_temp2.iter().sum::<f64>() / 3.0 - 273.15,
            outside_temp: outside_temp2.iter().sum::<f64>() / 3.0 - 273.15,
            brake_temp: wheel2.m_brake_temp - 273.15,
        },
        rr: Tire {
            health: wheel3.m_wear,
            inside_temp: inner_temp3.iter().sum::<f64>() / 3.0 - 273.15,
            outside_temp: outside_temp3.iter().sum::<f64>() / 3.0 - 273.15,
            brake_temp: wheel3.m_brake_temp - 273.15,
        },
    };
    let fuel = FuelData {
        fuel: telemetry.telemetry.telemetry_info[cur_driver_id].m_fuel,
        max_fuel: telemetry.telemetry.telemetry_info[cur_driver_id].m_fuel_capacity,
        ve: telemetry.telemetry.telemetry_info[cur_driver_id].m_virtual_energy,
    };

    Ok(DynDriverInfo {
        damages,
        tires,
        fuel,
    })
}
