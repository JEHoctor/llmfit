use llmfit_core::hardware::SystemSpecs;
use pyo3::prelude::*;

#[derive(Clone)]
#[pyclass(frozen, skip_from_py_object)]
pub struct GpuInfo {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub vram_gb: Option<f64>,
    #[pyo3(get)]
    pub backend: String,
    #[pyo3(get)]
    pub count: u32,
    #[pyo3(get)]
    pub unified_memory: bool,
}

#[pymethods]
impl GpuInfo {
    fn __repr__(&self) -> String {
        format!(
            "GpuInfo(name={:?}, backend={:?}, vram_gb={:?}, count={})",
            self.name, self.backend, self.vram_gb, self.count
        )
    }
}

#[pyclass(frozen)]
pub struct SystemInfo {
    #[pyo3(get)]
    pub total_ram_gb: f64,
    #[pyo3(get)]
    pub available_ram_gb: f64,
    #[pyo3(get)]
    pub cpu_cores: usize,
    #[pyo3(get)]
    pub cpu_name: String,
    #[pyo3(get)]
    pub has_gpu: bool,
    #[pyo3(get)]
    pub gpu_vram_gb: Option<f64>,
    #[pyo3(get)]
    pub gpu_name: Option<String>,
    #[pyo3(get)]
    pub gpu_count: u32,
    #[pyo3(get)]
    pub unified_memory: bool,
    #[pyo3(get)]
    pub backend: String,
    #[pyo3(get)]
    pub gpus: Vec<GpuInfo>,
}

#[pymethods]
impl SystemInfo {
    fn __repr__(&self) -> String {
        format!(
            "SystemInfo(cpu_name={:?}, total_ram_gb={}, cpu_cores={}, has_gpu={})",
            self.cpu_name, self.total_ram_gb, self.cpu_cores, self.has_gpu
        )
    }
}

#[pyfunction]
pub fn detect_system() -> PyResult<SystemInfo> {
    let specs = SystemSpecs::detect();
    let gpus = specs
        .gpus
        .into_iter()
        .map(|g| GpuInfo {
            name: g.name,
            vram_gb: g.vram_gb,
            backend: g.backend.label().to_string(),
            count: g.count,
            unified_memory: g.unified_memory,
        })
        .collect();
    Ok(SystemInfo {
        total_ram_gb: specs.total_ram_gb,
        available_ram_gb: specs.available_ram_gb,
        cpu_cores: specs.total_cpu_cores,
        cpu_name: specs.cpu_name,
        has_gpu: specs.has_gpu,
        gpu_vram_gb: specs.gpu_vram_gb,
        gpu_name: specs.gpu_name,
        gpu_count: specs.gpu_count,
        unified_memory: specs.unified_memory,
        backend: specs.backend.label().to_string(),
        gpus,
    })
}

#[pymodule]
fn llmfit_pyo3(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SystemInfo>()?;
    m.add_class::<GpuInfo>()?;
    m.add_function(wrap_pyfunction!(detect_system, m)?)?;
    Ok(())
}
