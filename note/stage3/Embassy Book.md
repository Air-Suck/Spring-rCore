# Embassy Book

参考：[Embassy book](https://embassy.dev/book/)

## 简介

Embassy是一个**project**，他让async和await提供的异步成为了嵌入式开发最好的选项之一

### 什么是异步

实际上就是原来的概念，这里直接贴原文了，没什么必要翻译感觉。。。

> When handling I/O, software must call functions that block program execution until the I/O operation completes. When running inside of an OS such as Linux, such functions generally transfer control to the kernel so that another task (known as a “thread”) can be executed if available, or the CPU can be put to sleep until another task is ready.
>
> Because an OS cannot presume that threads will behave cooperatively, threads are relatively resource-intensive, and may be forcibly interrupted they do not transfer control back to the kernel within an allotted time. If tasks could be presumed to behave cooperatively, or at least not maliciously, it would be possible to create tasks that appear to be almost free when compared to a traditional OS thread.
>
> In other programming languages, these lightweight tasks are known as “coroutines” or ”goroutines”. In Rust, they are implemented with async. Async-await works by transforming each async function into an object called a future. When a future blocks on I/O the future yields, and the scheduler, called an executor, can select a different future to execute.
>
> Compared to alternatives such as an RTOS, async can yield better performance and lower power consumption because the executor doesn’t have to guess when a future is ready to execute. However, program size may be higher than other alternatives, which may be a problem for certain space-constrained devices with very low memory. On the devices Embassy supports, such as stm32 and nrf, memory is generally large enough to accommodate the modestly-increased program size.

### 什么是Embassy

Embassy由以下几个模块组成，他们可以组合起来使用，也可以**独立使用**：

- **执行器** [embassy-executor](https://docs.embassy.dev/embassy-executor/)：embassy-executor是一个异步执行器，它通常执行固定数量的任务，并且在启动的时候就确定了任务的数量，尽管在启动之后还能继续增加任务的数量。执行器也可以提供一个系统时钟，你可以使用它来实现异步，也可以用它来实现阻塞延时。对小于一毫秒的情况，最好使用阻塞延时，因为上下文切换所带来的开销在此时就太大了，并且执行器也没有办法提供如此精确的计时

- **硬件抽象层**（HALs）：硬件抽象层实现了安全的Rust API，通过这些API你可以使用各种各样的外设，如USART, UART, I2C, SPI, CAN, and USB，而不需要直接操纵寄存器。Embassy 提供了异步API实现以及阻塞API实现。例如DMA就很适合使用异步方式，而GPIO就比较适合使用阻塞的方式，或者说，同步的方式。

  Embassy维护了一些硬件，但是您可以在任何使用Embassy的项目中使用HAL。

  - [embassy-stm32](https://docs.embassy.dev/embassy-stm32/), for all STM32 microcontroller families.
  - [embassy-nrf](https://docs.embassy.dev/embassy-nrf/), for the Nordic Semiconductor nRF52, nRF53, nRF91 series.
  - [embassy-rp](https://docs.embassy.dev/embassy-rp/), for the Raspberry Pi RP2040 microcontroller.
  - [esp-rs](https://github.com/esp-rs), for the Espressif Systems ESP32 series of chips.
  - [ch32-hal](https://github.com/ch32-rs/ch32-hal), for the WCH 32-bit RISC-V(CH32V) series of chips.

  > **注意**：很多人想知道是否可以单独使用HAL。答案是可以！因为HAL并不依赖于Embassy执行器。你甚至可以在没有异步的情况下使用他们，因为他们实现了 [Embedded HAL](https://github.com/rust-embedded/embedded-hal)的阻塞和异步特性，正如上面所说的，Embassy 提供了异步API实现以及阻塞API实现。

- **网络**[embassy-net](https://docs.embassy.dev/embassy-net/)：Embassy网络协议栈实现了常用的网络功能，如以太网、IP、TCP、UDP, ICMP and DHCP。异步极大地简化了超时管理以及多服务连接并发。还能找到一些关于WIFI和一以太网的驱动。

- 蓝牙[nrf-softdevice](https://github.com/embassy-rs/nrf-softdevice)：这个模块为nRF52 微控制器提供了蓝牙低功耗4.x和5.x的支持。

- LoRa[lora-rs](https://github.com/lora-rs/lora-rs)：不太关心，直接摆上原文：

  > supports LoRa networking on a wide range of LoRa radios, fully integrated with a Rust LoRaWAN implementation. It provides four crates — lora-phy, lora-modulation, lorawan-encoding, and lorawan-device — and basic examples for various development boards. It has support for STM32WL wireless microcontrollers or Semtech SX127x transceivers, among others.

- USB[embassy-usb](https://docs.embassy.dev/embassy-usb/)：稍微看看原文就好了

  >  implements a device-side USB stack. Implementations for common classes such as USB serial (CDC ACM) and USB HID are available, and a rich builder API allows building your own.

- Bootloader and DFU[embassy-boot](https://github.com/embassy-rs/embassy/tree/master/embassy-boot)：

  >  is a lightweight bootloader supporting firmware application upgrades in a power-fail-safe way, with trial boots and rollbacks.

### 什么是DMA？

概念问题就不翻译了，之后有需求再来翻译吧

实际上就是计组课上讲的DMA

> For most I/O in embedded devices, the peripheral doesn’t directly support the transmission of multiple bits at once, with CAN being a notable exception. Instead, the MCU must write each byte, one at a time, and then wait until the peripheral is ready to send the next. For high I/O rates, this can pose a problem if the MCU must devote an increasing portion of its time handling each byte. The solution to this problem is to use the Direct Memory Access controller.
>
> The Direct Memory Access controller (DMA) is a controller that is present in MCUs that Embassy supports, including stm32 and nrf. The DMA allows the MCU to set up a transfer, either send or receive, and then wait for the transfer to complete. With DMA, once started, no MCU intervention is required until the transfer is complete, meaning that the MCU can perform other computation, or set up other I/O while the transfer is in progress. For high I/O rates, DMA can cut the time that the MCU spends handling I/O by over half. However, because DMA is more complex to set-up, it is less widely used in the embedded community. Embassy aims to change that by making DMA the first choice rather than the last. Using Embassy, there’s no additional tuning required once I/O rates increase because your application is already set-up to handle them.

### 示例

Embassy 为所有支持的HAL都提供了示例。你可以在`examples/`目录下找到这些示例。

主循环示例：

```rust
use embassy_executor::Spawner;
use embassy_time::Timer;
use log::*;

#[embassy_executor::task]
async fn run() {
    loop {
        info!("tick");
        Timer::after_secs(1).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp_nanos()
        .init();

    spawner.spawn(run()).unwrap();
}
```

### Embassy的实际应用

> Here are known examples of real-world projects which make use of Embassy. Feel free to [add more](https://github.com/embassy-rs/embassy/blob/main/docs/pages/embassy_in_the_wild.adoc)!
>
> - [RMK: A feature-rich Rust keyboard firmware](https://github.com/haobogu/rmk/)
>   - RMK has built-in layer support, wireless(BLE) support, real-time key editing support using vial, and more!
>   - Targets STM32, RP2040, nRF52 and ESP32 MCUs
> - [Printhor: The highly reliable but not necessarily functional 3D printer firmware](https://github.com/cbruiz/printhor/)
>   - Targets some STM32 MCUs
> - [Card/IO firmware](https://github.com/card-io-ecg/card-io-fw) - firmware for an open source ECG device
>   - Targets the ESP32-S3 or ESP32-C6 MCU
> - The [lora-rs](https://github.com/lora-rs/lora-rs) project includes [various standalone examples](https://github.com/lora-rs/lora-rs/tree/main/examples/stm32l0/src/bin) for NRF52840, RP2040, STM32L0 and STM32WL
> - [Air force one: A simple air quality monitoring system](https://github.com/matoushybl/air-force-one)
>   - Targets nRF52 and uses nrf-softdevice
> - [YLab Edge Go](https://github.com/schmettow/ylab-edge-go) and [YLab Edge Pro](https://github.com/schmettow/ylab-edge-pro) projects develop firmware (RP2040, STM32) for capturing physiological data in behavioural science research. Included so far are:
>   - biopotentials (analog ports)
>   - motion capture (6-axis accelerometers)
>   - air quality (CO2, Temp, Humidity)
>   - comes with an app for capturing and visualizing data [[Ystudio](https://github.com/schmettow/ystudio-zero)]

### 一些资源

一些关于异步Rust和Embassy的阅读资料：

- [Comparsion of FreeRTOS and Embassy](https://tweedegolf.nl/en/blog/65/async-rust-vs-rtos-showdown)
- [Tutorials](https://dev.to/apollolabsbin/series/20707)
- [Firmware Updates with Embassy](https://blog.drogue.io/firmware-updates-part-1/)

之后可以来看看

## 快速入门

这个部分主要是针对Embassy初学者。在这里展示了如何入门、如何构建你自己的项目以及其他的最佳实践

### 开始

首先需要在开发环境中安装一下的工具：

- [rustup](https://rustup.rs/) ：Rust 工具链，用于编译Rust代码。
- [probe-rs](https://probe.rs/) ：用于将固件烧写在设备上。使用openocd也是可以的

如果没有开发板的话还可以直接在PC上使用`std` examples执行Embassy

综设中已经配好了openocd，就直接使用openocd了

#### 在开发板上执行示例

这里就先不翻译了，先按照目录结构来记录一些问题。如果之后有需求的话再全篇翻译

这里给出stm32f401的Embassy示例代码：[stm32f4示例代码](https://github.com/embassy-rs/embassy/tree/main/examples/stm32f4)

#### 执行示例

需要clone仓库：[Embassy Repository](https://github.com/embassy-rs/embassy)

这里作者建议先点个灯，经典，嵌入式人的“Hello World”

这里有一段话：

> How does the `cargo run` command know how to connect to our board and program it? In each `examples` folder, there’s a `.cargo/config.toml` file which tells cargo to use [probe-rs](https://probe.rs/) as the runner for ARM binaries in that folder. probe-rs handles communication with the debug probe and MCU. In order for this to work, probe-rs needs to know which chip it’s programming, so you’ll have to edit this file if you want to run examples on other chips.

这里作者说的是，目标板是.cargo/config.toml指定的，在这个文件中有一个变量：runner，他指定了cargo run指令应该执行的操作。在原本的文件中是使用probe进行烧录，但是我之前是使用Jlink烧录的，所以这里就不用cargo run进行烧录了，而是通过cargo build编译之后再通过JLink相关的指令烧录到板子上。

需要注意的是这里没办法直接烧录cargo build生成的文件，因为生成的文件本质上是一个elf文件，直接烧录的话会出现以下的报错：

> File is of unknown / unsupported format.

所以这里需要使用相关的指令将elf文件转换为二进制文件（.bin文件），这样才能正常烧录

转换为bin文件之后就能正常烧录了：

![image-20240607214226307](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240607214226307.png)

但是需要注意的是这里的引脚可能是不对的，需要修改为板子上的**引脚PA5**。但是成功烧录之后灯还是没有闪烁，可能是因为有一些地方没有配好，就先往下看吧，感觉是因为还不太会用Embassy导致的。后面会有自己创建一个Embassy工程的例子，到那里的时候再回来看看这里应该就能解决了

### 一个基本的嵌入式应用程序

#### main

首先需要现在setting.json文件中将stm32f4的注释打开，并且需要将`"embassy-stm32/Cargo.toml"`注释,，这样rust-analyzer才能识别我的工作目录

> To work on the examples, comment **the line above** and **all of the cargo.features lines**,
>
> then uncomment **ONE** line below to select the chip you want to work on.
>
> This makes rust-analyzer work on the example crate and all its dependencies.

##### Bare metal

首先在项目中需要两个feature：

```rust
#![no_std]
#![no_main]
```

##### 解决错误

使用下面的库可以直接在终端中打印信息

```rust
use {defmt_rtt as _, panic_probe as _}; // global logger
```

##### 任务声明

> An embassy task must be declared `async`, and may NOT take generic arguments. In this case, we are handed the LED that should be blinked and the interval of the blinking.

一个嵌入式任务必须被声明为async的，并且不能接收泛型参数。在点灯的例子中（也就是下例），传参是一个需要被点亮的led以及闪烁的间隔

```rust
#[embassy_executor::task]
async fn blinker(mut led: Output<'static>, interval: Duration) {
    loop {
        led.set_high();
        Timer::after(interval).await;
        led.set_low();
        Timer::after(interval).await;
    }
}
```

由于上面这个任务是异步的，所以不会出现忙等现象，他是通过Embassy的定时器来进行yield的，这个时候在闪烁的间隔中处理器就可以sleep了

##### 主函数

这里给出一段代码

```rust
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    let led = Output::new(p.P0_13, Level::Low, OutputDrive::Standard);
    unwrap!(spawner.spawn(blinker(led, Duration::from_millis(300))));
}
```

- 在Embassy中应用程序的主函数需要使用\#[embassy_executor::main]宏来进行定义。并且主函数接收一个Spawner对象，主函数通过这个对象可以创建新的任务

- 接下来需要初始化硬件抽象层，上面给出的例子是使用默认的硬件抽象层

- 接下来就可以通过初始化hal返回的Peripherals结构体来访问控制器的外设了。在上例中就通过GPIO口创建了一个LED灯对象。然后就需要使用主函数的Spawner对象来spawn一个blinker任务了

  当blinker任务被创建的时候到底发生了什么？首先需要知道，main其实与其他的异步任务没有什么区别，只不过main函数只能有一个并且他有特定类型的函数参数（也就是Spawner），这些都是\#[embassy_executor::main]宏决定的，这个宏会做如下的事情：

  - 创建一个执行器
  - 为程序入口定义一个主任务
  - 运行执行器并spawn一个主任务（也就是main）

  当然也可以不使用这个宏，只不过如果不使用的话就不会自动生成一个Embassy执行器了。这个时候就需要自己手动实现一个运行时来支持异步了

####  The Cargo.toml

在Cargo.toml文件中定义了embassy 的相关依赖。这里面依赖的定义都是取决于当前的项目的

有一句话：

> Depending on your microcontroller, you may need to replace `embassy-nrf` with something else (`embassy-stm32` for STM32. Remember to update feature flags as well).

**这最后一句话真是救我的命，我才发现Cargo.toml文件里面定义的feature不是401，修改了之后灯就能正常闪烁了**

定义stm32f401re 这个feature就会启用`stm32-metapac` crate中的`stm32f401re`功能，进而为我提供了401xe的支持

> `stm32f401re = [ "stm32-metapac/stm32f401re" ]`定义了一个名为`stm32f401re`的功能。当这个功能被启用时，它会启用`stm32-metapac` crate中的`stm32f401re`功能。
>
> `stm32-metapac/stm32f401re`是对`stm32-metapac` crate中的一个特定功能的引用。这个功能提供了对STM32F401RE微控制器的硬件抽象。
>
> 所以，`stm32f401re = [ "stm32-metapac/stm32f401re" ]`的作用是，当你在构建项目时启用`stm32f401re`功能，Cargo会启用`stm32-metapac` crate中的`stm32f401re`功能，从而提供对STM32F401RE微控制器的硬件抽象。

### Project Structure

一个基本的Embassy项目结构为：

```
{} = Maybe

my-project
|- .cargo
|  |- config.toml
|- src
|  |- main.rs
|- build.rs
|- Cargo.toml
|- {memory.x}
|- rust-toolchain.toml
```

下面开始逐一介绍这些文件（上面已经介绍了Cargo.toml和main）

#### .cargo/config.toml

这个文件中描述了目标平台，并且指示了probe烧录的时候的行为，里面的代码为：

```toml
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
# replace STM32F429ZITx with your chip as listed in `probe-rs chip list`
runner = "probe-rs run --chip STM32F429ZITx"
[build]
target = "thumbv7em-none-eabi"

[env]
DEFMT_LOG = "trace"
```

- runner：指示执行cargo run时的行为（所以需要在这里修改目标板）。由于我没有使用probe，而是直接使用Jlink烧录，就不用这个了
- target：指示目标平台
- DEFMT_LOG：表示的是日志等级

#### build.rs

build脚本链接了defmt（日志输出）以及memory.x文件。由于这个文件对不同的板子的区别还是比较大的，所以作者给出的建议是，需要使用的时候就直接从example中复制

#### Cargo.toml

在这个文件中设置需要使用哪些**Embassy**组件（通过修改feature）。作者在这里举了一个feature：

> ###### Time
>
> - tick-hz-x: Configures the tick rate of `embassy-time`. Higher tick rate means higher precision, and higher CPU wakes.
> - defmt-timestamp-uptime: defmt log entries will display the uptime in seconds.

是关于定时器的feature

#### memory.x

这个文件概述了程序的flash/ram使用情况。一个Memory.x文件示例如下：

```
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* These values correspond to the NRF52840 with Softdevices S140 7.0.1 */
  FLASH : ORIGIN = 0x00027000, LENGTH = 868K
  RAM : ORIGIN = 0x20020000, LENGTH = 128K
}
```

这个看着就很像链接脚本中指定的，就是flash、ram的起始地址和长度

#### rust-toolchain.toml

这个文件设置了rust的版本以及相关的设置，作者也给出了一个示例

```toml
[toolchain]
channel = "nightly-2023-08-19" # <- as of writing, this is the exact rust version embassy uses
components = [ "rust-src", "rustfmt" ] # <- optionally add "llvm-tools-preview" for some extra features like "cargo size"
targets = [
    "thumbv6m-none-eabi" # <-change for your platform
]
```

Embassy中的该文件是放在主目录下的

### Starting a new project

终于可以自己开始跟着写代码了

#### Tools for generating Embassy projects

整理给出了一些构建Embassy工程的工具

##### CLI

- [cargo-embassy](https://github.com/adinack/cargo-embassy) (STM32 and NRF)

直接在终端中输入：

```shell
cargo install cargo-embassy
```

即可下载cargo-embassy。

这个工具主要是用来快速构建一个Embassy工程的。在终端中输入下面的命令（如果使用的是stm32401re）：

```
cargo embassy init my_project --chip stm32f401re
```

就会自动构建出项目的目录（也就是上面项目目录中介绍的那个样子），并且与目标板相关的配置都自动生成了：

![image-20240608112439432](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240608112439432.png)

从这里也可以看见自动生成的调试工具还是probe-rs。

##### cargo-generate

- [embassy-template](https://github.com/lulf/embassy-template) (STM32, NRF, and RP)
- [embassy-rp2040-template](https://github.com/bentwire/embassy-rp2040-template) (RP)

这个也是用来生成模板的。这里就选择使用cargo embassy了

#### 从头开始一个项目

这里是使用cargo new来创建一个rust项目，我就直接使用cargo embassy来创建一个项目了。

##### The .cargo/config.toml

涉及到conf文件的配置，但是由于我并没有使用cargo run指令来运行程序，所以就完全没有必要配置了。实际上就是修改一下config.toml文件中的runner选项

##### Cargo.toml

在这里就是加一些依赖的，直接使用cargo Embassy就能自动生成对应板子需要的依赖了。对于点灯程序，只需要`embassy-stm32`, `embassy-executor` and `embassy-time`.作者说在写的时候embassy 在 crates.io中还不是最新的。但是我直接使用cargo embassy就不需要管这个了。这一步我就直接跳过了

##### rust-toolchain.toml

使用cargo embassy也能自动生成，在这里也不赘述了。而关于直接文件在前面的项目结构中也有介绍到，就不再提了

##### build.rs

使用cargo embassy可以自动生成。文件作用前面有提到，是为了链接日志输出（defmt）还有memory.x的

##### Building and running

执行指令：

```shell
cargo run --release
```

在我的项目中有Makefile，就直接make就好了

### 最佳实践

这里主要是在考虑嵌入式系统有限的内存空间。在这种上下文下就需要多考虑一点跟内存分配有关的问题了

#### Passing Buffers by Reference

意思是传递一个缓冲区的时候使用引用。这里作者给了一个例子：

```rust
fn process_buffer(mut buf: [u8; 1024]) -> [u8; 1024] {
    // do stuff and return new buffer
    for elem in buf.iter_mut() {
        *elem = 0;
    }
    buf
}

pub fn main() -> () {
    let buf = [1u8; 1024];
    let buf_new = process_buffer(buf);
    // do stuff with buf_new
    ()
}
```

首先需要知道copy trait。这个类型也是能自动导出的（跟Unpin一样）。所以在上面的例子中buf这个数组也是实现了copy trait的。所以这个时候如果没有给函数process_buffer传递buf的引用的话，就会导致函数传参的时候进行copy（因为函数传参也算是一个赋值操作，这个时候就会执行copy trait）。并且在被调函数返回的时候返回了这个数组对象，所以这个时候被调函数会在主调函数的栈帧中存放这个1024字节的数组。这个时候在主调函数的栈空间中就有两个1024字节的数组，这很容易导致栈溢出（特别是在内存空间有限的嵌入式系统上）

所以最好的解决方法就是——传递缓冲区的引用：

```rust
fn process_buffer<'a>(buf: &'a mut [u8]) -> &'a mut[u8] {
    for elem in buf.iter_mut() {
        *elem = 0;
    }
    buf
}

pub fn main() -> () {
    let mut buf = [1u8; 1024];
    let buf_new = process_buffer(&mut buf);
    // do stuff with buf_new
    ()
}
```

这里直接传递buf的引用，就不会导致栈空间因为一些不必要的copy而被顶爆

### 从裸板到异步Rust

这一节的主要目的就是为了阐明一些Embassy里面的不同的层，以及这些层的作用。这里作者是以STM32 IOT01A 为例进行描述的，但是所有的32的板子都大同小异的。这里作者将会带着我们写一个按键点灯程序。作者会介绍**不同版本**（一层一层向上）的按键点灯程序是什么样的

#### PAC version

Peripheral Access Crate (PAC)。

如果不考虑直接读/写内存地址（汇编或者裸指针干的事），PAC是访问外设和寄存器的最底层API。他提供了不同的类型来让我们能够更轻易的访问外设寄存器，但是他并不禁止你写出unsafe的代码。

不建议直接使用PAC来编写应用程序，但是如果你想使用的功能没有在更高层中被暴露出来的话，那么你就需要使用PAC了

下面给出使用PAC编写的按键点灯程序：

```rust
#![no_std]
#![no_main]

use pac::gpio::vals;
use {defmt_rtt as _, panic_probe as _, stm32_metapac as pac};

#[cortex_m_rt::entry]
fn main() -> ! {
    // Enable GPIO clock
    let rcc = pac::RCC;
    unsafe {
        rcc.ahb2enr().modify(|w| {
            w.set_gpioben(true);
            w.set_gpiocen(true);
        });

        rcc.ahb2rstr().modify(|w| {
            w.set_gpiobrst(true);
            w.set_gpiocrst(true);
            w.set_gpiobrst(false);
            w.set_gpiocrst(false);
        });
    }

    // Setup button
    let gpioc = pac::GPIOC;
    const BUTTON_PIN: usize = 13;
    unsafe {
        gpioc.pupdr().modify(|w| w.set_pupdr(BUTTON_PIN, vals::Pupdr::PULLUP));
        gpioc.otyper().modify(|w| w.set_ot(BUTTON_PIN, vals::Ot::PUSHPULL));
        gpioc.moder().modify(|w| w.set_moder(BUTTON_PIN, vals::Moder::INPUT));
    }

    // Setup LED
    let gpiob = pac::GPIOB;
    const LED_PIN: usize = 14;
    unsafe {
        gpiob.pupdr().modify(|w| w.set_pupdr(LED_PIN, vals::Pupdr::FLOATING));
        gpiob.otyper().modify(|w| w.set_ot(LED_PIN, vals::Ot::PUSHPULL));
        gpiob.moder().modify(|w| w.set_moder(LED_PIN, vals::Moder::OUTPUT));
    }

    // Main loop
    loop {
        unsafe {
            if gpioc.idr().read().idr(BUTTON_PIN) == vals::Idr::LOW {
                gpiob.bsrr().write(|w| w.set_bs(LED_PIN, true));
            } else {
                gpiob.bsrr().write(|w| w.set_br(LED_PIN, true));
            }
        }
    }
}
```

这个就跟综设第一阶段里面使用的方法一模一样。。。就是**直接操作寄存器**。也能看见这里面很多的unsafe代码块（PAC层不禁止使用unsafe）

正如你所见，需要**很多的代码**来使能外设时钟并且设置应用程序的输入输出引脚。

这个应用程序的另一个缺点就是：当轮询按键的状态的时候，按键是一个忙等的循环。这使得MCU不能利用任何的睡眠模式来提高能源的利用率

#### HAL version

为了简化我们的应用程序，我们可以使用HAL层。HAL层暴露了更高层级的API，这些API实现了以下的细节：

- 当使用外设时，自动使能外设时钟
- 从更高级的类型中派生并应用寄存器的设置（大概的意思就是会自动帮我们完成一些固定的配置）
- 实现了embedded-hal traits让外设在第三方驱动中更有用

下面给出使用HAL编写的按键点灯程序：

```rust
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use {defmt_rtt as _, panic_probe as _};

#[entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());
    let mut led = Output::new(p.PB14, Level::High, Speed::VeryHigh);
    let button = Input::new(p.PC13, Pull::Up);

    loop {
        if button.is_low() {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}
```

这里只需要使用init方法并且传入一个默认的类型就能初始化一个开发版，并且在配置LED灯和按键的时候，只需要传递一些需要自定义的参数即可（如引脚号、上下拉模式、引脚速度等等）。这应该就是上面说的“从更高级的类型中派生并应用寄存器的设置”。并且从代码量上来看使用HAL比使用PAC码量少太多了（这个就对应了综设里面的HAL库了吧。但是这个显然做的比HAL库更好。他帮我隐藏了很多丑陋的硬件细节）

正如你所见，代码简单了很多，甚至没有使用任何的异步代码（所以说HAL可以单独使用）。Input与Output这两个类型隐藏了所有关于GPIO寄存器的配置，并且提供了更简单的API来查看按键的状态以及改变LED引脚的输出。

但与使用PAC同样的缺点是存在忙等

#### Interrupt driven

为了节能，我们需要**使用中断**修改这个应用程序，让它可以知道到按键是否被按下。

一旦中断被配置了，应用程序就可以指示MCU进入休眠模式（可以通过中断被唤醒），进而消耗更少的能源

鉴于Embassy聚焦于异步Rust（会在下一个示例中介绍。这个意思应该是Embassy没有提供中断相关的处理？），在这个示例中必须结合使用HAL和PAC以便能够使用中断（就是需要PAC层来直接操作寄存器进而配置中断相关的部分）。由于这个原因，应用程序也包含了一些函数来访问PAC（在下面的示例中并没有展示出来）

```rust
#![no_std]
#![no_main]

use core::cell::RefCell;

use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::NVIC;
use cortex_m_rt::entry;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::{interrupt, pac};
use {defmt_rtt as _, panic_probe as _};

static BUTTON: Mutex<RefCell<Option<Input<'static>>>> = Mutex::new(RefCell::new(None));
static LED: Mutex<RefCell<Option<Output<'static>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());
    let led = Output::new(p.PB14, Level::Low, Speed::Low);
    let mut button = Input::new(p.PC13, Pull::Up);

    // 配置中断
    cortex_m::interrupt::free(|cs| {
        enable_interrupt(&mut button);

        LED.borrow(cs).borrow_mut().replace(led);
        BUTTON.borrow(cs).borrow_mut().replace(button);

        unsafe { NVIC::unmask(pac::Interrupt::EXTI15_10) };
    });

    loop {
        // 处理器休眠，wait for event
        cortex_m::asm::wfe();
    }
}

// 外部中断ISR
#[interrupt]
fn EXTI15_10() {
    cortex_m::interrupt::free(|cs| {
        let mut button = BUTTON.borrow(cs).borrow_mut();
        let button = button.as_mut().unwrap();

        let mut led = LED.borrow(cs).borrow_mut();
        let led = led.as_mut().unwrap();
        if check_interrupt(button) {
            if button.is_low() {
                led.set_high();
            } else {
                led.set_low();
            }
        }
        clear_interrupt(button);
    });
}
```

上面简单的示例又变得复杂起来了，主要是因为我们需要在全局作用域上保存按键和LED的状态（之前状态只在main函数的作用域上可见。但是在这个示例上要求在ISR和main中都可见，就只能做成全局变量了）

为了做到这个（将状态保存在全局作用域上），BUTTON和LED必须加锁，并且在访问状态的时候，需要关闭中断以便于能够访问外设。

幸运的是，在Embassy中有一个更优雅的方式来解决忙等的问题

**疑问**为什么要关闭中断？虽然这里全局变量的访问是临界区，但是既然已经使用了Mutex进行全局变量访问控制的，那么为什么还需要在进入临界区的时候关中断？（锁，或者说信号量以及中断是两种进行临界区保护的方式吧。使用信号量就允许任务切换了，但是能够通过信号量机制来保护临界区；而使用中断的话就不允许任务进行切换进而保证了临界区的互斥访问。那么为什么这里既要使用锁又要使用关中断？

#### Async version

是时候使用发挥Embassy的全部威力了。实际上，Embassy有一个为异步任务执行提供支持的异步执行器（或者运行时）。执行器会轮询一系列的任务（在**编译时**被定义。所以Embassy中的任务是**不支持泛型**的）

给出代码

```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Level, Output, Pull, Speed};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let mut led = Output::new(p.PB14, Level::Low, Speed::VeryHigh);
    let mut button = ExtiInput::new(p.PC13, p.EXTI13, Pull::Up);

    loop {
        button.wait_for_any_edge().await;
        if button.is_low() {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}
```

这里本质上也是通过中断来处理的。

到这里