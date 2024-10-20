mod api_client;
mod errors;
mod handlers;
mod runtime;

use pyo3::prelude::*;

use api_client::PyApiClient;
use handlers::{
    PyColorLightHandler, PyColorLightSetDeviceInfoParams, PyEnergyDataInterval,
    PyGenericDeviceHandler, PyHubHandler, PyLightHandler, PyPlugEnergyMonitoringHandler,
    PyPlugHandler, PyPowerStripHandler, PyPowerStripPlugHandler, PyT100Handler, PyT110Handler,
    PyT300Handler, PyT31XHandler, TriggerLogsS200BResult, TriggerLogsT100Result,
    TriggerLogsT110Result, TriggerLogsT300Result,
};
use tapo::requests::Color;
use tapo::responses::{
    AutoOffStatus, ColorLightState, CurrentPowerResult, DefaultBrightnessState,
    DefaultColorLightState, DefaultLightState, DefaultPlugState, DefaultPowerType,
    DefaultStateType, DeviceInfoColorLightResult, DeviceInfoGenericResult, DeviceInfoHubResult,
    DeviceInfoLightResult, DeviceInfoPlugEnergyMonitoringResult, DeviceInfoPlugResult,
    DeviceInfoPowerStripResult, DeviceUsageEnergyMonitoringResult, DeviceUsageResult,
    EnergyDataResult, EnergyUsageResult, KE100Result, OvercurrentStatus, OverheatStatus, PlugState,
    PowerProtectionStatus, PowerStripPlugResult, S200BLog, S200BResult, S200BRotationParams,
    Status, T100Log, T100Result, T110Log, T110Result, T300Log, T300Result, T31XResult,
    TemperatureHumidityRecord, TemperatureHumidityRecords, TemperatureUnit, TemperatureUnitKE100,
    UsageByPeriodResult, WaterLeakStatus,
};

#[pymodule]
#[pyo3(name = "tapo")]
fn tapo_py(py: Python, module: &Bound<'_, PyModule>) -> PyResult<()> {
    let requests = PyModule::new_bound(py, "tapo.requests")?;
    let responses = PyModule::new_bound(py, "tapo.responses")?;

    register_handlers(module)?;
    register_requests(&requests)?;
    register_responses(&responses)?;
    register_responses_hub(&responses)?;
    register_responses_power_strip(&responses)?;

    module.add_submodule(&requests)?;
    module.add_submodule(&responses)?;

    let sys = py.import_bound("sys")?;
    let modules = sys.getattr("modules")?;
    modules.set_item("tapo.requests", requests)?;
    modules.set_item("tapo.responses", responses)?;

    Ok(())
}

fn register_requests(module: &Bound<'_, PyModule>) -> Result<(), PyErr> {
    module.add_class::<PyEnergyDataInterval>()?;
    module.add_class::<Color>()?;
    module.add_class::<PyColorLightSetDeviceInfoParams>()?;

    Ok(())
}

fn register_handlers(module: &Bound<'_, PyModule>) -> Result<(), PyErr> {
    module.add_class::<PyApiClient>()?;
    module.add_class::<PyColorLightHandler>()?;
    module.add_class::<PyGenericDeviceHandler>()?;
    module.add_class::<PyLightHandler>()?;
    module.add_class::<PyPlugEnergyMonitoringHandler>()?;
    module.add_class::<PyPlugHandler>()?;

    module.add_class::<PyHubHandler>()?;
    module.add_class::<PyT100Handler>()?;
    module.add_class::<PyT110Handler>()?;
    module.add_class::<PyT300Handler>()?;
    module.add_class::<PyT31XHandler>()?;

    module.add_class::<PyPowerStripHandler>()?;
    module.add_class::<PyPowerStripPlugHandler>()?;

    Ok(())
}

fn register_responses(module: &Bound<'_, PyModule>) -> Result<(), PyErr> {
    module.add_class::<CurrentPowerResult>()?;
    module.add_class::<DefaultBrightnessState>()?;
    module.add_class::<DefaultPowerType>()?;
    module.add_class::<DefaultStateType>()?;
    module.add_class::<DeviceUsageEnergyMonitoringResult>()?;
    module.add_class::<DeviceUsageResult>()?;
    module.add_class::<EnergyDataResult>()?;
    module.add_class::<EnergyUsageResult>()?;
    module.add_class::<OvercurrentStatus>()?;
    module.add_class::<OverheatStatus>()?;
    module.add_class::<PowerProtectionStatus>()?;
    module.add_class::<UsageByPeriodResult>()?;

    // device info: color light
    module.add_class::<DeviceInfoColorLightResult>()?;
    module.add_class::<DefaultColorLightState>()?;
    module.add_class::<ColorLightState>()?;

    // device info: generic
    module.add_class::<DeviceInfoGenericResult>()?;

    // device info: light
    module.add_class::<DeviceInfoLightResult>()?;
    module.add_class::<DefaultLightState>()?;

    // device info: plugs
    module.add_class::<DefaultPlugState>()?;
    module.add_class::<DeviceInfoPlugEnergyMonitoringResult>()?;
    module.add_class::<DeviceInfoPlugResult>()?;
    module.add_class::<PlugState>()?;

    Ok(())
}

fn register_responses_hub(module: &Bound<'_, PyModule>) -> Result<(), PyErr> {
    module.add_class::<DeviceInfoHubResult>()?;
    module.add_class::<KE100Result>()?;
    module.add_class::<S200BResult>()?;
    module.add_class::<T100Result>()?;
    module.add_class::<T110Result>()?;
    module.add_class::<T300Result>()?;
    module.add_class::<T31XResult>()?;

    // child devices
    module.add_class::<S200BLog>()?;
    module.add_class::<S200BRotationParams>()?;
    module.add_class::<Status>()?;
    module.add_class::<T100Log>()?;
    module.add_class::<T110Log>()?;
    module.add_class::<T300Log>()?;
    module.add_class::<TemperatureHumidityRecord>()?;
    module.add_class::<TemperatureHumidityRecords>()?;
    module.add_class::<TemperatureUnit>()?;
    module.add_class::<TemperatureUnitKE100>()?;
    module.add_class::<TriggerLogsS200BResult>()?;
    module.add_class::<TriggerLogsT100Result>()?;
    module.add_class::<TriggerLogsT110Result>()?;
    module.add_class::<TriggerLogsT300Result>()?;
    module.add_class::<WaterLeakStatus>()?;

    Ok(())
}

fn register_responses_power_strip(module: &Bound<'_, PyModule>) -> Result<(), PyErr> {
    module.add_class::<DeviceInfoPowerStripResult>()?;

    // child devices
    module.add_class::<AutoOffStatus>()?;
    module.add_class::<PowerStripPlugResult>()?;

    Ok(())
}
