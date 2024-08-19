use defmt::info;
use embassy_executor::{Executor, Spawner};
use embassy_rp::multicore::Stack;
use embassy_time::Timer;
use static_cell::StaticCell;

static mut CORE1_STACK: Stack<4096> = Stack::new();
static EXECUTOR0: StaticCell<Executor> = StaticCell::new();
static EXECUTOR1: StaticCell<Executor> = StaticCell::new();

#[embassy_executor::task]
async fn core0_task() {
    loop {
        info!("Core 0 ticks!");
        Timer::after_secs(5).await;
    }
}

#[embassy_executor::task]
async fn core1_task() {
    loop {
        info!("Core 1 ticks!");
        Timer::after_secs(3).await;
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = embassy_rp::init(Default::default());

    embassy_rp::multicore::spawn_core1(p.CORE1, unsafe { &mut CORE1_STACK }, move || {
        let executor1 = EXECUTOR1.init(Executor::new());
        executor1.run(|spawner| spawner.spawn(core1_task()).unwrap());
    });

    let executor0 = EXECUTOR0.init(Executor::new());
    executor0.run(|spawner| spawner.spawn(core0_task()).unwrap())
}
