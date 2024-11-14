{%- if test -%}
#![feature(custom_test_frameworks)]
#![feature(test)]
#![test_runner(test_runner_mod::esp_test_runner)]
esp32framework::esp_test::use_esp32_tests!(esp32framework::esp_test);
{%- endif %}
use esp32framework::Microcontroller;

fn main(){
    let mut micro = Microcontroller::take();
    let mut timer_driver = micro.get_timer_driver().unwrap();
    timer_driver.interrupt_after(3_000_000, || {println!("Hello World")});
    timer_driver.enable().unwrap();
    println!("WARNING, Hello World incoming...");
    micro.wait_for_updates(None);
}
{% if test %}
#[cfg(test)]
mod test{
    #[test]
    fn passing_test(){
        assert!(true)
    }
}
{% endif %}