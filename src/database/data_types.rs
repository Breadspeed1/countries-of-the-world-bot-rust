use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use chrono::{DateTime, Utc};

pub struct Territory {
    territory_id: u64,
    owner_id: u64,
    color_hex: String,
    name: String
}

pub struct Country {
    territory_id: u64,
    country_id: u8,
    troops: u32,
    citizens: u32
}

pub struct User {
    user_id: u64,
    money: u64,
    distance_traveled: f32
}

pub struct Building {
    country_id: u8,
    building_id: u64,
    building_type: BuildingType
}

pub struct Conflict {
    attack_id: u64,
    aggressor_id: u64,
    victim_country_id: u8,
    aggressor_troops_engaged: u32,
    arrival_date: u128
}

pub enum BuildingType {
    OilRig,
    Settlement
}

impl BuildingType {
    pub fn from_string(s: String) -> Result<BuildingType, Box<dyn Error>> {
        match s.as_str() {
            "oil_rig" => Ok(BuildingType::OilRig),
            "settlement" => Ok(BuildingType::Settlement),
            _ => Err(Box::new(TypeDefinitionError))
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            BuildingType::OilRig => String::from("oil_rig"),
            BuildingType::Settlement => String::from("settlement")
        }
    }
}

pub struct Vehicle {
    territory_id: u64,
    vehicle_id: u64,
    vehicle_type: VehicleType
}

pub enum VehicleType {
    Car,
    Helicopter
}

impl VehicleType {
    pub fn from_string(s: String) -> Result<VehicleType, Box<dyn Error>> {
        match s.as_str() {
            "car" => Ok(VehicleType::Car),
            "helicopter" => Ok(VehicleType::Helicopter),
            _ => Err(Box::new(TypeDefinitionError))
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            VehicleType::Car => String::from("car"),
            VehicleType::Helicopter => String::from("helicopter")
        }
    }
}

struct TypeDefinitionError;

impl Debug for TypeDefinitionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!("failed to define type")
    }
}

impl Display for TypeDefinitionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!("failed to define type")
    }
}

impl Error for TypeDefinitionError {}