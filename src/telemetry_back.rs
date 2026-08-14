#[cfg(not(target_os = "windows"))]
use memmap2::Mmap;
#[cfg(not(target_os = "windows"))]
use std::fs::File;
#[cfg(target_os = "windows")]
use windows_sys::Win32::Foundation::CloseHandle;
#[cfg(target_os = "windows")]
use windows_sys::Win32::System::Memory::{
    FILE_MAP_READ, MapViewOfFile, OpenFileMappingW, UnmapViewOfFile,
};

//##################################################################################################
//#                                                                                                #
//#                            Struct and Enums definitions for the Shm                            #
//#                                                                                                #
//##################################################################################################
const MAX_PATH: usize = 260;

pub struct Telemetry {
    full_mode: bool,
    mmap: Mmap,
}

impl Telemetry {
    pub fn new(path: &str) -> Telemetry {
        Self {
            full_mode: false,
            mmap: Telemetry::get_mmap(path),
        }
    }

    fn mmap_fallback() -> Mmap {
        let file = tempfile::NamedTempFile::new().unwrap().into_file();
        file.set_len(1).unwrap();
        unsafe { Mmap::map(&file).unwrap() }
    }

    #[cfg(not(target_os = "windows"))]
    pub fn get_mmap(path: &str) -> Mmap {
        const EXPECTED_LEN: u64 = std::mem::size_of::<SharedMemoryObjectOut>() as u64;
        const MAX_RETRIES: u32 = 20;
        const RETRY_DELAY: std::time::Duration = std::time::Duration::from_millis(100);

        for attempt in 0..MAX_RETRIES {
            let file = match File::open(path) {
                Ok(v) => v,
                Err(e) => {
                    if attempt == 0 {
                        println!("Could not open telemetry file: {e}");
                    }
                    std::thread::sleep(RETRY_DELAY);
                    continue;
                }
            };

            let len = match file.metadata() {
                Ok(m) => m.len(),
                Err(e) => {
                    println!("Could not stat telemetry file: {e}");
                    std::thread::sleep(RETRY_DELAY);
                    continue;
                }
            };

            if len < EXPECTED_LEN {
                // writer hasn't pre-sized/written the file yet — don't mmap a short file
                std::thread::sleep(RETRY_DELAY);
                continue;
            }

            match unsafe { Mmap::map(&file) } {
                Ok(mmap) => {
                    return mmap;
                }
                Err(e) => {
                    println!("Could not mmap telemetry file: {e}");
                    std::thread::sleep(RETRY_DELAY);
                    continue;
                }
            }
        }

        println!("Telemetry file never became ready after {MAX_RETRIES} attempts");
        println!("Switching into no telemetry mode");
        Telemetry::mmap_fallback()
    }

    #[cfg(target_os = "windows")]
    pub fn get_mmap(_path: &str, state: &Mutex<TelemetryState>) -> Mmap {
        // Windows telemetry is read from the named mapping object directly in update_telemetry.
        state.lock().unwrap().full_mode = true;
        mmap_fallback()
    }

    #[cfg(not(target_os = "windows"))]
    pub fn update_telemetry(&self, mmap: &Mmap) -> Option<Box<SharedMemoryObjectOut>> {
        let layout_size = std::mem::size_of::<SharedMemoryLayout>();

        if mmap.len() < layout_size {
            return None;
        }

        unsafe {
            // 1. Allocate space for the full layout directly on the HEAP
            // This prevents the 300KB from ever touching the stack
            let mut layout_ptr = Box::<SharedMemoryLayout>::new_uninit();

            // 2. Copy directly from memory map to our heap pointer
            std::ptr::copy_nonoverlapping(
                mmap.as_ptr(),
                layout_ptr.as_mut_ptr() as *mut u8,
                layout_size,
            );

            // 3. Convert to initialized and return just the .data field
            // We box the result so the 300KB stays on the heap
            let full_layout = layout_ptr.assume_init();
            Some(Box::new(full_layout.data))
        }
    }

    #[cfg(target_os = "windows")]
    pub fn update_telemetry(&self, _mmap: &Mmap) -> Option<Box<SharedMemoryObjectOut>> {
        const LMU_SHARED_MEMORY_FILE: &str = "LMU_Data";

        let layout_size = std::mem::size_of::<SharedMemoryLayout>();
        let name_wide: Vec<u16> = LMU_SHARED_MEMORY_FILE
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();

        unsafe {
            let mapping_handle = OpenFileMappingW(FILE_MAP_READ, 0, name_wide.as_ptr());
            if mapping_handle.is_null() {
                return None;
            }

            let mapped_view = MapViewOfFile(mapping_handle, FILE_MAP_READ, 0, 0, layout_size);
            if mapped_view.Value.is_null() {
                CloseHandle(mapping_handle);
                return None;
            }

            let mut layout_ptr = Box::<SharedMemoryLayout>::new_uninit();
            std::ptr::copy_nonoverlapping(
                mapped_view.Value as *const u8,
                layout_ptr.as_mut_ptr() as *mut u8,
                layout_size,
            );

            UnmapViewOfFile(mapped_view);
            CloseHandle(mapping_handle);

            let full_layout = layout_ptr.assume_init();
            Some(Box::new(full_layout.data))
        }
    }
}

pub fn i8_array32_to_string(buf: &[i8; 32]) -> String {
    let bytes: Vec<u8> = buf
        .iter()
        .take_while(|&&b| b != 0)
        .map(|b| *b as u8)
        .collect();

    String::from_utf8_lossy(&bytes).to_string()
}
pub fn i8_array64_to_string(buf: &[i8; 64]) -> String {
    let bytes: Vec<u8> = buf
        .iter()
        .take_while(|&&b| b != 0)
        .map(|b| *b as u8)
        .collect();

    String::from_utf8_lossy(&bytes).to_string()
}

#[repr(u32)]
#[allow(dead_code)]
pub enum SharedMemoryEvent {
    Enter = 0,
    Exit,
    Startup,
    Shutdown,
    Load,
    Unload,
    StartSession,
    EndSession,
    EnterRealtime,
    ExitRealtime,
    UpdateScoring,
    UpdateTelemetry,
    InitApplication,
    UninitApplication,
    SetEnvironment,
    Ffb,
    Max,
}

#[repr(C, packed(4))]
pub struct SharedMemoryLayout {
    pub data: SharedMemoryObjectOut,
}

#[repr(C, packed(4))]
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct SharedMemoryObjectOut {
    pub generic: SharedMemoryGeneric,
    pub paths: SharedMemoryPathData,
    pub scoring: SharedMemoryScoringData,
    pub telemetry: SharedMemoryTelemtryData,
}

#[repr(C, packed(4))]
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct SharedMemoryGeneric {
    // In the C++ API this is declared as `SharedMemoryEvent events[SME_MAX]`,
    // but it is used like a boolean flag array (0/1). Reading raw bytes into
    // a Rust enum is UB if the discriminant isn't valid, so represent as u32.
    pub events: [u32; SharedMemoryEvent::Max as usize],
    pub game_version: i32,
    pub ffb_torque: f32,
    pub app_info: ApplicationStateV01,
}

#[repr(C, packed(4))]
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct SharedMemoryPathData {
    pub user_data: [i8; MAX_PATH],
    pub custom_variables: [i8; MAX_PATH],
    pub steward_results: [i8; MAX_PATH],
    pub player_profile: [i8; MAX_PATH],
    pub plugins_folder: [i8; MAX_PATH],
}

#[repr(C, packed(4))]
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct SharedMemoryTelemtryData {
    // Remember to check CopySharedMemoryObj still works properly when updating this struct
    pub active_vehicles: u8,
    pub player_vehicle_idx: u8,
    pub player_has_vehicle: bool,
    pub telemetry_info: [TelemInfoV01; 104],
}

#[repr(C, packed(4))]
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct SharedMemoryScoringData {
    // Remember to check CopySharedMemoryObj still works properly when updating this struct
    pub scoring_info: ScoringInfoV01,
    // Matches the working Python/ctypes mapping: this covers the (padding + size_t)
    // region used by the producer. Keeping it as raw bytes avoids size/alignment
    // mismatches across compilers/ABIs.
    pub scoring_stream_size: [u8; 12],
    pub veh_scoring_info: [VehicleScoringInfoV01; 104], // MUST NOT BE MOVED!
    pub scoring_stream: [i8; 65536],
}

#[repr(C, packed(4))]
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct ApplicationStateV01 {
    pub m_app_window: u64,        // application window handle
    pub m_width: u32,             // screen width
    pub m_height: u32,            // screen height
    pub m_refresh_rate: u32,      // refresh rate
    pub m_windowed: u32,          // really just a boolean whether we are in windowed mode
    pub m_options_location: u8,   // 0=main UI, 1=track loading, 2=monitor, 3=on track
    pub m_options_page: [i8; 31], // the name of the options page
    pub m_expansion: [u8; 204],   // future use
}

#[repr(C, packed(4))] // Crucial for matching C++ memory layout (3 doubles in a row)
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct TelemVect3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[repr(C, packed(4))]
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct TelemInfoV01 {
    // Time
    pub m_id: i32,         // long (8 bytes)
    pub m_delta_time: f64, // double
    pub m_elapsed_time: f64,
    pub m_lap_number: i32,
    pub m_lap_start_et: f64,
    pub m_vehicle_name: [i8; 64], // char[64]
    pub m_track_name: [i8; 64],   // char[64]

    // Position and derivatives
    pub m_pos: TelemVect3,
    pub m_local_vel: TelemVect3,
    pub m_local_accel: TelemVect3,

    // Orientation and derivatives
    pub m_ori: [TelemVect3; 3], // TelemVect3 mOri[3]
    pub m_local_rot: TelemVect3,
    pub m_local_rot_accel: TelemVect3,

    // Vehicle status
    pub m_gear: i32,
    pub m_engine_rpm: f64,
    pub m_engine_water_temp: f64,
    pub m_engine_oil_temp: f64,
    pub m_clutch_rpm: f64,

    // Driver input
    pub m_unfiltered_throttle: f64,
    pub m_unfiltered_brake: f64,
    pub m_unfiltered_steering: f64,
    pub m_unfiltered_clutch: f64,

    // Filtered input
    pub m_filtered_throttle: f64,
    pub m_filtered_brake: f64,
    pub m_filtered_steering: f64,
    pub m_filtered_clutch: f64,

    // Misc
    pub m_steering_shaft_torque: f64,
    pub m_front_3rd_deflection: f64,
    pub m_rear_3rd_deflection: f64,

    // Aerodynamics
    pub m_front_wing_height: f64,
    pub m_front_ride_height: f64,
    pub m_rear_ride_height: f64,
    pub m_drag: f64,
    pub m_front_downforce: f64,
    pub m_rear_downforce: f64,

    // State/damage info
    pub m_fuel: f64,
    pub m_engine_max_rpm: f64,
    pub m_scheduled_stops: u8, // unsigned char
    pub m_overheating: bool,   // bool in C++ is usually 1 byte, matching Rust bool
    pub m_detached: bool,
    pub m_headlights: bool,
    pub m_dent_severity: [u8; 8],
    pub m_last_impact_et: f64,
    pub m_last_impact_magnitude: f64,
    pub m_last_impact_pos: TelemVect3,

    // Expanded
    pub m_engine_torque: f64,
    pub m_current_sector: i32,
    pub m_speed_limiter: u8,
    pub m_max_gears: u8,
    pub m_front_tire_compound_index: u8,
    pub m_rear_tire_compound_index: u8,
    pub m_fuel_capacity: f64,
    pub m_front_flap_activated: u8,
    pub m_rear_flap_activated: u8,
    pub m_rear_flap_legal_status: u8,
    pub m_ignition_starter: u8,

    pub m_front_tire_compound_name: [u8; 18],
    pub m_rear_tire_compound_name: [u8; 18],

    pub m_speed_limiter_available: u8,
    pub m_anti_stall_activated: u8,
    pub m_unused: [u8; 2],
    pub m_visual_steering_wheel_range: f32, // float

    pub m_rear_brake_bias: f64,
    pub m_turbo_boost_pressure: f64,
    pub m_physics_to_graphics_offset: [f32; 3],
    pub m_physical_steering_wheel_range: f32,

    // deltabest
    pub m_delta_best: f64,

    pub m_battery_charge_fraction: f64,

    // electric boost motor
    pub m_electric_boost_motor_torque: f64,
    pub m_electric_boost_motor_rpm: f64,
    pub m_electric_boost_motor_temperature: f64,
    pub m_electric_boost_water_temperature: f64,
    pub m_electric_boost_motor_state: u8,

    pub m_lap_invalidated: bool,
    pub m_abs_active: bool,
    pub m_tc_active: bool,
    pub m_speed_limiter_active: bool,
    pub m_wiper_state: u8,
    pub m_tc: u8,
    pub m_tc_max: u8,
    pub m_tc_slip: u8,
    pub m_tc_slip_max: u8,
    pub m_tc_cut: u8,
    pub m_tc_cut_max: u8,
    pub m_abs: u8,
    pub m_abs_max: u8,
    pub m_motor_map: u8,
    pub m_motor_map_max: u8,
    pub m_migration: u8,
    pub m_migration_max: u8,
    pub m_front_anti_sway: u8,
    pub m_front_anti_sway_max: u8,
    pub m_rear_anti_sway: u8,
    pub m_rear_anti_sway_max: u8,
    pub m_lift_and_coast_progress: u8,
    pub m_track_limits_steps: u8, // Normalized track limits points (TrackLimitPoints * TrackLimitStepsPerPoint)
    pub m_regen: f32,             // kW
    pub m_so_cl: f32,
    pub m_virtual_energy: f32,
    pub m_time_gap_car_ahead: f32,
    pub m_time_gap_car_behind: f32,
    pub m_time_gap_place_ahead: f32,
    pub m_time_gap_place_behind: f32,
    pub m_vehicle_model: [u8; 30],
    pub m_vehicle_class: IPVehicleClass,
    pub m_vehicle_championship: IPVehicleChampionship,

    pub m_expansion: [u8; 20],

    // Wheel info
    pub m_wheel: [TelemWheelV01; 4],
}

#[repr(C, packed(4))]
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct TelemWheelV01 {
    pub m_suspension_deflection: f64,
    pub m_ride_height: f64,
    pub m_susp_force: f64,
    pub m_brake_temp: f64,
    pub m_brake_pressure: f64,

    pub m_rotation: f64,
    pub m_lateral_patch_vel: f64,
    pub m_longitudinal_patch_vel: f64,
    pub m_lateral_ground_vel: f64,
    pub m_longitudinal_ground_vel: f64,
    pub m_camber: f64,
    pub m_lateral_force: f64,
    pub m_longitudinal_force: f64,
    pub m_tire_load: f64,

    pub m_grip_fract: f64,
    pub m_pressure: f64,
    pub m_temperature: [f64; 3],
    pub m_wear: f64,
    pub m_terrain_name: [i8; 16],
    pub m_surface_type: u8,
    pub m_flat: bool,
    pub m_detached: bool,
    pub m_static_undeflected_radius: u8,

    pub m_vertical_tire_deflection: f64,
    pub m_wheel_y_location: f64,
    pub m_toe: f64,

    pub m_tire_carcass_temperature: f64,
    pub m_tire_inner_layer_temperature: [f64; 3],

    pub m_expansion: [u8; 24],
}

#[repr(C, packed(4))]
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct VehicleScoringInfoV01 {
    pub m_id: i32,               // long -> i64
    pub m_driver_name: [i8; 32], // char -> i8
    pub m_vehicle_name: [i8; 64],
    pub m_total_laps: i16, // short -> i16
    pub m_sector: i8,      // signed char -> i8
    pub m_finish_status: i8,
    pub m_lap_dist: f64, // double -> f64
    pub m_path_lateral: f64,
    pub m_track_edge: f64,

    pub m_best_sector1: f64,
    pub m_best_sector2: f64,
    pub m_best_lap_time: f64,
    pub m_last_sector1: f64,
    pub m_last_sector2: f64,
    pub m_last_lap_time: f64,
    pub m_cur_sector1: f64,
    pub m_cur_sector2: f64,

    pub m_num_pitstops: i16,
    pub m_num_penalties: i16,
    pub m_is_player: bool, // bool -> 1 byte

    pub m_control: i8,
    pub m_in_pits: bool,
    pub m_place: u8, // unsigned char -> u8
    pub m_vehicle_class: [i8; 32],

    // Dash Indicators
    pub m_time_behind_next: f64,
    pub m_laps_behind_next: i32,
    pub m_time_behind_leader: f64,
    pub m_laps_behind_leader: i32,
    pub m_lap_start_et: f64,

    // Position and derivatives
    pub m_pos: TelemVect3,
    pub m_local_vel: TelemVect3,
    pub m_local_accel: TelemVect3,

    // Orientation and derivatives
    pub m_ori: [TelemVect3; 3],
    pub m_local_rot: TelemVect3,
    pub m_local_rot_accel: TelemVect3,

    pub m_headlights: u8,
    pub m_pit_state: u8,
    pub m_server_scored: u8,
    pub m_individual_phase: u8,

    pub m_qualification: i32,

    pub m_time_into_lap: f64,
    pub m_estimated_lap_time: f64,

    pub m_pit_group: [i8; 24],
    pub m_flag: u8,
    pub m_under_yellow: bool,
    pub m_count_lap_flag: u8,
    pub m_in_garage_stall: bool,

    pub m_upgrade_pack: [u8; 16],
    pub m_pit_lap_dist: f32, // float -> f32

    pub m_best_lap_sector1: f32,
    pub m_best_lap_sector2: f32,

    pub m_steam_id: u64, // unsigned long long -> u64

    pub m_veh_filename: [i8; 32],

    pub m_attack_mode: i16,

    pub m_fuel_fraction: u8,

    pub m_drs_state: bool,

    pub m_expansion: [u8; 4],
}

#[repr(C, packed(4))]
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct ScoringInfoV01 {
    pub m_track_name: [i8; 64], // char[64] -> [i8; 64]
    pub m_session: i32,         // long -> i64
    pub m_current_et: f64,      // double -> f64
    pub m_end_et: f64,
    pub m_max_laps: i32,
    pub m_lap_dist: f64,
    // Pointer value from the producer process; not valid in our address space.
    pub m_results_stream: [u8; 8],

    pub m_num_vehicles: i32,

    pub m_game_phase: u8,        // unsigned char -> u8
    pub m_yellow_flag_state: i8, // signed char -> i8
    pub m_sector_flag: [i8; 3],  // signed char[3] -> [i8; 3]
    pub m_start_light: u8,
    pub m_num_red_lights: u8,
    pub m_in_realtime: bool, // bool -> 1 byte
    pub m_player_name: [i8; 32],
    pub m_plr_file_name: [i8; 64],

    // weather
    pub m_dark_cloud: f64,
    pub m_raining: f64,
    pub m_ambient_temp: f64,
    pub m_track_temp: f64,
    pub m_wind: TelemVect3, // Your previously implemented struct
    pub m_min_path_wetness: f64,
    pub m_max_path_wetness: f64,

    // multiplayer
    pub m_game_mode: u8,
    pub m_is_password_protected: bool,
    pub m_server_port: u16,      // unsigned short -> u16
    pub m_server_public_ip: u32, // unsigned long -> u32 (C++ 'long' is 4 bytes on Windows)
    pub m_max_players: i32,      // long -> i64
    pub m_server_name: [i8; 32],
    pub m_start_et: f32, // float -> f32

    pub m_avg_path_wetness: f64,

    // Future use
    pub m_expansion: [u8; 200],

    // Array of vehicle scoring info
    // Pointer value from the producer process; not valid in our address space.
    pub m_vehicle: [u8; 8],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn sizes_match_python_mapping() {
        // Values printed by `python3 -c 'import ctypes; ... ctypes.sizeof(...)'`
        // in `pyLMUSharedMemory/lmu_data.py`.
        assert_eq!(size_of::<SharedMemoryGeneric>(), 332);
        assert_eq!(size_of::<ScoringInfoV01>(), 548);
        assert_eq!(size_of::<SharedMemoryScoringData>(), 126_832);
        assert_eq!(size_of::<SharedMemoryTelemtryData>(), 196_356);
        assert_eq!(size_of::<SharedMemoryObjectOut>(), 324_820);
        assert_eq!(size_of::<SharedMemoryLayout>(), 324_820);
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IPVehicleClass {
    Hypercar = 0x00,
    Lmp2Elms = 0x02,
    Lmp2 = 0x03,
    Lmp3 = 0x04,
    Gte = 0x05,
    Gt3 = 0x06,
    PaceCar = 0x08,
    Unknown = 0xFF,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IPVehicleChampionship {
    Wec2023 = 0x00,
    Wec2024 = 0x01,
    Wec2025 = 0x02,
    Wec2026 = 0x03,

    Elms2025 = 0x10,
    Elms2026 = 0x11,

    Unknown = 0xFF,
}
