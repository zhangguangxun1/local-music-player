use log::{error, info};

// 检查实例是否唯一
pub fn check_is_single_instance(ins_name: &str) -> bool {
    match single_instance::SingleInstance::new(ins_name) {
        Ok(ins) => {
            info!("Starting single instance");
            if !ins.is_single() {
                error!("The single instance is not present.");
                return false;
            }
            true
        }
        Err(e) => {
            error!("{}", e);
            false
        }
    }
}
