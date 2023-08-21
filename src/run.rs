use anyhow::Result;
use std::{thread, time::Duration};
use esp_idf_sys::{
    esp, gpio_config, gpio_config_t, gpio_int_type_t_GPIO_INTR_DISABLE,
    gpio_mode_t_GPIO_MODE_OUTPUT, gpio_set_level,
};

pub fn run() -> Result<()> {
    const GPIO_NUM: i32 = 2;

    let io_conf = gpio_config_t {
        pin_bit_mask: 1 << GPIO_NUM,
        mode: gpio_mode_t_GPIO_MODE_OUTPUT,
        pull_up_en: false.into(),
        pull_down_en: false.into(),
        intr_type: gpio_int_type_t_GPIO_INTR_DISABLE,
    };

    unsafe {
        esp!(gpio_config(&io_conf))?;
    }

    let mut led = false;
    loop {
        unsafe {
            esp!(gpio_set_level(GPIO_NUM, led.into()))?;
        }
        led ^= true;
        thread::sleep(Duration::from_millis(100));
    }
}
