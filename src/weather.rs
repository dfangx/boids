use amethyst::{
    prelude::*,
    core::{
        SystemDesc,
        timing::Time,
    },
    derive::SystemDesc,
    ecs::prelude::{
        System,
        SystemData,
        Write,
        Read,
    },
};
use chrono::prelude::*;
use crate::resources::{
    Weather,
    WeatherType,
};

#[derive(SystemDesc, Default)]
pub struct WeatherSystem;
impl WeatherSystem {
    fn change_weather(&self) {

    }
}
impl<'s> System <'s> for WeatherSystem {
    type SystemData = (Write<'s, Weather>,
                       Read<'s, Time>);

    fn run(&mut self, (mut weather, time): Self::SystemData) {
        self.change_weather();
    }
}
