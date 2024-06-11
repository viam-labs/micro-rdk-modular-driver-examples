use micro_rdk::common::registry::{ComponentRegistry, RegistryError};

pub mod moisture_sensor;
pub mod water_pump;
pub mod free_heap_sensor;
pub mod wifi_rssi_sensor;

pub fn register_models(registry: &mut ComponentRegistry) -> Result<(), RegistryError> {
    moisture_sensor::register_models(registry)?;
    water_pump::register_models(registry)?;
    free_heap_sensor::register_models(registry)?;
    wifi_rssi_sensor::register_models(registry)?;
    Ok(())
}
