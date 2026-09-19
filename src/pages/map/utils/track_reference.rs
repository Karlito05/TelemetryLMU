use crate::pages::map::map_main::Track;

pub fn set_track_reference(track_reference: &mut Option<Track>, track: &str) {
    match track {
        "Algrave International Circuit" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/Algarve International Circuit.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Autodromo Enzo e Dino Ferrari" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/Autodromo Enzo e Dino Ferrari.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Autodromo Nazionale Monza" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/Autodromo Nazionale Monza.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Autodrómo José Carlos Pace" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/Autódromo José Carlos Pace.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Bahrain Endurance Circuit" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/Bahrain Endurance Circuit.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Bahrain International Circuit" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/Bahrain International Circuit.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Bahrain Outer Circuit" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/Bahrain Outer Circuit.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Bahrain Paddock Circuit" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/Bahrain Paddock Circuit.json"
                ))
                .unwrap_or_default(),
            )
        }
        "COTA National Circuit" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/COTA National Circuit.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Circuit de Barcelona" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/Circuit de Barcelona.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Circuit de Spa-Francorchamps" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/Circuit de Spa-Francorchamps.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Circuit de la Sarthe Mulsanne" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/Circuit de la Sarthe Mulsanne.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Circuit de la Sarthe" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/Circuit de la Sarthe.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Circuit of the Americas" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/Circuit of the Americas.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Daytona International Speedway Road Course" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/Daytona International Speedway Road Course.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Fuji Speedway" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/Fuji Speedway.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Fuji Speedway Classic" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/Fuji Speedway Classic.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Lusail International Circuit" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/Lusail International Circuit.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Lusail Short Circuit" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/Lusail Short Circuit.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Monza Curva Grande Circuit" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/Monza Curva Grande Circuit.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Paul Ricard - 1A-V2-Short" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/Paul Ricard - 1A-V2-Short.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Paul Ricard - 1A-V2" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/Paul Ricard - 1A-V2.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Paul Ricard - 1A" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/Paul Ricard - 1A.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Paul Ricard - 3A" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/Paul Ricard - 3A.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Paul Ricard - ELMS" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/Paul Ricard - ELMS.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Sebring International Raceway" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/Sebring International Raceway.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Sebring School Circuit" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/Sebring School Circuit.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Silverstone Grand Prix Circuit - WEC" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/Silverstone Grand Prix Circuit - WEC.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Silverstone International Circuit" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/Silverstone International Circuit.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Silverstone National Circuit" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/Silverstone National Circuit.json"
                ))
                .unwrap_or_default(),
            )
        }
        "WeatherTech Raceway Laguna Seca" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../../../public/tracks/WeatherTech Raceway Laguna Seca.json"
                ))
                .unwrap_or_default(),
            )
        }
        &_ => println!("Unknown track"),
    }
}
