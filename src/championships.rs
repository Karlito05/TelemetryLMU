use crate::{
    frontend::car_info::{FuelInfo, TireInfo},
    telemetry_back::{IPVehicleClass, Telemetry, i8_array32_to_string, i8_array64_to_string},
};

pub struct StaleDriverInfo {
    pub name: String,
    pub car: String,
    pub car_class: IPVehicleClass,
    pub index: usize,
}

pub fn get_stale_driver_info(
    telemetry: &Telemetry,
    desired_name: String,
) -> Result<StaleDriverInfo, String> {
    let telemetry = telemetry.update_telemetry().unwrap();

    let mut drivers: Vec<(String, usize)> = Vec::new();

    for (i, car) in telemetry.scoring.veh_scoring_info.iter().enumerate() {
        let name = i8_array32_to_string(&car.m_driver_name);
        if !name.is_empty() {
            drivers.push((name, i));
        }
    }
    let cur_driver_id;

    if let Some(driver) = drivers.iter().find(|(name, _)| name == &desired_name) {
        cur_driver_id = driver.1
    } else {
        return Err("Driver not found".to_owned());
    }

    let name =
        i8_array32_to_string(&telemetry.scoring.veh_scoring_info[cur_driver_id].m_driver_name);
    let car =
        i8_array64_to_string(&telemetry.scoring.veh_scoring_info[cur_driver_id].m_vehicle_name);
    let car_class = telemetry.telemetry.telemetry_info[cur_driver_id].m_vehicle_class;

    Ok(StaleDriverInfo {
        name,
        car,
        car_class,
        index: cur_driver_id,
    })
}

pub struct DynDriverInfo {
    pub tires: [TireInfo; 4],
    pub fuel: FuelInfo,
}

pub fn get_dyn_driver_info(telemetry: &Telemetry, cur_driver_id: usize) -> DynDriverInfo {
    let telemetry = telemetry.update_telemetry().unwrap();

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

    let tires = [
        TireInfo {
            health_percent: wheel0.m_wear as f32,
            inside_temp: (inner_temp0.iter().sum::<f64>() / 3.0 - 273.15) as f32,
            outside_temp: (outside_temp0.iter().sum::<f64>() / 3.0 - 273.15) as f32,
            brake_temp: (wheel0.m_brake_temp - 273.15) as f32,
        },
        TireInfo {
            health_percent: wheel1.m_wear as f32,
            inside_temp: (inner_temp1.iter().sum::<f64>() / 3.0 - 273.15) as f32,
            outside_temp: (outside_temp1.iter().sum::<f64>() / 3.0 - 273.15) as f32,
            brake_temp: (wheel1.m_brake_temp - 273.15) as f32,
        },
        TireInfo {
            health_percent: wheel2.m_wear as f32,
            inside_temp: (inner_temp2.iter().sum::<f64>() / 3.0 - 273.15) as f32,
            outside_temp: (outside_temp2.iter().sum::<f64>() / 3.0 - 273.15) as f32,
            brake_temp: (wheel2.m_brake_temp - 273.15) as f32,
        },
        TireInfo {
            health_percent: wheel3.m_wear as f32,
            inside_temp: (inner_temp3.iter().sum::<f64>() / 3.0 - 273.15) as f32,
            outside_temp: (outside_temp3.iter().sum::<f64>() / 3.0 - 273.15) as f32,
            brake_temp: (wheel3.m_brake_temp - 273.15) as f32,
        },
    ];
    let fuel = FuelInfo {
        fuel_percent: (telemetry.telemetry.telemetry_info[cur_driver_id].m_fuel
            / telemetry.telemetry.telemetry_info[cur_driver_id].m_fuel_capacity)
            as f32,
        fuel_liters: telemetry.telemetry.telemetry_info[cur_driver_id].m_fuel as f32,
        virt_eng_percent: telemetry.telemetry.telemetry_info[cur_driver_id].m_virtual_energy,
    };

    DynDriverInfo { tires, fuel }
}
