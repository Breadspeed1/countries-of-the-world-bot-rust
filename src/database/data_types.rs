use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use chrono::{DateTime, Utc};

pub struct Territory {
    pub territory_id: u64,
    pub owner_id: u64,
    pub color: u32,
    pub name: String
}

pub struct Country {
    pub territory_id: u64,
    pub country_id: u8,
    pub troops: u32,
    pub citizens: u32
}

pub struct User {
    pub user_id: u64,
    pub money: u64,
    pub distance_traveled: f32
}

pub struct Building {
    pub country_id: u8,
    pub building_id: u64,
    pub building_type: BuildingType
}

pub struct Conflict {
    pub attack_id: u64,
    pub aggressor_id: u64,
    pub victim_country_id: u8,
    pub aggressor_troops_engaged: u32,
    pub arrival_date: DateTime<Utc>
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
    pub territory_id: u64,
    pub vehicle_id: u64,
    pub vehicle_type: VehicleType
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
        write!(f, "failed to define type")
    }
}

impl Display for TypeDefinitionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to define type")
    }
}

impl Error for TypeDefinitionError {}