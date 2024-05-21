# 5.2

终于把学校的事情做完了，可以开始做题哩

## 环境的配置

没啥好说的，直接跟着Guide走就好了：[rCore-Tutorial-Guide 2024](https://learningos.cn/rCore-Tutorial-Guide-2024S/0setup-devel-env.html)

## 从Makefile看起

Makefile指导着编译器的工作，先看看Makefile能让我更好地了解我的代码是怎么构建成一个能在计算机上跑的程序的，我觉得这对非常有利于一个人去从宏观的角度上去了解整个项目。

这里看的是os文件夹下的Makefile，根目录下的Makefile没啥好看的，大概就是一些跟docker构建项目有关系的指令，这里就不再说了。os文件夹中的Makefile除去伪目标之后也没有几行了，这里就主要看看执行make run之后到底发生了什么。

执行make run时的依赖关系如下：kernel->env,内核二进制文件->build->run-inner->run，下面逐个解释目标的作用

- kernel：构建内核，相当于就是编译生成elf文件吧
- 内核二进制文件：根据elf文件生成bin文件

- env：设置rust的执行环境，在里面创建了目标文件夹（riscv64gc-unknown-none-elf），并且安装了cargo-binutils（用于处理二进制文件的工具链），然后添加了rust-src（rust的源代码，常用于Rust的开发和调试），然后添加了llvm-tools-preview（暂时还不知道有什么用？**疑问**？）

- build：依赖于env，内核二进制文件，没有做任何事

- run-inner：拉一个qemu虚拟机起来，通过一条qemu的指令实现。在这条指令中指定了bootloader，需要烧录的文件，以及程序的加载地址（也就是将程序烧录到设备上的什么位置）。入口地址指定为0x80200000（这个地址应该是代码段的起始，或者是flash的位置**疑问**）

- run：依赖于run-inner，没有做任何事

  至此run伪目标就创建出来了。由于run目标的构建过程中构建了env目标，所以会安装一些组件，这也是Guide上建议我们使用make run的原因

此外Makefile中还提供了一些其他的指令，如disasm还有disasm-vim，看名字也能猜出来是反汇编的指令。通过这两个指令可以构建一个根据elf文件反汇编生成的汇编文件。

另外发现Makefile中都没有直接指定所有rs文件的路径，跟c的Makefile很不一样，可能是rust的编译器还有包管理工具做了很多事情吧，之后可以看看**疑问**

理解了Makefile之后就要

**PS**：由于所有的实验我都是感觉有东西就随手记，所有我不知道的小点我都分开写了，所以这里可能记得东西有点杂，）

## Guide1——ch1

看过Makefile之后就可以直接按照Guide里面的内容开始着手实践了

- 在第一章中有一个工具可以展示一个文件的结构：

  ```bash
  tree os
  ```

  需要注意的是这个指令只能在os的上级目录中使用。使用之后将展示os的文件结构（对项目而言就是代码树了）

![image-20240502141923252](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240502141923252.png)

我对上图的感触还是比较深的，因为在我们的综设项目的c语言代码中，由于代码是直接烧在裸板上的，所以所有的标准库都没办法使用。现在看来就是因为我们没有c标准库需要的相关依赖（如操作系统，系统调用等）。而我们实际上在裸板上重新为库函数中的printf函数实现了他需要的系统调用_word，经过这样的处理之后，我们在裸板上也能按照我们的需要使用printf函数将数据输出到串口上了。需要注意的是，所有语言的库函数都是其对应的编译器进行自动引入的

- 关于对使用了rustc --version --verbose指令之后输出信息中host的解释

  > x86_64-unknown-linux-gnu： CPU 架构是 x86_64，CPU 厂商是 unknown，操作系统是 linux，运行时库是 gnu libc
  >
  > riscv64gc-unknown-none-elf： 的 CPU 架构是 riscv64gc，厂商是 unknown，操作系统是 none， elf 表示**没有标准的运行时库**。

  所以riscv64gc-unknown-none-elf才是我们的操作系统的目标平台，也就是烧在一个riscv裸板上，并且没有任何的库

- 目标平台的修改是在.cargo目录下的config.toml中实现的：

  ```toml
  [build]
  target = "riscv64gc-unknown-none-elf"
  ```

  > 这将使 cargo 工具在 os 目录下默认会使用 riscv64gc-unknown-none-elf 作为目标平台

- #![no_std]注解可以告诉 Rust 编译器不使用 Rust 标准库 std 转而使用核心库 core

- #[panic_handler]标记一个错误处理函数。在我们自己的函数上使用 #[panic_handler] 就能告知编译器采用我们的错误处理函数的实现

- 下面这条指令能查看文件的头部（如查看elf头什么的。elf的头中还是包含了很多的信息的）：

  ```bash
  rust-readobj -h 文件名
  ```

- 下面这条指令能导出反汇编文件

  ```bash
  rust-objdump -S 文件名
  ```

- rust编译器指定的入口函数是_start。这个函数在编写rust应用程序的时候是在rsut的标准库中提供的（所以前面说的start语义项就是这个？**疑问**）

- rust的内核库中提供了write trait

- 格式化宏的理解：

  ```rust
  #[macro_export]
  macro_rules! print {
      ($fmt: literal $(, $($arg: tt)+)?) => {
          $crate::console::print(format_args!($fmt $(, $($arg)+)?));
      }
  }
  ```

  - #[macro_export]：这个属性表示这个宏是公开的，可以在其他模块中使用。
  - macro_rules! print：这定义了一个名为print的宏。
  - ($fmt: literal $(, $($arg: tt)+)?)：这是宏的参数列表。$fmt: literal表示第一个参数是一个字面量（也就是printf中的第一个格式化字符串），$(, $($arg: tt)+)?表示后面可以跟随零个或多个（正则表达式中的？跟这里相同）参数，这些参数的类型是tt，tt是"token tree"的缩写，表示任意的Rust语法结构。
  - $crate::console::print(format_args!($fmt $(, $($arg)+)?));：这是宏的主体，它调用了console::print函数（也就是前面自己实现的print函数），并使用format_args!宏来格式化参数。（format_args!宏在核心库中可用）

  println只是在末尾处增加了一个换行符

  从这里看出，如果要实现在裸板上使用print函数的话只需要去实现一下print函数需要的系统调用即可。这也是我们综设项目中的执行逻辑，实现C中printf函数需要的系统调用_write，就能在裸板上实现printf函数了

- 裸板启动的过程感觉比较重要。这里就再写一遍

  - 首先PC先指向0x1000，执行一段固化的引导代码（这段loader代码应该是CPU厂家刷上去的）
  - 然后PC跳转到RustSBI（RustSBI是一个RISC-V架构下的一种接口规范，就好像是操作系统的操作系统，是为操作系统提供硬件初始化以及硬件接口的。并且在执行RustSBI时权限是最高的，是硬件级别）中，执行硬件的初始化（也就是boot部分），如果需要的话还需要进行代码的搬运（也就是loader部分）
  - 最后PC跳转到操作系统内核代码的部分开始执行，也就是KERNEL_ENTRY_PA指定的地址：0x80200000

  需要注意的是这些地址部分应该都是硬件决定的，就好像32的flash地址固定，ram的地址固定一样，RustSBI规定操作系统的代码就需要放在0x80200000处执行（本质上是硬件决定的**疑问**）。而RustSBI存放在0x80000000处就完全是由硬件决定的了（主要看那段固化代码怎么引导）

- 同样的可以在cargo（这玩意真牛逼）的配置文件中指定链接脚本

  ```toml
  [target.riscv64gc-unknown-none-elf]
  rustflags = [
      "-Clink-arg=-Tsrc/linker.ld", "-Cforce-frame-pointers=yes"
  ]
  ```

  其中-T选项后跟上的就是链接脚本的相对路径

- 关于链接脚本

  在连接脚本中指定的ENTRY(_start)和*(.text.entry)是不一样的，ENTRY(\_start)是指定最终生成的elf文件中程序的入口为\_start，但是\_start并不一定是放在text段的最低位置（copilot说不一定放在最低位置，但是综设上老师说的是一定会放在最低位置**疑问**），而\*(.text.entry)表示的是所有（通配符\*）参与链接的文件（在c语言中都是.o文件）的text段的入口。

  而在汇编代码中，有下面的这些部分：

  ```assembly
  	 .section .text.entry
       .globl _start
   _start:
       la sp, boot_stack_top
       call rust_main
  ```

  .section .text.entry这行汇编代码就指定了要将_start的部分放进.text.entry这个段中。这样全局就只有一个.text.entry段，所以就指定了将\_start放在RustSBI指定的0x80200000中

  在连接脚本中还有一行很有意思：

  ```
      . = ALIGN(4K);
  ```

  这个实际上就是分页对齐，要求每一个段都占整数个4K

  从链接脚本中可以看出，操作系统的栈是放在bss段的底部

  在rust中如果要使用链接脚本中的符号的话就需要extern “C”（C语言提供了一些获取链接脚本中的符号的函数）：

  ```rust
  extern "C" {
          fn sbss();
          fn ebss();
      }
  ```

  为什么这里是一个函数而不是一个全局符号？C语言中也是这样用的吗？**疑问**。自己有一个猜测就是这里声明为一个函数就相当于在给定的标号位置创建了一个函数（或者说将一个值解释为了一个空函数指针），只不过这个函数是一个空函数，也就满足了链接脚本中符号的一些要求——地址有用但是值没有用。在后面使用的过程中sbss和ebss就被解释为一个地址（甚至不是裸指针）

- 对bss段清零是为了保证所有未初始化变量的默认值是确定的。

- 需要将用户态的print函数修改为系统态的print本质上就是把内核库调用+系统调用换成了内核库调用+RustSBI调用

## Guide2——ch2

之后有时间了可以去看看riscv的特权机制：[RISC-V特权机制](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter2/1rv-privilege.html)

- 有一个宏：#[link_section = ".text.entry"]。这个宏的作用跟ch1中将_start放在.text.entry段中的的汇编代码是一样的。需要注意的是这里的\_start是用户程序的入口（text段基地址为0x80400000），而ch1中提到的\_start是操作系统的入口（text段基地址为0x80200000）。感觉存储位置的不同能为特权级机制打下一个硬件基础
- 系统调用的封装以及系统调用时需要的参数都是在用户库中实现的，而在操作系统层面实际上就是根据应用传来的参数真正执行相关的系统调用就好了（操作系统内核中实现系统调用函数应该就是使用RustSBI还有核心库）

- 关于link_app.S（这玩意是build.rs自动生成的。build文件就相当于是构建项目的脚本文件之一，cargo在执行build的时候会执行build脚本）

  下面这一段汇编挺有意思的：

  ```assembly
  app_0_start:
      .incbin "../user/target/riscv64gc-unknown-none-elf/release/hello_world.bin"
  app_0_end:
  ```

  这段汇编的意思就是直接"../user/target/riscv64gc-unknown-none-elf/release/hello_world.bin"这个二进制文件的内容直接贴在当前位置，也就是将这个用户程序放在了app_0_start标号和app_0_end之间，这里就相当于是直接把程序（包括用户程序的代码段，数据段等等，只不过是二进制形式的，而不是elf格式）放到操作系统的数据段了

  还有一段汇编代码：

  ```assembly
      .align 3
      .section .data
      .global _num_app
  _num_app:
      .quad 3
      .quad app_0_start
      .quad app_1_start
      .quad app_2_start
      .quad app_2_end
  ```

  这里是在_num_app的位置上放上了五个64位数（是64位是因为指令.quad，并不是因为字长。但是在后续的代码中，又直接使用usize来接收这些数据，说明当前cpu就是64位的），依次为：用户程序数量3、程序0起始地址、程序1起始地址、程序2起始地址、程序2结束地址，实际上在数据段上就是一个五个元素的64位数组

- lazy_static!这个宏用于创建只在首次访问时初始化的静态变量，就相当于是java中的**单例**，是为了解决静态变量无法在编译时确定值的情况

  > `lazy_static!` 宏提供了全局变量的运行时初始化功能。一般情况下，全局变量必须在编译期设置初始值， 但是有些全局变量的初始化依赖于运行期间才能得到的数据。 如这里我们借助 `lazy_static!` 声明了一个 `AppManager` 结构的名为 `APP_MANAGER` 的全局实例， 只有在它第一次被使用到的时候才会进行实际的初始化工作。

- 使用UPSafeCell将AppManager结构体包裹起来就可以实现APP_MANAGER不被重复获取实际上是用到了rust的语言特性——借用规则。rust中不允许一个变量有多个可变引用，而在UPSafeCell中实现的获取AppManager的方法是直接返回被UPSafeCell包裹的对象的可变引用。如果有多个地方在调用UPSafeCell的获取方法的话就会因为内部变量有多个可变引用而导致panic。从这里也可以看出，在进行UPSafeCell的new操作的时候是将AppManager结构体的所有权都转移给了UPSafeCell（实际上结构体就应该拥有其内部成员的所有权）

- 有一段代码

  ```rsut
  core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1);
  ```

  这个函数的作用是根据一个裸指针以及一个长度创建一个切片，num_app_ptr.add(1)提供裸指针（add1是因为首地址放的是元素的个数），num_app + 1提供元素个数，这里如果只需要获取应用程序的地址的话传入num_app 应该就好了，但是为了获取最后一个应用程序的结束地址，就需要多一个单元。这段代码结束之后就实现了_num_app数组在rust中的创建（到这里只是创建了一个切片）。接下来就是根据这个切片来创建一个数组，然后存放在AppManager中即可。到这里就能大概想出来操作系统是怎么运行的了，就是在操作系统的主函数中一直去根据这个数组来执行代码，直到数组被遍历完（也就是所有的用户程序执行结束），再让qemu退出并且打印全部执行完成的信息。

- load_app负责加载程序的二进制文件到约定的地址（相当于是一个应用程序的loader？），所以在用户程序的连接脚本中写的地址好像就没有那么重要了（所以原来的0x0也是可以正常执行的），因为最后都会由link_app.S加载到操作系统的数据段，并且会由load_app函数将应用程序搬运到0x80400000（这个位置是批处理操作系统和应用程序之间约定的常数地址）的位置。感觉这个load函数很有东西，这里就稍微解释一下（首先需要注意的是这段代码是操作系统的需要执行的，所以执行这段代码的时候系统应该在内核态，并且PC应该在0x80200000附近）：

  ```rust
      unsafe fn load_app(&self, app_id: usize) {
          //表示应用程序执行结束
          if app_id >= self.num_app {
              println!("All applications completed!");
              use crate::board::QEMUExit;
              crate::board::QEMU_EXIT_HANDLE.exit_success();
          }
          println!("[kernel] Loading app_{}", app_id);
          // clear app area
          //用0填充APP_BASE_ADDRESS（0x80400000）地址起的APP_SIZE_LIMIT个字（因为是usize）单元
          core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT).fill(0);
          //获取需要执行的应用程序的切片（在数据段中）
          let app_src = core::slice::from_raw_parts(
              self.app_start[app_id] as *const u8,
              self.app_start[app_id + 1] - self.app_start[app_id],
          );
          //获取应用程序目标地址的一段可变（可变是因为需要修改这段内存的值）切片
          let app_dst = core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len());
          //向app_dst（应用程序目标地址）中写入app_src（应用程序）的值
          app_dst.copy_from_slice(app_src);
          // Memory fence about fetching the instruction memory
          // It is guaranteed that a subsequent instruction fetch must
          // observes all previous writes to the instruction memory.
          // Therefore, fence.i must be executed after we have loaded
          // the code of the next app into the instruction memory.
          // See also: riscv non-priv spec chapter 3, 'Zifencei' extension.
          asm!("fence.i");//清除指令缓存
      }
  ```

- 关于缓存

  > 清空内存前，我们插入了一条奇怪的汇编指令 `fence.i` ，它是用来清理 i-cache 的。 我们知道， 缓存又分成 **数据缓存** (d-cache) 和 **指令缓存** (i-cache) 两部分，分别在 CPU 访存和取指的时候使用。 通常情况下， CPU 会认为程序的代码段不会发生变化，因此 i-cache 是一种只读缓存。 但在这里，**我们会修改会被 CPU 取指的内存区域**，使得 i-cache 中含有与内存不一致的内容， 必须使用 `fence.i` 指令手动清空 i-cache ，让里面所有的内容全部失效， 才能够保证程序执行正确性。

  “我们会修改会被 CPU 取指的内存区域”指的就是每一个应用程序都必须加载到0x80400000这个位置

- 第二章的倒数第二节结束之后我还是不知道系统是怎么进行任务的调度执行的，因为当在主函数中我只能看见其调用了一次run_next_app，之后在倒二节的内容中我就没有再看见这个函数了，就比较好奇如果没有其他地方调用的话，操作系统就应该只能执行一个任务。这显然是不合理的，所以我猜测可能在用户程序退出时会调用这个函数。而在倒二节的末尾也验证了我所猜想的：

  > 批处理操作系统完成初始化，或者应用程序运行结束/出错后会调用该函数。

  在每个任务退出的时候也会执行：

  ```rust
  pub fn sys_exit(exit_code: i32) -> ! {
      trace!("[kernel] Application exited with code {}", exit_code);
      run_next_app()
  }
  ```

- 连接脚本只要写一个，但是对每一个用户程序都是一样的，每一个用户程序编译的过程中都会使用到链接脚本和Makefile（因为要生成多个elf文件，或者说bin文件），这也就说明了其实每一个用户程序都是单独编译的（这样也才合理）。正是因为每一个用户程序都是单独编译的，所以每一个用户程序都应该需要一个_start函数作为该用户程序编译时的入口

- 摆一张表在这里以便后面速查（状态寄存器）：

  | CSR 名  | 该 CSR 与 Trap 相关的功能                                    |
  | ------- | ------------------------------------------------------------ |
  | sstatus | `SPP` 等字段给出 Trap 发生之前 CPU 处在哪个特权级（S/U）等信息 |
  | sepc    | 当 Trap 是一个异常的时候，记录 Trap 发生之前执行的最后一条指令的地址 |
  | scause  | 描述 Trap 的原因                                             |
  | stval   | 给出 Trap 附加信息                                           |
  | stvec   | 控制 Trap 处理代码的入口地址                                 |

- 硬件完成的事情还是很多的，感觉除了设定trap的入口还有参数传递啥的其他都做了，比如保存状态、保存断点、跳转啥的。这么看trap就好像一个中断（根据下面的关于stvec寄存器的介绍，说道stvec存储的是一个中断服务程序的入口地址，所以trap就是一个中断）。这个stvec寄存器只需要62位（在当前的CPU架构中一个地址应该是64位）来保存一个地址，是因为指令需要32位对齐（甚至可能是64位），所以低两位恒为0。需要注意的是，就像中断一样，每次trap都要把sepc（注意不是ra，也就是不是x1，ra是用来处理普通的函数调用的，而sepc是专门用来处理中断的，就跟arm处理器中断模式和用户模式下有的寄存器不一样差不多）还有sstatus保存，因为trap可能是嵌套的，如果没有保存的话trap嵌套时之前的返回地址就会丢失，这也是TrapContext结构体中有这两个成员的原因

- 在trap之前需要保存当前的执行现场，并且现场是保存在内核栈中的，用户栈里面就是保存一些基本的函数调用啥的。这个感觉就只是一种规范吧，应该用哪个栈都是可以的

- **疑问**

  > 两个栈以全局变量的形式实例化在批处理操作系统的 `.bss` 段中。

  为什么这两个栈会实例化在bss段？（静态变量的值是可变的，但是他的大小不是可变的，所以如果没有指定类型的话，静态变量是不能被声明的）

  另外，bss段还有data段都是在编译的时候就确定好了的，所以不会出现移动的问题。结构体本身的**定义**最终并不会在elf文件中

- 在TrapContext结构体中需要保存所有的寄存器，原因如下：

  > 对于通用寄存器而言，两条控制流（应用程序控制流和内核控制流）运行在不同的特权级，所属的软件也可能由不同的编程语言编写，虽然在 Trap 控制流中只是会执行 Trap 处理 相关的代码，但依然可能直接或间接调用很多模块，因此很难甚至不可能找出哪些寄存器无需保存。既然如此我们就只能全部保存了。

- tp（x4）是线程指针的意思

  > 它通常用于存储当前线程的本地存储（Thread Local Storage，TLS）的地址。

- 关于汇编和rust的交互

  内联汇编：

  ```rust
  core::arch::asm!("fence.i");
  ```

  直接在当前位置插入一个汇编文件

  ```rust
  global_asm!(include_str!("trap.S"));
  ```


# 5.3

接着看ch2

## Guide2——ch2续

- sscratch寄存器相当于是一个陷入时的备份栈指针（跟arm中断中的寄存器备份差不多）。

  ```assembly
  csrrw sp, sscratch, sp
  ```

  这一条指令的作用就是交换sp和sscratch的值，实现用户栈和核心栈的转换

- #![deny(missing_docs)]这个注释的存在使得所有的公有项都需要注释（注意是文档注释，就是///），否则就会报错

- 测试了一下之前的问题，就是为什么前面说KERNEL_STACK是在操作系统的bss段，但是实际测试出来好像不是这样的，因为内核的bss段如下：

  ![image-20240503094801207](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240503094801207.png)

  但是我在内核的主函数中直接打印出内核栈的栈顶的时候出现：

  ![image-20240503095644437](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240503095644437.png)

  内核栈好像是在。。。rodata段？这显然不合理吧（因为内核栈的空间中一定会进行写操作**疑问**）

  ![image-20240503101337918](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240503101337918.png)

  接下来继续查看用户栈的内存地址：

  ![image-20240503100139723](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240503100139723.png)

  可以发现用户栈是紧接着内核栈的，也是在rodata段中。这样的话在bss段中开出来的栈区都没用了

- 为什么这个函数不需要增加文档注释**疑问**：

  ![image-20240503100756771](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240503100756771.png)

- riscv的栈也是满递减的

- 下面这段汇编指令挺有意思的：

  ```assembly
      .set n, 5
      .rept 27
          SAVE_GP %n
          .set n, n+1
      .endr
  ```

  > 这是一段 RISC-V 汇编语言的代码，使用了 GNU 汇编器的预处理器指令。
  >
  > .set n, 5：这条指令设置了一个名为 n 的符号，初始值为 5。
  >
  > .rept 27：这条指令开始了一个重复块，它的内容会被重复 27 次。
  >
  > SAVE_GP %n：这是一个宏调用，它会被替换为一些实际的汇编代码。%n 是一个参数，它的值是符号 n 的当前值。
  >
  > .set n, n+1：这条指令将符号 n 的值加 1。
  >
  > .endr：这条指令结束了重复块。
  >
  > 所以，这段代码的作用是重复执行 27 次 SAVE_GP 宏，每次执行时，参数 n 的值都会加 1。初始时，n 的值为 5，所以第一次执行 SAVE_GP 宏时，n 的值为 5，第二次执行时，n 的值为 6，以此类推，直到 n 的值为 31。

  这里面用到了汇编宏（这样就要预处理了，所以trap的文件后缀是S）

  ```assembly
  .macro SAVE_GP n
      sd x\n, \n*8(sp)
  .endm
  .macro LOAD_GP n
      ld x\n, \n*8(sp)
  .endm
  ```

- trapcontext结构体的结构影响了保护上下文时的寄存器保存顺序：

  ```rust
  pub struct TrapContext {
      /// general regs[0..31]
      pub x: [usize; 32],
      /// CSR sstatus      
      pub sstatus: Sstatus,
      /// CSR sepc
      pub sepc: usize,
  }
  ```

  由于这个结构体是一个整体，所以压栈的时候就是低地址（x1）放在栈空间的低地址

- t0、t1、t2是x5、x6、x7的别名（riscv好多寄存器都有别名）

- 在保存trap上下文的过程中最后需要将内核栈的指针放在a0中以将其作为参数传递，能够让处理函数获取内核栈的指针，这样的话硬件就能直接去获取trap的上下文了（user调用syscall的时候传递的参数在保存trap上下文的时候已经到内核栈中了）

- 这个时候再回去看user里面系统调用的代码就很通透了：

  ```rust
          core::arch::asm!(
              "ecall",
              inlateout("x10") args[0] => ret,
              in("x11") args[1],
              in("x12") args[2],
              in("x17") id
          );
  ```

  这里就是通过ecall指令进行一个trap，由于在os中已经将trap的入口寄存器规定为我们自己编写的trap上下文保护的部分了，所以这个时候就会跳到该上下文保护函数中执行。后面的inlateout、in等部分实际上是在ecall指令之前执行的，这些寄存器存放的位置都要符合规范（因为如果不规范的话，在trap跳转的时候，trap_handler函数只知道内核栈的栈指针，不符合规范的话就会从内核栈中取出错误的数据）

- trap_handler是自己写的，所以说硬件在这个过程中只根据陷入指令以及返回指令提供了一些寄存器的操作（比如保存返回地址什么的）并且进行了特权级的切换，这样系统才能进行系统调用。上下文的保护、上下文的恢复、trap处理都是要操作系统来实现的。所以这个trap就好像是一个超大号的任务切换

- 需要注意，trap上下文保护的时候栈中存储的栈指针是trap之前的栈指针（在trap入口的第一行规定），所以在出栈的时候由于需要使用到内核栈指针，所以栈指针不能很快恢复，所以取出栈指针的时候就只能放在栈指针备份中（实际上就是原来在保存上下文中的栈指针备份，从哪里读就恢复到哪里，很合理吧）。同样的因为trap可能嵌套，所以栈指针也是需要保存以便之后恢复的。并且在trap处理函数返回的时候，返回值为trap上下文的首地址，所以在trap处理函数执行结束之后，a0的值就将变为保存trap上下文之后的内核栈指针，并在在恢复上下文的过程中返回值a0是在trap处理函数中直接通过栈指针来操作的，所以在恢复的时候就顺便恢复到约定的a0中了

- sepc寄存器存储的是trap之前最后一条指令的地址，所以如果要返回成下一条指令的地址的话就要在trap处理中对其+4

- 这里也解答了之前的问题，就是为什么系统没有循环去遍历app数组就能自动调度，是因为：

  ```rust
  pub fn sys_exit(exit_code: i32) -> ! {
      trace!("[kernel] Application exited with code {}", exit_code);
      run_next_app()
  }
  ```

  这个系统调用在每一个用户程序退出的时候都会被执行，也就会调用下一个用户程序了

- 应该是SBI在执行的过程中就是内核态的，然后跳转到操作系统的部分也就是内核态的了

- 通过复用__restore实现：

  - 跳转到应用程序入口点 `0x80400000`；
  - 将使用的栈切换到用户栈；
  - 在 `__alltraps` 时我们要求 `sscratch` 指向内核栈，这个也需要在此时完成；
  - 从 S 特权级切换到 U 特权级。

  这个实际上就很像嵌入式操作系统课上讲的模拟压栈。在复用__restore之前需要自己模拟压入一个trap上下文，这个上下文中的返回地址为0x80400000，sscratch为用户栈，这样在使用\_\_restore的时候就能直接跳转到0x80400000，栈将变为用户栈，sscratch将变为内核栈，并且特权级将下降

  而这个函数是在run_next_app中调用的，执行run_next_app的时候一定是内核态，因为只有当批处理操作系统初始化完成（此时一定是内核态），或者是某个应用程序运行结束或出错（这个时候一定会调用系统调用中的退出或者错误处理而trap，就转换为系统态了）的时候才会执行run_next_app，所以这个在这个函数中一定要调用__restore进行模拟压栈恢复用户程序的运行状态，这不仅仅是特权级的问题，还有栈的转换，PC指针的恢复（需要将PC恢复为x80400000），以及其他寄存器的重置（实际上就是所有寄存器的恢复，包括通用寄存器以及状态寄存器）

- 做到这里感觉其实应用程序的地址也是可以自己规定的，只要AppManager搬运代码的时候将代码搬运到某个约定好的地址，然后在执行run_next_app中的__restore传参的时候将该地址传入就可以执行用户程序了。当然这只是一个很朴素的想法，当真正实践的时候是需要考虑各种硬件特性的，比如存储器的组织等

## Guide3——ch3

- ch3刚刚开始的话挺有意思的：

  > 在第二章中，内核让所有应用都共享同一个固定的起始地址。 正因如此，内存中同时最多只能驻留一个应用，

  在第二章中所有的用户程序都被烧录在0x80400000的位置，所以内存中只能驻留最后一个烧录的程序。但是在第二章中仍然能实现多道程序的处理是因为所有程序都经过link_app.s（在执行build.rs的时候自动生成）的时候都被硬编码进了操作系统的data段，并且通过appmanager在每次需要进行调度的时候将程序拷贝到app执行的位置（0x80400000）,所以还是能实现多道程序的功能，但是这让操作系统内核变得更庞大了

  继续往下看的话好像还是存在一个link_app.s汇编程序将所有的应用程序都直接编码进os的数据段。这样来看的话os的data段应该是所有应用程序的加载域（应用程序总是要找个地方存的，这个时候没有外存，所以这里就先放在os的数据段中了），然后在py脚本中指定的地址应该是应用程序的执行域。

  在ch2中的loadapp函数只会加载一个任务，每次执行runapp的时候都需要load一个app；而在ch3中所有的app都在一个loadapp函数中直接加载到用户程序对应的运行域（通过循环实现）。现在需要理解一下实验的逻辑，由于qemu中没有外存，所以这里就认为os的数据段是外存，而前面指定的0x80400000应该就算是内存了。但实际上应用程序并没有直接被烧在他的运行域（因为在qemu中没有发现烧录用户程序的地方），所以将用户程序的代码加载到qemu上的过程是由link_app实现的

  这么想的话在连接脚本中**指定的应该是运行域**，这个就要分两种情况了

  - 程序直接被烧录：这个时候连接脚本指定的地址就应该指定的是程序被**烧录的位置**了

  - 程序没有被直接烧录：这个时候在连接脚本中指定的地址好像没有起到什么实际的作用，因为在os内核中是重新计算了所有用户程序的运行域基地址的。而在连接脚本中提供的地址就好像提供一种约定一样，**表示的是连接器期望这些程序在某些位置被执行**。另外，在py脚本中指定的各个应用程序运行域的基地址实际上就是链接脚本中的基地址了（这是因为py脚本的执行在链接脚本之后，就会使得py脚本中指定的app运行域基地址会覆盖链接脚本中指定的基地址）。在Guide也提到了这一点：

    > 我们可以在 `config` 子模块中找到这两个常数， `APP_BASE_ADDRESS` 被设置为 `0x80400000` ， 而 `APP_SIZE_LIMIT` 和上一章一样被设置为 `0x20000` 。这种放置方式与 `user/build.py` 的实现一致

    这里就是说了链接器指定的加载域的基地址和os中指定的加载域地址应该保持一致。但实际上我应该可以完全不管链接器指定的运行域（如果执行程序之前硬件不会去检查当前地址是不是当前文件中指定的运行域地址的话），而直接在os内核中指定运行域的地址并在load的时候将用户程序加载到该地址上，然后在restore中去指定该地址应该也是能执行应用程序的（如果硬件不拦我的话）

- 有一句话：

  > 要一次加载运行多个程序，就要求每个用户程序被内核加载到内存中的起始地址都不同。 为此，我们编写脚本 `user/build.py` 为每个应用定制各自的起始地址。 它的思路很简单，对于每一个应用程序，使用 `cargo rustc` 单独编译， 用 `-Clink-args=-Ttext=xxxx` 选项指定链接时 .text 段的地址为 `0x80400000 + app_id * 0x20000`

  在用户程序构建的py脚本中是有实现的（这也是昨天有疑问的地方）

# 5.4 

## Guide3——ch3续

调整一下策略，先做lab，不然真来不及了），笔记的话之后再来看文档补

- 在TCB中增加一个成员sche_time，表示当前任务接受调度的时间，这样的话就可以在TaskManager进行调度的时候直接记录任务被调度的时间了

- 在TCB中增加一个成员syscalltimes，类似于taskinfo。实在想不到什么好办法能在异常处理之后但是具体系统调用之前维护系统调用次数。我大概想了四种方法，但是感觉可行的只有这一个）

  - 在syscall函数中定义一个临时变量，直到调用sys_task_info系统调用的时候再进行累加。但是如果定义一个局部变量的话每次系统调用都会重置变量，用不了一点
  - 增加一个全局变量。这样虽然可以避免变量被重置，但是会导致所有任务的系统调用都会被记录在同一个地方。
  - 增加传参（或者在其他的系统调用前增加一个sys_task_info调用）。在调用syscall的时候增加一个taskinfo的传参，但是这样好像就要修改测试代码了（因为调用的时候需要多传一个参数就要修改user那边的syscall了，然后内核这边也不好修改，还要去修改异常处理函数）
  - 第四种就是直接在TCB中增加成员了，直到当前任务调用sys_task_info的时候进行累加，这样的话每个任务都是独立的，而且能够持久记录

  增加了这个成员之后只需要在执行info系统调用的时候直接将当前tcb中的系统调用数组直接复制给taskinfo的成员即可
  
  但是在TCB增加了一个成员之后还需要在taskmanager中增加一些函数，比如获取当前任务的TCB等函数，以便在执行系统调用的时候修改TCB。

- 关于调试，这里使用的是gdb-multiarch，没有使用手册上说的那个gdb，用那个老是会出问题。并且要进行调试的时候需要将Makefile里面的mode改成debug，如果是release的话在构建项目时生成的文件很多调试信息都被删除了，所以需要使用debug

- 在os内核中调用的get_time返回的时间单位是时钟周期，因为对频率不是很了解，所以这里选择使用已经封装好了的get_time_us函数，返回的是微妙。但是在测试中使用的是毫秒，所以需要将微妙转换为毫秒存储在taskinfo中

- 对于inner的理解：

  每次调用完获取inner的方法之后都需要直接drop（在函数结束的时候会自动drop）。不能在一个inner还有效的时候再获取一个inner。因为这是借用规则不允许的。因为taskmanager中的inner每次最多只能有一个可变引用，如果直接从taskmanager中进行返回就相当于创建了一个inner可变引用，如果没有drop的话就会使得这个可变引用一直存在，直到下一次想要再次获取inner的时候就就会出现panic。

- 对于TCB的理解：

  如果在函数中直接返回任务模块中的TCB的话，这样返回的时候发生所有权转移的时候会clone一个TCB，而不是他本身，因为在创建TCB的时候给他实现了copy和clone这两个trait，所以这个时候如果要修改原有的就只能使用裸指针。copy和clone trait：

  ```rust
  #[derive(Copy, Clone)]
  pub struct TaskControlBlock {
      /// The task status in it's lifecycle
      pub task_status: TaskStatus,
      /// The task context
      pub task_cx: TaskContext,
      /// The scheduled time of the task
      pub sche_time: usize,
      /// The current syscall type of the task
      pub syscall_times: [u32; MAX_SYSCALL_NUM],
  }
  ```

  所以如果没有使用裸指针的话，每次调用gettcb都会将当前任务的tcb中的数组copy一份，但是并没有真正记录到tcb中，也就会出现第一个测试get_time系统调用次数都过不了的情况

- pub关键字要好好看看了，没有编译器提示我根本搞不明白什么地方能用哪些函数

  现在搞明白了，就是之前理解的pub，只不过一个imp不算是一个项，因为pub关键字没办法使用在imp上，就证明了其不是一个项。所以这个时候imp内部的函数应该就是当前crate中的最高级别的项，就使得其他同级别的项能直接访问这些函数

- info记录的就是任务第一次被调度到调用sys_task_info的时间。之前一直错是因为我在每一次任务被调度的时候都重新设置了任务的调度时间，这样任务的调度时间就不是他第一次被调度的时间了。所以修改之后使用一个Option来设置任务第一次被调度的时间，如果这个Option为空的话就设置，表示当前任务第一次被调度；如果不空的话就说明当前任务之前已经被调度过了，这个时候就需要再进行赋值了：

  ```rust
  if task0.sche_time.is_none(){
              task0.sche_time=Some(get_time_us());
  }
  ```

## Guide4——ch4

- 下面这个图应该比较重要：

  ![image-20240504170352313](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240504170352313.png)

  这个就涉及到实验需要的虚存和实际内存的转换了

- 关于虚拟内存的作用

  - 内存抽象：虚拟内存允许每个进程都认为它拥有全部的地址空间，这简化了内存管理和编程。
  - 内存保护：每个进程在其自己的地址空间中运行，这防止了一个进程意外或恶意地修改另一个进程的数据。
  - 内存共享：虚拟内存允许多个进程共享同一份物理内存，例如共享库或者进程间通信。
  - 内存映射文件：虚拟内存允许文件被映射到进程的地址空间，使得进程可以像访问普通内存一样访问这些文件。
  - 按需分页：虚拟内存允许内存被按需分页到磁盘，使得实际的物理内存使用更加高效。

- 实验要求写一个函数sys_mmap分配长度为len的空间，并且指定了虚拟地址为start。本质上就是通过StackFrameAllocator来分配若干个物理页以满足len，然后在页表中增加start虚拟地址和分配的实际物理地址的对应关系即可

- 实验要求写一个函数sys_munmap将虚拟地址对应的物理空间释放，实际上就是根据虚拟地址查询页表获取物理地址，然后调用StackFrameAllocator提供的释放函数释放len对应的页表长度。甚至虚拟内存的销毁都可以让drop函数直接实现

  ```rust
  impl Drop for FrameTracker {
      fn drop(&mut self) {
          frame_dealloc(self.ppn);
      }
  }
  ```

- 对于多级页表，首先需要知道，页表也是存储在内存中的，而多级页表实际上就是按照分页机制来存储页表，最高级的页表占用一个物理页，物理页中存储的是512个页表项（可以根据9位地址寻址，所以取虚拟地址的时候是9位9位取的。这里也能理解什么叫恒等映射了，也就是一个虚拟地址的虚拟页号直接对应多级页表中的一个表项）。这里面每个页表项中指示的物理页号就是下一级页表的位置，所以下一级页表应该是有512个的 

- 这里有一段话：

  > - `VirtPageNum` 的 `indexes` 可以取出虚拟页号的三级页索引，并按照从高到低的顺序返回。注意它里面包裹的 usize 可能有 27 位，也有可能有 64−12=52 位，但这里我们是用来在多级页表上进行遍历，因此 只取出低 27 位。

  52位是因为usize是64位的，对虚拟地址而言低12位用于标记offset，接下来的27位就是虚拟页号。然后高位可能是1（因为高位必须和38位相同），也可能是0。因为我们这里只是要去取出虚拟页号，所以只需要取27位

- 这里还有一段话：

  > 在寻找页表项的时候，可能出现页表的中间级节点还未被创建的情况，这个时候我们需要手动分配一个物理页帧来存放这个节点， 并将这个节点接入到当前的多级页表的某级中。

  这个意思是，我正在通过一个虚拟页号查找多级页表（三级），可能我查询到第二级的时候没有这个表项了，这个时候我就要新建一个表项来指向第三级的页表

- 在创建页表的时候，是直接分配一个物理页，这个时候就会选取一个没有使用的物理页来存储当前用户进程的页表，并且在初始化的时候会对这个页表进行清零。然后如果当前用户进程进行虚拟地址查询的时候就会查询当前这个页表，第一次查询的时候查询到的页表项一定是0，就会使得页表在当前的基础上再继续申请物理页来存储下一级的页表，并且同样的，由于是重新申请一个物理页，所以会进行清零操作。从这里也能看出页表一定是不会跟用户程序的代码或者数据之间冲突的，因为每次都是直接重新申请一个物理页，这个物理页一定是没有被使用的。并且所有用户程序的页表都存储在内存中（因为是多道程序）

- 采用的是恒等映射还是什么映射感觉是由硬件决定的，就是要看MMU是怎么查询的。如果MMU是采用的恒等映射的话，那就只能像Guide上面说的去做了。这里顺便再说一下MMU的作用，实际上就是根据虚拟地址查找页表项然后获取物理页号进而获取物理地址，所以操作系统需要做的就是根据虚拟地址中提供的页号，去根据MMU的映射规则去初始化页表项就好了

- 与上一个实验一样，页表也是每一个任务特有的（跟上一个实验中的每个任务的系统调用次数差不多），所以在任务的TCB中增加一个页表成员

- 太阴间了，在mm这个crate下面的mod中有下面这段代码：

  ```rust
  use page_table::{PTEFlags, PageTable};
  ```

  这个导致了一直在task中无法引入页表，需要在这一项前面加上一个pub。关于mod.rs的作用如下：

  > 当你在其他地方使用use crate::mm::...时，Rust编译器会查找mm文件夹下的mod.rs文件，根据其中的定义来解析你的use语句

  相当于是这个mod.rs文件就管理了当前文件夹下的所有的文件

  这个可以再去复习一下rust**疑问**


# 5.5

## Guide4——ch4续

- PTEFlags的from_bits方法通常是由bitflags宏自动实现的。并且这个类型只能跟自己这个类型进行位运算，不能直接跟u8什么的进行位运算。并且创建页表项的时候是不需要去管V这一位，因为在map函数中已经对该位置一了（中间级的多级页表节点在查询过程中已经创建了）
- 在sys_munmap函数中不用考虑存在没有被映射的虚拟内存地址，因为在提供的unmap函数中已经检查了根据提供的虚拟内存查询到的页表项，并且在函数内部判断了当前页表项是否有效
- 每一个程序被执行的时候页表会重新加载吗。copilot给出的答案是会重新加载，并且在上下文切换的时候将当前任务的页表装载进MMU。所以应该是所有的用户程序的页表都是直接存储在内存中的（因为是多道程序），所以不会出现重新从外存中下载页表的情况
- 虚存应该只会在使用裸指针的时候产生影响，因为如果使用虚存的话，对一个变量取值的时候，取出来的就是虚拟地址。如果是直接使用一个变量的话，MMU就会直接通过页表来获取真实的值
- 启用虚拟地址的时候，一个变量的虚拟地址一定是连续的，但是物理地址不一定（因为虚拟地址连续只能代表两个页表项是连续的，并不能代表页表项中的物理页号也是连续的）
- 由于现在在猛赶实验，所以在做ch4的时候还以为load函数还是和task放在一起的，原来已经单独占一个文件了。之后要好好看看代码是怎么load用户程序的
- core::slice::from_raw_parts函数我传入的是什么类型的指针，返回的切片中的元素就是什么类型的
- md进行make run的时候只会执行一些用例（也就是不需要修改就能成功执行的用例），所以在make的时候要制定一下BASE
- 不能在TCB中新建一个页表，因为页表的基地址是不能自己决定的，只能由satp指定，按照自己创建的页表找当然不行了（因为MMU不是按照我的页表找的）。在TCB中已经实现了一个MemorySet，这里面有一个页表，这个页表就是mmu使用的页表
- 物理页号转换为usize返回的只是页号，如果要转换为物理地址的话就要左移12位
- 有点阴间，提供的map函数中的断言好像是反过来的，map应该判断之前这个页表项有没有已经被映射，所以断言的应该是pte.is_valid()

# 5.6

- from函数不是把一个虚拟地址转换为虚拟页号，而是把一个虚拟页号（usize）转换为一个虚拟页号（vpn）

- 在map和unmap中遍历虚拟页号的时候每次都要检查一下当前的虚拟页号是不是合法的（map的页号有没有被映射过，unmap的页号有没有映射）。只有当所有的虚拟页号都通过检查时map和unmap才能返回成功

- 有一句话：

  > 然而，当以 `Framed` 映射的时候，不要忘记同时将虚拟页面被映射到的物理页帧 `FrameTracker` 从 `data_frames` 中移除，这样这个物理页帧才能立即被回收以备后续分配。

  这句话变相表明需要将被分配的物理页帧放在逻辑段下面，这样FrameTracker才会一直被引用，这样物理帧才不会被回收（如果自己分配的话并且不放在逻辑段中时，那么当前作用域结束物理帧就会被回收，映射就马上失效了）

- 直接调用memory_set的append_to方法其实就好了。这个函数首先会先找到当前虚地址对应的逻辑段（根据第一个参数），然后扩展该逻辑段（按照第二个参数扩展，将逻辑段的结尾扩展至第二个参数）。在扩展的过程中，会直接对扩展出来的虚拟内存进行初始化（用过逻辑段的map_one方法），这个时候就会将物理帧直接挂载在当前逻辑段的data_frames下了

- 感觉题目的意思是要让我重新创建一个maparea，这样才需要给我提供权限信息。这样的话如果start提供的虚地址已经在已有的maparea中就一定是错的。这个时候需要先insert一个maparea（insert的时候是没有分配内存的）。然后可以借助append_to函数定位到该area然后对这段内存进行初始化。对于unmap，就可以采用shrink方法直接将整个area都shrink掉

- md编译器优化等级如果调整了的话有的函数都跑不进去。我把优化等级调成0就导致syscall_task_info函数进不去了

## Guide5——ch5

- 一个任务创建的过程应该是：

  - 先根据elf文件生成一个TCB
  - 将TCB加入任务管理器
  - 开始调度

- 有一段代码：

  ```rust
  /// Get a copy of the current task
  pub fn current_task() -> Option<Arc<TaskControlBlock>> {
      PROCESSOR.exclusive_access().current()
  }
  ```

  current函数如下：

  ```rust
      pub fn current(&self) -> Option<Arc<TaskControlBlock>> {
          self.current.as_ref().map(Arc::clone)
      }
  ```

  这段代码是在获取当前任务的副本，就是再创建一个指向当前任务TCB的Arc指针（这不比裸指针好多了，直接完美替代我的获取当前TCB的裸指针方法）。**疑问**为什么能直接使用Arc。因为Arc虽然是在标准库中的，但是是核心库的一部分。在主函数中仍然使用了nostd，但是仍然能使用智能指针：

  > The Rust core allocation and collections library

  上面这段话是官方文档说的，就说明alloc应该是核心库的一部分，并且在主函数中引入了该模块

- 现在可变的东西需要放在TaskControlBlockInner中。这个结构体经过了UPSafeCell的包装，所以使用是有严格的借用规则要求的

# 5.7

## Guide5——ch5续

- 创建一个任务的时候，首先需要创建任务的TCB，然后需要将任务放到任务管理器的就绪队列中，这样任务就能正常执行了。使用fork+exec的时候也是这样，首先fork复制当前任务的TCB，然后将TCB加入到就绪队列中（所以如果想要一个完全一样的进程的话，其实直接fork就可以实现了）。然后执行exec来指定要执行的任务，这个时候就会对原有的TCB进行覆盖（此时的TCB已经在就绪队列中了），这个时候就能执行exec中指定的任务了。所以如果要实现spwan的话，其实只需要创建一个需要执行的任务的TCB，然后在将任务放到就绪队列中就好了，甚至不需要进行exec

# 5.8

## Guide5——ch5续

- stride算法会出现溢出是因为可能出现数的溢出。比如问答题中的例子。所以在我的实验中，按照Guide给出的建议，就选取一个适中的数0x8000，并且用usize的类型（由于usize最小可能是32位的，所以这里是32位数据表示范围的一半）

- idle并不在就绪队列中，所以进行run_tasks的时候是不会找到Idle的（也就是不会出现idle的上下文跟idle自己的上下文交换的情况，这种就是没有意义的性能开销了）。所以idle进程在rCore中就相当于是让机器在没有任务执行的时候不要闲着的进程（其实初始进程也是这样的，一旦开机初始进程就会一直运行）。但是在lab中的idle进程和初始进程好像是没有关系的，这是因为初始进程是在用户程序开始执行的时候才会一直转的（因为把初始进程写成了一个用户程序），而在操作系统还没有装载初始进程之前，idle进程就已经存在了，只不过是一种“假”的进程，是直接绑定在CPU上的。所以按道理来说，因为初始进程一直在就绪队列中，所以不会出现run_tasks中的else的情况。只不过在上下文切换的时候需要使用idle进程的上下文来进行过渡

- 可以发现如果某个任务占据了CPU，那么他就不在就绪队列中了。

- 每次执行任务的时候都需要修改任务的stride

- 从测试样例中可以看出，实际上设置优先级的系统调用就是判断输入的参数是否合法然后直接返回传入的参数

  ```rust
  pub fn main() -> i32 {
      assert_eq!(set_priority(10), 10);
      assert_eq!(set_priority(isize::MAX), isize::MAX);
      assert_eq!(set_priority(0), -1);
      assert_eq!(set_priority(1), -1);
      assert_eq!(set_priority(-10), -1);
      println!("Test set_priority OK!");
      0
  }
  ```

- spwan返回的应该当前进程的pid，而不是0。并且所有swpan出来的进程都应该是当前进程的子进程

- 因为要维护一个进程之间的关系，还需要维护一个就绪队列（本质上都是需要TCB的）。这种的拷贝是必须的，所以这里需要一直clone TCB的Arc指针，使得拷贝的时候不用拷贝TCB，而是拷贝Arc指针，这样开销就很小了

- wait函数只能是当前进程等待其子进程，所以实现的时候会去遍历子进程队列

- 为了debuug方便（不知道为什么打开gdb终端之后就没办法再向qemu终端中输入信息了），这里就先将initproc替换为usertest了：

  ![image-20240508121852883](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240508121852883.png)
  
- 在使用current_task的时候就是在进行一次Arc指针的clone，这个时候指针的计数就会增加了，但是如果像下图一样使用current_task：

  ![image-20240508214424659](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240508214424659.png)

  就不会出现计数加一的情况，因为这里并没有将clone出来的对象绑定在某个变量上，所以在这行代码结束的时候就会被drop

# 5.9

## Guide6——ch6

- 有一段代码：

  ```rust
  pub trait File : Send + Sync {
      fn readable(&self) -> bool;
      fn writable(&self) -> bool;
      fn read(&self, buf: UserBuffer) -> usize;
      fn write(&self, buf: UserBuffer) -> usize;
  }
  ```

  这里冒号后面表示的是File继承了Send + Sync

- 在ch6中就不能直接get_app_data_by_name然后根据名称来创建一个用户程序的TCB了（之后源代码看细一点，看看load是怎么做的），而是需要像下面这样先打开文件然后再去读取用户程序的信息：

  ```rust
  pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new({
          let inode = open_file("ch6b_initproc", OpenFlags::RDONLY).unwrap();
          let v = inode.read_all();
          TaskControlBlock::new(v.as_slice())
      });
  ```

- git cherry-pick指令需要传递一个git的日志信息（相当于是某次commit的编号），比如说如果ch6中需要拷贝ch5的代码，就可以在ch6的目录下使用git cherry-pick commit号来进行合并。commit号可以在ch5的目录下使用git log来查看所有commit的编号

- TCB中增加了一段代码：

  ```rust
  pub fd_table: Vec<Option<Arc<dyn File + Send + Sync>>>,
  ```

  这里最内层使用了dyn关键字，作用如下：

  > `dyn` 关键字表明 `Arc` 里面的类型实现了 `File/Send/Sync` 三个 Trait ，但是编译期无法知道它具体是哪个类型（可能是任何实现了 `File` Trait 的类型如 `Stdin/Stdout` ，故而它所占的空间大小自然也无法确定），需要等到运行时才能知道它的具体类型。

  其实就是使用了trait bound语法，这个时候接受的类型可以是File，也可以是Send，还可以是Sync（这是一种运行时多态。编译时多态就是泛型）

- 在创建一个TCB的时候已经为TCB打开了标准输入输出文件（落实到代码层面就是将文件描述符放到当前TCB的文件描述符表上）

- 实现系统调用中的read和write的时候就可以向指定的文件（通过文件描述符指定）来进行读写了。之前的系统调用其实都没啥用，因为都是标准输入输出，这个时候就是直接调用SBI提供的接口来直接将信息输入到终端上。而在现在如果想要向标准输入输出中读写数据，这个时候就要指定标准输入输出的文件描述符（虽然之前也指定了，但是之前的功能是只能在标准输入输出中读写，所以那个时候传的文件描述符就好像没用了）

- 打开文件的时候会返回文件的文件描述符（一个整数）

- 打开一个文件在现在的rCore中就相当于是一个外存上文件的抽象出来的树结构，如果要创建一个文件其实就是向这个树结构中加入一个节点（节点包含文件的所有信息，所以应该是一个u8的切片）。至于像Windows一样的可视化其实只是在遍历当前的文件抽象树结构并打印罢了

- 这个感觉挺重要的，能让我对文件系统有一个系统的认识：

  - 磁盘块设备接口层：以块为单位对磁盘块设备进行读写的 trait 接口
  - 块缓存层：在内存中缓存磁盘块的数据，避免频繁读写磁盘
  - 磁盘数据结构层：磁盘上的超级块、位图、索引节点、数据块、目录项等核心数据结构和相关处理
  - 磁盘块管理器层：合并了上述核心数据结构和磁盘布局所形成的磁盘文件系统数据结构
  - 索引节点层：管理索引节点，实现了文件创建/文件打开/文件读写等成员函数

  如下图：

  ![image-20240509113438471](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240509113438471.png)

- 实际跟磁盘打交道的应该是块设备接口，也就是下面提供的这两个函数：

  ```rust
  pub trait BlockDevice : Send + Sync + Any {
      fn read_block(&self, block_id: usize, buf: &mut [u8]);
      fn write_block(&self, block_id: usize, buf: &[u8]);
  }
  ```

  这两个函数的实现必然涉及硬件磁盘的操作

- 下面有一段代码：

  ```rust
     pub fn get_ref<T>(&self, offset: usize) -> &T where T: Sized {
          let type_size = core::mem::size_of::<T>();
          assert!(offset + type_size <= BLOCK_SZ);
          let addr = self.addr_of_offset(offset);
          unsafe { &*(addr as *const T) }
      }
  ```

  并不是所有泛型都能直接使用core::mem::size_of，只有实现了Sized这个trait的类型才能使用（实现了Sized表示当前类型能在编译时确定大小）

- 其实在cache的时候就已经将数据读取到内存的缓冲区中了

- 泛型的静态分发功能是什么**疑问**

- 下面这段话是什么意思：

  > 这里我们传入闭包的类型为 `FnOnce` ，这是因为闭包里面的变量被捕获的方式涵盖了不可变引用/可变引用/和 move 三种可能性，故而我们需要选取范围最广的 `FnOnce` 。参数中的 `impl` 关键字体现了一种类似泛型的静态分发功能。

- modified 标记将会决定数据是否需要写回磁盘，所以在修改缓冲区的时候需要将modified 置为true

- 块缓存其实就是将磁盘上的一个块读取到内存中（按道理来说块的大小应该是能自己设置的，只不过这个时候需要考虑硬件，并且块缓冲区如果小的话，就只能让块编号变多一点了）。这样处理的话一个块缓冲实际上就是相当于是磁盘上一个块的拷贝，这个时候在管理的时候就只需要维护块号还有块缓冲了

- 这是一个经典组合（Arc加上Mutex。这个Mutex应该也是一个智能指针）：

  ```rust
  pub struct BlockCacheManager {
      queue: VecDeque<(usize, Arc<Mutex<BlockCache>>)>,
  }
  ```

  > 队列 `queue` 维护块编号和块缓存的二元组。块缓存的类型是一个 `Arc<Mutex<BlockCache>>` ，这是 Rust 中的经典组合，它可以同时提供共享引用和互斥访问。这里的共享引用意义在于块缓存既需要在管理器 `BlockCacheManager` 保留一个引用，还需要将引用返回给块缓存的请求者。而互斥访问在单核上的意义在于提供内部可变性通过编译，在多核环境下则可以帮助我们避免可能的并发冲突。

  块缓冲区是一个共享资源（多个进程可能要读取同一块磁盘的内容，更具体的来说，多个线程可能同时需要对一个文件进行修改）

  **疑问**这里说的对单核的意义是什么

- 通过Arc是无法直接修改变量的值的，他只是提供一个线程安全的引用计数（引用既然多了就一定不是可变引用。因为如果是可变引用的话就一定不符合借用规则）

- 但是通过上面说的那个经典组合，我们可以使用Mutex的lock方法提供互斥的访问。lock方法返回的是

- 下面是缓存替换的规则：

  > 第 13 行对应找不到的情况，此时必须将块从磁盘读入内存中的缓冲区。读取前需要判断已保存的块数量是否达到了上限。是，则执行缓存替换算法，替换的标准是其强引用计数 \eq1 ，即除了块缓存管理器保留的一份副本之外，在外面没有副本正在使用。

  就是当缓存的数量满了的时候（一定是有上限的，毕竟外存一定比内存大），寻找一个能被替换的缓冲区。一个能被替换的缓冲区，一定是只有在当前的缓冲区管理器中才有一个引用的（很像回收僵尸进程的时候的操作，这个强计数真好用吧）

- easy-fs的磁盘布局看着就像是一个虚拟内存的寻址过程一样。首先先有一个超级块（类似虚拟内存中的多级页表的根页表），然后通过一个位图来构建出了索引节点块（就相当于是多级页表中的所有的页表），最后再通过索引找到实际的数据区（相当于是虚拟内存中的物理地址）

- 下面有一段话：

  > 为了尽可能节约空间，在进行索引的时候，块的编号用一个 `u32` 存储。索引方式分成直接索引和间接索引两种：
  >
  > - 当文件很小的时候，只需用到直接索引， `direct` 数组中最多可以指向 `INODE_DIRECT_COUNT` 个数据块，当取值为 28 的时候，通过直接索引可以找到 14KiB 的内容。
  > - 当文件比较大的时候，不仅直接索引的 `direct` 数组装满，还需要用到一级间接索引 `indirect1` 。它指向一个一级索引块，这个块也位于磁盘布局的数据块区域中。这个一级索引块中的每个 `u32` 都用来指向数据块区域中一个保存该文件内容的数据块，因此，最多能够索引 5124=128 个数据块，对应 64KiB 的内容。
  > - 当文件大小超过直接索引和一级索引支持的容量上限 78KiB 的时候，就需要用到二级间接索引 `indirect2` 。它指向一个位于数据块区域中的二级索引块。二级索引块中的每个 `u32` 指向一个不同的一级索引块，这些一级索引块也位于数据块区域中。因此，通过二级间接索引最多能够索引 128×64KiB=8MiB 的内容。

  这里当文件比较大的时候实际上就已经像页表一样了，这个时候在磁盘上就有一个块（也就是上面所说的一级索引块，位于数据块区域中），这个块的所有4096个字节都用于存储数据块的索引（这个块的索引就是在DiskInode 中提供的indirect1）。而二级索引就相当于是一个二级页表，这里就不在赘述了（这个块的索引就是indirect2）

- `get_block_id` 方法体现了 `DiskInode` 最重要的数据块索引功能，它可以从索引中查到它自身用于保存文件内容的第 `block_id` 个数据块的块编号，这样后续才能对这个数据块进行访问。这个方法就是传入一个文件中的逻辑编号（也就是inner_id，相当于是索引编号），然后返回该数据块的物理编号

- 需要注意的是，这里所有的东西都是存储在内存中的，文件的真实信息都是存储磁盘上的，所以这里说的什么超级块、索引块、数据块其实都只是磁盘上文件的抽象。真正跟磁盘硬件相关的应该只有BlockDevice 

- 应该是一个文件或者目录就是一个diskInode（所以这个数据结构下有一个type_字段），然后超级块对应的实体应该就是整个文件系统，他的下面挂载着所有的文件以及目录

- 整体的逻辑应该是，首先是一层物理设备的抽象，然后在这个抽象上实现一个cache（这个cache还提供了一些能直接供我使用的函数，也就是那个要传入闭包的函数），然后就是在这个cache的基础上再创建了一个cache管理器，提供一些关于cache的操作（如根据一个磁盘物理块创建一个cache等）然后磁盘文件在内存中的抽象（DiskInode）通过调用cache管理器提供的方法来实现从磁盘中将文件取出并进行读写的操作（DiskInode这个结构还提供了将索引块号转换为物理块号的方法。这个就是针对一个文件而言的了）。然后再使用efs来将所有的DiskInode都管理起来，实现对整个磁盘区域的访问（efs也提供了获取物理块号的方法，但是这个时候就不是局限在一个文件中的了），以及创建文件（分配一个DiskInode索引节点以及相关的数据节点，并且返回inode的id），删除文件（收回相关的索引节点以及数据）。所以所有的DiskInode应该都是由efs进行管理的，而DiskInode是磁盘上文件以及目录的抽象。然后efs暴露给使用者的就是inode，相当于一个inode就对应了磁盘上的一个DiskInode，用户只需要操作inode就能让efs直接去操作磁盘上的DiskInode。

- 如果一个DiskInode的类型是一个目录的话，那么通过索引查找到的就不是一个字节序列（文件在磁盘上就是一个字节序列，在内存中只需要存储其所占有的物理块号即可，这些物理块号由DiskInode维护），而是一个表示目录的特定结构，在内核中用DirEntry来表示。

- 有一段话

  > 在 `root_inode` 中，主要是在 `Inode::new` 的时候将传入的 `inode_id` 设置为 0 ，因为根目录对应于文件系统中第一个分配的 inode ，因此它的 `inode_id` 总会是 0 。同时在设计上，我们不会在 `Inode::new` 中尝试获取整个 `EasyFileSystem` 的锁来查询 inode 在块设备中的位置，而是在调用它之前预先查询并作为参数传过去。

  这里说的不会在Inode::new中直接获取整个 `EasyFileSystem` 的锁来直接alloc一个inode然后根据返回的编号直接进行相关的操作，而是在new之前先去alloc之后再去创建一个inode

- 关于位图：

  > 它将会返回分配的bit所在的位置，等同于索引节点/数据块的编号。

  所以调用位图的alloc方法实际上就是将位图的对应位置一，然后返回该块的编号。而在efs中，直接将inode的编号设置为其对应的DiskInode的编号。在位图进行alloc的时候只是对磁盘上的位图区进行操作（也就是将位图对应的位置1，然后返回该编号），而真正初始化对应编号的块需要调用相关块的初始化方法。而inode的creat方法就是对上面的这个过程进行一个组合。首先先调用efs的alloc方法在位图上先alloc一个，并获取id，然后根据id去查找相应的块并进行初始化（块的数量在efs创建的时候就已经计算好了，实际上就是位图的位数）。（所以我只需要操作inode就好了）

# 5.10

## Guide6——ch6续

- 一个目录在磁盘上是一个目录项的**序列**，每一个目录项都代表了这个目录下面的一个文件的信息，包含的信息就是文件的名称以及文件的inode编号。而在进行inode的create的时候不仅会在位图上分配当前inode，还会在磁盘上对该disknode进行初始化，还会去维护跟当前inode的目录项（是在目录的inode中调用的create方法，所以此时维护的就是目录的目录项，在其中加入当前创建的目录项）

- 有一句话

  > 具体实现比较简单，需要注意在 `DiskInode::write_at` 之前先调用 `increase_size` 对自身进行扩容

  inode的write方法是这样实现的，在里面就有调用`increase_size` 方法提前将需要写入的数据大小先扩充出来，然后再进行modify操作。

- 感觉在inode中提供的read_disk_inode中传递的函数闭包的&DiskInode类型的传参好像没有什么用，好吧其实好像有用，因为在cache的read方法中是根据&DiskInode类型的传参来将磁盘上的一块数据解释为一个DiskNode（也就是将一个磁盘块解释为一个文件的索引块），这个时候才能调用DiskNode的read方法对磁盘上的某个文件或目录进行读写（因为一个文件是绑定到一个Disknode上的，并且一个inode是对应一个Disknode的），最终才能实现我直接调用inode的方法就能实现磁盘上文件的读写操作

- 跟之前想的一样，link_app其实就是将所有的用户程序都放在os内核的数据段中来模拟一个磁盘的，现在实现了文件系统就可以去将所有的用户程序都放在磁盘上了，然后通过BlockDevice提供的方法将需要执行的文件读取进内存

- 应该要去看一下文件系统是怎么将所有的elf文件都装载到磁盘上的，也就是函数easy_fs_pack。大概的逻辑就是通过命令行给出用户程序的路径，然后通过解析用户程序的大小将所有的用户程序交给一个efs来进行管理（也就是efs的初始化工作，然后将主机上的某些文件直接复制到我们的efs文件系统上，这个时候就将文件复制到我们的虚拟磁盘上了）

- 在下面还有一段代码

  ```rust
  pub struct OSInode {
      readable: bool,
      writable: bool,
      inner: UPSafeCell<OSInodeInner>,
  }
  
  pub struct OSInodeInner {
      offset: usize,
      inode: Arc<Inode>,
  }
  ```

  这里就是对inode进一步进行了封装，所以我只需要使用OSInode，然后去获取实例即可，他已经实现了File这个trait，所以只需要将这个结构体当成是文件的抽象即可了（每一个OSInode都有自己的文件描述符），他已经将inode进行了封装并且提供了一些我需要使用的接口。

- 好像没有根目录创建一个OSInode，所以进行根目录的相关操作（比如说需要创建一个文件）。这个时候就需要直接使用根目录的Inode了

- 所以处理一个文件的流程应该是：首先先打开一个文件（甚至不需要管这个文件有没有存在，只需要传对应的标志位就好了），然后对文件进行操作即可。所有关于文件的创建、打开文件是需要进行的操作等都是直接在openfile之中实现的。openfile之后就会将文件对应的OSInode返回，接下来就可以通过这个OSInode对文件进行读写操作了

- fstat的要求应该就是直接修改Stat中的一些值（由于函数传参的时候是使用Stat的引用，所以在函数内部中接收的时候是接收一个指针，这个指针是一个虚拟地址），然后如果修改成功的话返回0，修改失败就返回1

- 在每一次调用open的时候都会将当前文件的文件描述符放到当前进程的队列中

  ```rust
      pub fn alloc_fd(&mut self) -> usize {
          if let Some(fd) = (0..self.fd_table.len()).find(|fd| self.fd_table[*fd].is_none()) {
              fd
          } else {
              self.fd_table.push(None);
              self.fd_table.len() - 1
          }
      }
  ```

  这里可以看出，每一个进程的相同文件的文件描述符是不一样的（只是标准输入输出的文件描述符是一样的，因为标准输入输出并不是输入输出到一个文件上），**这里可以看出在一个进程中文件的文件描述符就是这个文件对应的OSinode位于fd_table中的位置**

- 文件的状态应该绑定到OSInode上，因为一个OSinode就对应了一个inode，然后一个inode对应了一个磁盘上的一个Diskinode，而一个Diskinode就对应了一个磁盘上真实的文件，所以一个OSinode实际上就对应了一个实际的文件，所以应该将文件的状态绑定到OSinode上，所以这里就在OSinode结构体中间增加一个成员代表文件的状态

- 下面阐述一下inode的成员都代表什么：

  ```rust
  pub struct Inode {
      block_id: usize,
      block_offset: usize,
      fs: Arc<Mutex<EasyFileSystem>>,
      block_device: Arc<dyn BlockDevice>,
  }
  ```

  block_id：表示的是实际的Diskinode所在的物理块ID

  block_offset：Diskinode在块上的偏移量

  fs：文件系统

  block_device：硬件设备抽象

- 由于系统是松散耦合的，所以在os文件夹中没有办法使用（最好不要使用）inode等

- 再理一遍逻辑。整个过程是：首先先分配位图（这个时候还没有一个实际的物理块），然后返回一个inodeID（所以在下中说inode是一个索引），然后根据这个ID去查找当前Inode对应的目录项序列（所以create方法只能由目录调用），并获取inodeID实际对应的DiskNode（此时才将inode和disknode对应上了，讲道理这个时候inode还没有new出来，但是Disknode是在文件系统初始化的时候已经通过计算分配了若干个索引块以及数据块的，这个是根据文件系统初始化的时候提供的参数决定的，初始化的时候有提供总块数以及inode位图的大小）。然后根据相关的信息对磁盘上的DiskNode进行初始化（这里只需要传入文件的参数），然后再根据DiskNode的信息对inode进行初始化，这个时候就已经建立起了inode和Disknode的联系，所以这个文件系统暴露给我的接口实际上就是inode索引

- 关于文件系统中函数闭包的理解：实际上传参（一般都是传一个引用）这里都是表示要将磁盘上的某个位置的数据解释为什么（比如说，如果传参是&mut DiskInode，就是将磁盘上的该位置的数据解释为一个可变的DiskInode）

- 由于Stat中需要保存当前inode的编号，所以只能在inode中再增加一个字段，表示当前的inode编号，因为本来是需要通过inode编号才能获取当前inode对应的物理块的编号和偏移量，进而才能对disknode进行操作，但是由于inode中已经存储了Blockid和blockoff，这样就不需要再通过inode编号来查找文件了，所以在原来的inode结构体中没有inode编号这个成员，inode也没有提供相应的方法，所以就只能在创建的时候将inode编号直接保存在inode结构体中了

- 需要注意的是，在我们实现的文件系统中，根目录的inode_id是0

- 在实现硬链接的时候需要修改文件状态中的硬链接数量（也就是修改OSInode中Stat的硬链接数量）

- 因为好像只能通过fd找到一个

- 不能将文件的stat放在OSinode这一层，因为每次open一个文件的时候都是直接new一个出来，这并不能持续保存文件的信息（比如硬链接数量等），所以这里选择将文件的stat保存在inode上（不需要在inode上直接use Stat结构体，因为os目录没有lib文件是不能导入到efs中的。这里是直接在inode上创建。并且当创建一个硬链接的时候需要将两个inode的硬链接数量都进行修改）

- 获取文件的stat仍然通过OSInode来实现，只不过这个时候就需要在File这个trait中增加一个方法来返回文件的状态了（因为进程的文件描述符表中只能存储实现了File trait的东西，所以我并不能确定取出的东西一定是OSinode，所以这里只能给File再写一个方法了）

- as_ref方法是：

  > Converts from &Option<T> to Option<&T>.

  并且他会自动引用当前Option，这样就不会出现使用权的转移了

- 如果一个option是None的话，对他使用map方法就会直接返回None（所以map方法只能对返回值为Option的函数使用）

# 5.11

## Guide6——ch6续

- 有一句话：

  > 本教程只实现文件的顺序读写，而不考虑随机读写。

  这个体现在代码层面就是将OSinode的offset写死为0。在OSinode中调用read方法就会去索引DiskInode上的第零个数据块的块号并从这个位置开始读写，所以就是从文件的开始位置进行读写了

- 在inode的create中其实已经实现了目录项的创建，只是现在不是完全初始化一个Diskinode，而是去拷贝一个diskinode。并且释放的时候需要去修改位图，然后将对应的目录项回收（其实没必要将对应的diskinode进行清空，因为diskinode和inode是绑定的，并且是根据位图进行分配的，所以只需要将位图对应的位置为0就表示刚刚分配的diskinode已经无效了，这样就能在下一次位图分配的时候分配到这个diskinode的位置）

- 由于每一个进程都只是拥有一个inode的Arc指针，所以进程是没有办法修改inode的，而只能修改inode对应的磁盘数据，所以这个时候就只能将硬链接数量继续下放，放在Disknode上了。这样的话就是在Disknode的初始化函数中设置硬链接数量即可

- 在创建一个硬链接的时候一定会涉及到目录项的操作，所以是一定要将其引入的。

- 感觉之前对硬链接的理解错了，手册上说的是不同的目录项索引到同一个文件，意思应该是目录项不同，但是对应的Disknode相同，而不是目录项不同并且Disknode也不同。那这样的话就只需要删除目录项就好了，然后只在关于一个disknode所有的目录项都被删除的时候才会将文件的数据全部回收（也就是修改位图）

- 要按照java的规范来写，如果一个结构体的成员是私有的，那么不应该将其pub出来，而是应该为其专门写一个方法

- 有一段话：

  > - `alloc_data` 和 `dealloc_data` 分配/回收数据块传入/返回的参数都表示数据块在块设备上的编号，而不是在数据块位图中分配的bit编号；
  > - `dealloc_inode` 未实现，不支持文件删除。

  这里说到不支持文件的删除，那硬链接的目录项要怎么删除？这里还是有办法的，看inode的create方法，应该是能将size改小然后将目录项直接向前移的（但是这个好麻烦。好吧其实也不是很麻烦，只要改一下inode的find函数就好了）。而且inode的删除应该是很好实现的，其实就是将对应的disknode位置的数据清零，然后给位图对应的位清零就好了（真是不明白为什么没有实现），甚至可以不将disknode位置的数据清零，只需要将位图清零就好了）

  关于删除目录项的具体实现如下：

  ```rust
  /// delete a dirent by name
      pub fn delete_dirent(&self,name: &str)->i64{
          self.modify_disk_inode(|disk_inode| {
              // assert it is a directory
              assert!(disk_inode.is_dir());
              let file_count = (disk_inode.size as usize) / DIRENT_SZ;
              disk_inode.size= ((file_count-1) * DIRENT_SZ) as u32;
              let mut dirent = DirEntry::empty();
              for i in 0..file_count {
                  assert_eq!(
                      disk_inode.read_at(DIRENT_SZ * i, dirent.as_bytes_mut(), &self.block_device,),
                      DIRENT_SZ,
                  );
                  if dirent.name() == name {
                      // delete the dirent
                      for j in i..file_count {
                          let mut next_dirent = DirEntry::empty();
                          assert_eq!(
                              //注意这里要是j，如果是j+1的话会越界（最后一次循环将读取第file_count+1个DirEntry）
                              disk_inode.read_at(DIRENT_SZ * j, next_dirent.as_bytes_mut(), &self.block_device,),
                              DIRENT_SZ,
                          );
                          disk_inode.write_at(DIRENT_SZ * (j-1), next_dirent.as_bytes(), &self.block_device);
                      }
                      return 0;
                  }
              }
              return -1;
          })
      }
  ```

- 有一段代码：

  ```rust
      let fs = ROOT_INODE.get_fs();
      let mut fs = fs.lock();
  ```

  这段代码就必须在第一行绑定一下get_fs的返回值（因为返回值是一个clone），如果写为：

  ```rust
  let fs = ROOT_INODE.get_fs().lock();
  ```

  这样的话，get_fs返回值会在这行语句结束之后被drop掉，因为他没有被绑定在一个变量上，这个时候lock的东西就没有用了，就相当于创建了一个MutexGuard的裸指针

- inode并没有在磁盘上真实存在，inode只不过是磁盘上的diskinode在内存中的抽象，每次openfile的时候都会返回一个inode

- 因为我在disknode中增加了一个u32成员，所以需要将直接索引的数量-1以保证disknode的大小是128字节

- Disknode中direct存储的都是数据块的编号

- Disknode中给出的write_at中需要提供的offset参数是表示需要读写的位置位于文件或目录的什么位置，也就是提供一个总偏移，而不是一个磁盘块上的偏移

- 需要删除数据块应该比较简单，就直接调用inode的clear方法就好了，这个方法实现了所有数据块在位图上的回收，并且将数据清零了

- 关于diskinode删除的实现如下（在efs中增加函数）：

  ```rust
      /// Deallocate a inode
      pub fn dealloc_inode(&mut self, inode_id: u32) {
          // get the disk block id and offset to clear the DiskNode
          let (block_id,block_offset) = self.get_disk_inode_pos(inode_id);
          get_block_cache(block_id as usize, Arc::clone(&self.block_device))
              .lock()
              .modify(block_offset, |disk_inode: &mut DiskInode| {
                  unsafe{
                      ptr::write_bytes(disk_inode as *mut _ as *mut u8, 0, core::mem::size_of::<DiskInode>());
                  }
              });
          // calculate diskinode id
          self.inode_bitmap.dealloc(&self.block_device, (inode_id) as usize);
      }
  ```

  传入的参数为inode（或者说disknode）在位图上的编号

- 需要注意的是，在efs中，所有跟data块有关的都是直接返回（接收）物理设备上的块号，而inode就是直接返回位图上的编号，这个时候如果要delloc inode的话就只需要直接传入inode_id（也就是位图上的编号）就好了

- 看一下陈老师说的问题的原因是什么，我也遇到了这个问题：

  ![image-20240511160337911](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240511160337911.png)

  要这样修改：

  ![image-20240511160345232](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240511160345232.png)

## Guide8——ch8

这个算法在学校的操作系统课上学过，这里直接冲）

- 上一章遇到的还真是框架代码的问题，在ch8的代码中已经修改了

- ch8把代码大改了一遍，因为这里需要区分进程和线程了，所以线程就用TCB来代替，然后重新创建一个数据结构，也就是PCB（将所有线程共享的资源都放在PCB上）

- 有一段话：

  > 本次实验框架变动较大，且改动较为复杂，为降低同学们的工作量，本次实验不要求合并之前的实验内容， 只需通过 ch8 的全部测例和其他章节的基础测例即可。你可以直接在实验框架的 ch8 分支上完成以下作业。

  根本就不用cherry-pick。。。，但是尝试改了一下框架的代码，确实难改）

- 主要的思路就是：首先需要先让进程能够记录当前是否启用死锁，所以在进程控制块中创建一个新的成员，来表示当前是否需要开启死锁检测，这样sys_enable_deadlock_detect系统调用只需要去对该成员置位即可。然后手册上说对`mutex_lock` （申请一个锁）和 `semaphore_down` （申请一个资源）的操作进行死锁检测，那么就只需要在这两个的系统调用中去判断当前是否开启了死锁检测即可，如果开启了检测的话就执行银行家算法。并且这里不会出现mutex_lock和semaphore_down同时出现导致死锁的情况，这样就可以直接在两个syscall中实现了

- 进程中的分配器是用来分配进程中的tid的

- 信号量的id就是该信号量在semaphore_list中的位置

- 感觉需要在信号量中新建一个队列，表示已经将资源分配给谁了，不然alloc矩阵没办法填（还有一种办法，是在tcb中创建一个跟PCB中的信号量队列类似的信号量队列，并且必须每个位置上的信号量都是一一对应的。这样的话如果在进程中创建了一个信号量，维护成本就很大，因为需要给当前进程中的所有线程的分配资源队列都增加一个成员）

# 5.12

## Guide8——ch8续

- 自旋锁的话一样在锁上维护一个等待队列吧），因为只有实现了Mutex的才能放在锁队列中，导致他的队列只能接受Mutex这个trait，我并不知道是自旋锁还是阻塞锁，我就只能使用Mutex提供的方法了，这样的话就只能去给mutex再加一些方法了
- 需要注意的是，这里是在判断分配了一个资源**之后**会不会出现死锁，所以要尝试分配，对于锁而言，如果当前没有锁可以获取，就需要判断当前占有锁的是哪个线程，如果是本身，那一定会造成死锁，如果是其他线程，可以尝试等待，但是如果是等待的话（等待其他线程分配资源），仍然需要检查当前程序是否处于安全状态。因为如果不检查的话，如果所有资源都已经被分配出去了，这样的话就会出现所有线程都在等待，也就是死锁状况。但是如果在等待之前检查了当前是否是安全的，这样就相当于让出现死锁前的一个锁申请生效，但是在下一次锁申请的时候会检查到死锁。另外就是还有一种办法，就是我直接无脑检查当前系统是否是安全的，如果不是安全的就一定有出现死锁（只不过这个时候就不会去避免死锁了）。讲道理感觉这个才是正解，如果我去尝试分配然后再去检测会不会出现死锁的话，本质上应该算是死锁的避免（而不是死锁的检测，因为如果我尝试分配了之后如果系统不安全了，我就会阻止本次分配，所以应该是死锁的避免）。那我就采用直接无脑检查的方式了，每次分配的时候检查系统是否安全（当然这个时候考虑了当前线程需要当前资源，这样的话按照第一个测试，在第二次申请锁的时候就会因为need(1)>avaliable(0)，而检测出死锁。说白了就是，**need不仅由阻塞队列计算得到，还需要注意当前申请的线程也是需要的**），安全的话就分配，不管分配会不会造成死锁。出现死锁的话就交给下一次资源申请来检测

- 有一段代码：

  ```rust
  self.inner_exclusive_access().res.as_ref().unwrap().tid   
  ```

  和：

  ```rust
  let inner = self.inner_exclusive_access();
  let res=inner.res.as_ref();
  match res {
      None=>{
          println!("res is None");
          return 500;
      }
      Some(_)=>{
          return res.unwrap().tid;
      }
  }
  ```

  第一个在某种情况下使res为None，但是使用第二种解决方式就解决了（而且第一种方法还是有可能出问题，有点没泵注。感觉还是框架代码的问题，如果不是的话就不会有的时候可以有的时候不行了。并且使用第二种方法测试的时候并没有出现res为None的情况）

  应该是在只开"ch8_deadlock_sem2\0",然后再打开"ch8_deadlock_mutex1\0","ch8_deadlock_sem1\0",的时候就会出现（应该就是测试样例由少变多的时候才会出现问题，然后切换了方式之后又发现res不是None，然后正确运行）

  使用第二种方法的时候，在只开一个ch8_deadlock_sem2的情况下，运行结果如下：

  ![image-20240512195736294](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240512195736294.png)

  发现所有的id都是存在并且合法的，并没有出现res=None的情况

  而在使用第一种方法的时候，就出现了下面这个报错

  > [kernel] Panicked at src/task/task.rs:34 called `Option::unwrap()` on a `None` value

  大概就是说我的res为None

  总之如果不是框架代码出了问题，就不会是修改测试样例的时候出现我的代码可能跑不了的情况。这里就先不管了，使用第二种方法先交了再说）

  但是好像针对ci就不会这样，就会一直出现res为None的情况

- 找到问题的原因了（好像不是框架代码的原因，是我自己没看tasks是怎么构造的）：

  ```rust
  while tasks.len() < new_task_tid + 1 {
          tasks.push(None);
      }
      tasks[new_task_tid] = Some(Arc::clone(&new_task));
  ```

  因为在创建一个线程的时候就是，一定要将tid作为其在进程的tasks向量中的索引，所以如果只取出前面几个的话就会导致有空。但是sem_id就不是这样的，因为每次都是直接push一个进去，就能保证一定是一样的（不知道信号量是不是这样的，因为好像每次我创建一个的时候）

  这样的话只需要在函数中判断一下当前tcb是不是None就好了，判断完如果是Node就直接跳过当前行就好了

- 如果是用的是https进行clone的话还要去搞什么token才能改，不如直接密钥克隆





### ch3简答

1. **正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容（运行 [三个 bad 测例 (ch2b_bad_*.rs)](https://github.com/LearningOS/rCore-Tutorial-Test-2024S/tree/master/src/bin) ）， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。**

2. **深入理解 [trap.S](https://github.com/LearningOS/rCore-Tutorial-Code-2024S/blob/ch3/os/src/trap/trap.S) 中两个函数 `__alltraps` 和 `__restore` 的作用，并回答如下问题:**

   1. **L40：刚进入 `__restore` 时，`a0` 代表了什么值。请指出 `__restore` 的两种使用情景。**

   2. **L43-L48：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。**

      ```assembly
      ld t0, 32*8(sp)
      ld t1, 33*8(sp)
      ld t2, 2*8(sp)
      csrw sstatus, t0
      csrw sepc, t1
      csrw sscratch, t2
      ```

   3. **L50-L56：为何跳过了 `x2` 和 `x4`？**

      ```assembly
      ld x1, 1*8(sp)
      ld x3, 3*8(sp)
      .set n, 5
      .rept 27
         LOAD_GP %n
         .set n, n+1
      .endr
      ```

   4. **L60：该指令之后，`sp` 和 `sscratch` 中的值分别有什么意义？**

      ```assembly
      csrrw sp, sscratch, sp
      ```

   5. **`__restore`：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？**

   6. **L13：该指令之后，`sp` 和 `sscratch` 中的值分别有什么意义？**

      ```assembly
      csrrw sp, sscratch, sp
      ```

   7. **从 U 态进入 S 态是哪一条指令发生的？**











有时间可以去看看bootloader的反汇编代码

**解决！**用户程序中的连接脚本指定的基地址有什么用吗，不论我怎么修改用户程序的连接脚本中的BASE_ADDRESS的值，查看elf头的时候都是当前程序的入口点是0x80400000，难道是硬件直接决定的吗，表示PC要在这个地方取值，所以生成elf文件的时候就将入口地址写死了。——因为在user目录下的一个py脚本中指定了所有elf文件的entry为0x80400000，我已修改这个值进行make run的时候就发现elf文件的地址发生变化并且程序没有办法执行（没办法执行应该是因为）。另外所有用户程序的内存起始地址都指定为0x80400000，所以在程序烧录的时候所有程序的烧录地址都是一样的（在第二章中是直接覆盖的，但是在py脚本中有一个宏，指定了在第三章之后所有的程序都需要真实烧录在内存中，也就是从第三章开始所有的实验都是跟后面有关系的，那个时候可能build脚本中就不会直接将程序放在os的数据段中了，到时候可以印证一下）。烧录地址一样就会导致程序丢失，但是在os的build脚本中直接将二进制文件嵌入到os的数据段中了，所以程序最终还是在内存中了，只不过是在os的数据段（所以需要搬运），而不是在SBI指定的0x80400000

（我在os的连接脚本中修改BASE_ADDRESS就能在查看os的elf头的时候发现Entry被修改了，但是应用程序的不行就让我很恼火）

**疑问**zw说的拷贝一份，这不是拷贝出来之后也是一个git仓库吗，这样不是还是不能保存

