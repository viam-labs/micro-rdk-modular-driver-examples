use std::{
    cell::RefCell,
    collections::{HashMap},
    rc::Rc,
    sync::{Arc, Mutex},
};

use micro_rdk::common::{
    analog::AnalogReader,
    config::ConfigType,
    registry::{self, ComponentRegistry, Dependency, RegistryError},
    sensor::{
        GenericReadingsResult, Sensor, SensorResult, SensorT, SensorType, TypedReadingsResult,
    },
    status::Status,
};

pub struct MoistureSensor(Rc<RefCell<dyn AnalogReader<u16, Error = anyhow::Error>>>);

pub fn register_model(registry: &mut ComponentRegistry) -> anyhow::Result<(), RegistryError> {
    registry.register_sensor("moisture", &MoistureSensor::from_config)?;
    log::info!("moisture sensor registration ok");
    Ok(())
}

impl MoistureSensor {
    pub fn from_config(_cfg: ConfigType, deps: Vec<Dependency>) -> anyhow::Result<SensorType> {
        // get board
        let board = registry::get_board_from_dependencies(deps)
            .expect("failed to get board from dependencies");
        // get reader from the board (analog reader set in app.viam)
        let reader = board
            .lock()
            .unwrap()
            .get_analog_reader_by_name("moisture".to_string())?;
        Ok(Arc::new(Mutex::new(Self(reader))))
    }
}

impl Sensor for MoistureSensor {
    fn get_generic_readings(&self) -> anyhow::Result<GenericReadingsResult> {
        Ok(self
            .get_readings()?
            .into_iter()
            .map(|v| (v.0, SensorResult::<f64> { value: v.1 }.into()))
            .collect())
    }
}

impl SensorT<f64> for MoistureSensor {
    fn get_readings(&self) -> anyhow::Result<TypedReadingsResult<f64>> {
        let reading = self.0.borrow_mut().read()?;
        let mut x = HashMap::new();
        x.insert("millivolts".to_string(), reading as f64);
        Ok(x)
    }
}

impl Status for MoistureSensor {
    fn get_status(&self) -> anyhow::Result<Option<micro_rdk::google::protobuf::Struct>> {
        Ok(Some(micro_rdk::google::protobuf::Struct {
            fields: HashMap::new(),
        }))
    }
}
