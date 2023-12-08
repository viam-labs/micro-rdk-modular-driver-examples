use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use micro_rdk::DoCommand;

use micro_rdk::common::{
    board::BoardType,
    config::ConfigType,
    motor::{Motor, MotorSupportedProperties, MotorType},
    registry::{self, ComponentRegistry, Dependency, RegistryError},
    status::Status,
    stop::Stoppable,
};

/// This driver is for a water pump and optional led
#[derive(DoCommand)]
pub struct WaterPump {
    board_handle: BoardType,
    pin: i32,
    led: Option<i32>,
}

pub fn register_model(registry: &mut ComponentRegistry) -> anyhow::Result<(), RegistryError> {
    registry.register_motor("water_pump", &WaterPump::from_config)?;
    log::info!("water_pump motor registration ok");
    Ok(())

}

impl WaterPump {
    pub fn from_config(cfg: ConfigType, deps: Vec<Dependency>) -> anyhow::Result<MotorType> {
        let board_handle = registry::get_board_from_dependencies(deps)
            .expect("failed to get board from dependencies");
        let pin = cfg.get_attribute::<i32>("pin")?;
        let led = cfg.get_attribute::<i32>("led").ok();
        Ok(Arc::new(Mutex::new(Self {
            board_handle,
            pin,
            led,
        })))
    }
}

impl Motor for WaterPump {
    fn set_power(&mut self, pct: f64) -> anyhow::Result<()> {
        let pct = pct.clamp(-1.0, 1.0);
        if pct > 0.0 {
            // high
            self.board_handle
                .lock()
                .unwrap()
                .set_gpio_pin_level(self.pin, true)?;
            if let Some(pin) = self.led {
                self.board_handle
                    .lock()
                    .unwrap()
                    .set_gpio_pin_level(pin, true)?;
            }
        } else {
            // low
            self.board_handle
                .lock()
                .unwrap()
                .set_gpio_pin_level(self.pin, false)?;
            if let Some(pin) = self.led {
                self.board_handle
                    .lock()
                    .unwrap()
                    .set_gpio_pin_level(pin, false)?;
            }
        };
        Ok(())
    }
    fn get_position(&mut self) -> anyhow::Result<i32> {
        unimplemented!();
    }
    fn go_for(
        &mut self,
        _rpm: f64,
        _revolutions: f64,
    ) -> anyhow::Result<Option<std::time::Duration>> {
        unimplemented!();
    }

    fn get_properties(&mut self) -> MotorSupportedProperties {
        MotorSupportedProperties {
            position_reporting: false,
        }
    }
}

impl Stoppable for WaterPump {
    fn stop(&mut self) -> anyhow::Result<()> {
        self.set_power(0.0)
    }
}

impl Status for WaterPump {
    fn get_status(&self) -> anyhow::Result<Option<micro_rdk::google::protobuf::Struct>> {
        Ok(Some(micro_rdk::google::protobuf::Struct {
            fields: HashMap::new(),
        }))
    }
}
