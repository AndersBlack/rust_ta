use std::{fs::File, fs, io::{BufWriter, Write}};
use actix_web::{get, post, delete, HttpResponse, web, Responder};
use serde_json::Value;
use crate::car::{Car, NewCarData};

/// Returns a list of all cars
#[get("/cars")]
async fn get_all_cars() -> impl Responder {
  let car_data = read_json_file().await;

  HttpResponse::Ok().json(car_data)
}

/// Returns details about the car with the requested vin
#[get("/cars/{vin}")]
async fn get_specific_car(vin: web::Path<String>) -> impl Responder {
  let car_data = read_json_file().await;
  let vec_car_data: Vec<Car> = serde_json::from_value(car_data).expect("Car list was ill formatted");
  let mut found_car: Option<Car> = None;

  for car in vec_car_data {
      if car.vin == vin.to_string() {
        found_car = Some(car)
      }
  }
  
  if found_car.is_some() {
    HttpResponse::Ok().json(found_car.unwrap())
  } else {
    HttpResponse::NotFound().body("Unable to locate vin")
  }
}

/// Deletes the car with the requested vin
#[delete("/cars/{vin}")]
async fn delete_specific_car(vin: web::Path<String>) -> impl Responder {
  let car_data = read_json_file().await;
  let mut vec_car_data: Vec<Car> = serde_json::from_value(car_data).expect("Car list was ill formatted");
  let mut index: Option<usize> = None;

  for i in  0..vec_car_data.len() {
    if vec_car_data[i].vin == vin.to_string() {
      index = Some(i);
    }
  }

  if index.is_some() {
    vec_car_data.remove(index.unwrap());
    let data_file = get_file_data().await;

    let mut writer = BufWriter::new(data_file);
    serde_json::to_writer(&mut writer, &vec_car_data).expect("couldnt write to file");
    writer.flush().expect("couldnt flush file");

    let response = "Deleted car";
    HttpResponse::Ok().body(response)
  } else {
    HttpResponse::NotFound().body("Unable to locate vin")
  }
}

/// Post new car
#[post("/cars/submit")]
async fn add_car(new_car_data: web::Json<NewCarData>) -> impl Responder {
  let car_data = read_json_file().await;
  let mut vec_car_data: Vec<Car> = serde_json::from_value(car_data).expect("Car list was ill formatted");
  let car_struct = Car::new(new_car_data.make.clone(), new_car_data.model.clone(), new_car_data.mileage.clone());
  vec_car_data.push(car_struct);

  let data_file = get_file_data().await;
  let mut writer = BufWriter::new(data_file);
  serde_json::to_writer(&mut writer, &vec_car_data).expect("couldnt write to file");
  writer.flush().expect("couldnt flush file");

  let response = "Added new car";
  HttpResponse::Ok().body(response)
}

/// Update car
#[post("/cars/{vin}")]
async fn update_car(vin: web::Path<String>, update_data: web::Json<NewCarData>) -> impl Responder {
  let car_data = read_json_file().await;
  let mut vec_car_data: Vec<Car> = serde_json::from_value(car_data).expect("Car list was ill formatted");

  let mut found_car = false;

  for car in &mut vec_car_data {
    if car.vin == vin.to_string() {
      found_car = true;
      car.make = update_data.make.clone();
      car.model = update_data.model.clone();
      car.mileage = update_data.mileage.clone();
    }
  }

  if found_car {
    let data_file = get_file_data().await;
    let mut writer = BufWriter::new(data_file);
    serde_json::to_writer(&mut writer, &vec_car_data).expect("couldnt write to file");
    writer.flush().expect("couldnt flush file");

    let response = "Updated car";
    HttpResponse::Ok().body(response)
  } else {
    HttpResponse::NotFound().body("Unable to locate vin")
  }
}

async fn read_json_file() -> Value {
  let car_data = fs::read_to_string("src/cars.json").expect("Unable to read file");
  let json: serde_json::Value = serde_json::from_str(&car_data).expect("Json was ill formatted");

  json
}

async fn get_file_data() -> File {
  let data_res = File::create("src/cars.json");
  let data_file = match data_res {
      Ok(file) => file,
      Err(error) => panic!("Problem opening the data file: {:?}", error),
  };

  data_file
}