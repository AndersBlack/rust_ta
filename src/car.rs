use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::{self, Uuid};

#[derive(Debug, Deserialize, Serialize)]
pub struct Car {
  pub vin: String,
  pub make: String,
  pub model: String,
  pub mileage: String,
  pub car_object_created: DateTime<Utc>
}

#[derive(Debug, Deserialize)]
pub struct NewCarData {
  pub make: String,
  pub model: String,
  pub mileage: String,
}

impl Car {
  pub fn new(make: String, model: String, milage: String) -> Self {
    Self { 
      vin: Uuid::new_v4().to_string(), 
      make: make, 
      model: model, 
      mileage: milage, 
      car_object_created: Utc::now()
    }
  }
}