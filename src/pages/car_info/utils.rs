use crate::{
    pages::car_info::car_info_main::{CarInfo, FuelInfo, Inputs, TireInfo},
    providers::telemetry_provider::interface::{
        IPVehicleClass, i8_array32_to_string, i8_array64_to_string,
    },
};

pub(super) struct StaleDriverInfo {
    pub(super) name: String,
    pub(super) car: String,
    pub(super) car_class: IPVehicleClass,
    pub(super) index: usize,
}

pub(super) struct DynDriverInfo {
    pub(super) tires: [TireInfo; 4],
    pub(super) fuel: FuelInfo,
    pub(super) inputs: Inputs,
}

impl CarInfo {
    pub(super) fn get_stale_driver_info(&self) -> Result<StaleDriverInfo, String> {
        let telemetry = *self
            .telemetry_provider
            .as_ref()
            .as_ref()
            .unwrap()
            .get_telemetry_object();

        let mut drivers: Vec<(String, usize)> = Vec::new();

        for (i, car) in telemetry.scoring.veh_scoring_info.iter().enumerate() {
            let name = i8_array32_to_string(&car.m_driver_name);
            if !name.is_empty() {
                drivers.push((name, i));
            }
        }
        let cur_driver_id;

        if let Some(driver) = drivers
            .iter()
            .find(|(name, _)| name == &*self.settings_provider.in_game_name.read().unwrap())
        {
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

    pub(super) fn get_dyn_driver_info(&self) -> DynDriverInfo {
        let telemetry = self
            .telemetry_provider
            .as_ref()
            .as_ref()
            .unwrap()
            .get_telemetry_object();

        let wheels = &telemetry.telemetry.telemetry_info[self.driver_index].m_wheel;
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
            fuel_percent: (telemetry.telemetry.telemetry_info[self.driver_index].m_fuel
                / telemetry.telemetry.telemetry_info[self.driver_index].m_fuel_capacity)
                as f32,
            fuel_liters: telemetry.telemetry.telemetry_info[self.driver_index].m_fuel as f32,
            virt_eng_percent: telemetry.telemetry.telemetry_info[self.driver_index]
                .m_virtual_energy,
        };

        let inputs = Inputs {
            throttle: telemetry.telemetry.telemetry_info[self.driver_index].m_unfiltered_throttle
                as f32,
            brake: telemetry.telemetry.telemetry_info[self.driver_index].m_unfiltered_brake as f32,
            steering: telemetry.telemetry.telemetry_info[self.driver_index].m_unfiltered_steering
                as f32,
        };

        DynDriverInfo {
            tires,
            fuel,
            inputs,
        }
    }

    pub(super) fn update(&mut self) {
        if self.name.is_empty()
            && let Ok(info) = self.get_stale_driver_info()
        {
            self.name = info.name;
            self.car = info.car;
            self.car_class = info.car_class;
            self.driver_index = info.index;
        }

        if !self.name.is_empty() {
            let dyn_driver_info = self.get_dyn_driver_info();
            self.tires = dyn_driver_info.tires;
            self.fuel_info = dyn_driver_info.fuel;
            self.inputs = dyn_driver_info.inputs;
        }
    }
}
