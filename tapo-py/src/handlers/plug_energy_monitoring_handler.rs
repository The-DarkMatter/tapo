use std::sync::Arc;

use chrono::NaiveDate;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use tapo::requests::EnergyDataInterval;
use tapo::responses::{
    CurrentPowerResult, DeviceInfoPlugEnergyMonitoringResult, DeviceUsageEnergyMonitoringResult,
    EnergyDataResult, EnergyUsageResult,
};
use tapo::PlugEnergyMonitoringHandler;
use tokio::sync::Mutex;

use crate::call_handler_method;
use crate::errors::ErrorWrapper;

#[derive(Clone, PartialEq)]
#[pyclass(name = "EnergyDataInterval", eq, eq_int)]
pub enum PyEnergyDataInterval {
    Hourly,
    Daily,
    Monthly,
}

#[derive(Clone)]
#[pyclass(name = "PlugEnergyMonitoringHandler")]
pub struct PyPlugEnergyMonitoringHandler {
    handler: Arc<Mutex<PlugEnergyMonitoringHandler>>,
}

impl PyPlugEnergyMonitoringHandler {
    pub fn new(handler: PlugEnergyMonitoringHandler) -> Self {
        Self {
            handler: Arc::new(Mutex::new(handler)),
        }
    }
}

#[pymethods]
impl PyPlugEnergyMonitoringHandler {
    pub async fn refresh_session(&self) -> PyResult<()> {
        call_handler_method!(
            self,
            PlugEnergyMonitoringHandler::refresh_session,
            discard_result
        )
    }

    pub async fn on(&self) -> PyResult<()> {
        call_handler_method!(self, PlugEnergyMonitoringHandler::on)
    }

    pub async fn off(&self) -> PyResult<()> {
        call_handler_method!(self, PlugEnergyMonitoringHandler::off)
    }

    pub async fn device_reset(&self) -> PyResult<()> {
        call_handler_method!(self, PlugEnergyMonitoringHandler::device_reset)
    }

    pub async fn get_device_info(&self) -> PyResult<DeviceInfoPlugEnergyMonitoringResult> {
        call_handler_method!(self, PlugEnergyMonitoringHandler::get_device_info)
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let result = call_handler_method!(self, PlugEnergyMonitoringHandler::get_device_info_json)?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_device_usage(&self) -> PyResult<DeviceUsageEnergyMonitoringResult> {
        call_handler_method!(self, PlugEnergyMonitoringHandler::get_device_usage)
    }

    pub async fn get_current_power(&self) -> PyResult<CurrentPowerResult> {
        call_handler_method!(self, PlugEnergyMonitoringHandler::get_current_power)
    }

    pub async fn get_energy_usage(&self) -> PyResult<EnergyUsageResult> {
        call_handler_method!(self, PlugEnergyMonitoringHandler::get_energy_usage)
    }

    #[pyo3(signature = (interval, start_date, end_date=None))]
    pub async fn get_energy_data(
        &self,
        interval: PyEnergyDataInterval,
        start_date: NaiveDate,
        end_date: Option<NaiveDate>,
    ) -> PyResult<EnergyDataResult> {
        let interval = match interval {
            PyEnergyDataInterval::Hourly => EnergyDataInterval::Hourly {
                start_date,
                end_date: end_date.unwrap_or(start_date),
            },
            PyEnergyDataInterval::Daily => EnergyDataInterval::Daily { start_date },
            PyEnergyDataInterval::Monthly => EnergyDataInterval::Monthly { start_date },
        };

        let result =
            call_handler_method!(self, PlugEnergyMonitoringHandler::get_energy_data, interval)?;
        Ok(result)
    }
}
