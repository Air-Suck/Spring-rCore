# stage3

## 环境配置

今天是5.22。一直以为第三阶段是在六月份才开始的，所以这两天一直在赶学校的作业，一直没有碰第三阶段。。。

在正式进入第三阶段前，我打算先配置一下虚拟机的远程连接，因为VMware实在是太卡了，但是环境都配置在上面又不得不使用虚拟机，就只能尝试一下同学建议的远程连接了

主要参考：[vscode远程连接](https://blog.csdn.net/qq_29856169/article/details/115489702)

主要步骤如下：

- vscode 安装Remote-Developoment并配置
- Linux 安装 OpenSSH-Server 并配置
- 编写 vscode 的 ssh 配置文件

里面有一条指令我觉得很有意思：

```shell
sudo cp /etc/ssh/sshd_config /etc/ssh/sshd_config.backup # 备份
```

竟然可以直接使用cp命令生成备份文件

### 坑点

- 博主说的Remote-SSH版本其实没有问题，可能之后修复了

- 出现Failed to find a non-Windows SSH installed只需要按照提示的要去去关闭相应的配置即可。打开打开“文件->首选项->设置"，然后取消勾选下面的选项即可

  ![img](https://img-blog.csdnimg.cn/20200422192153207.png?x-oss-process=image/watermark,type_ZmFuZ3poZW5naGVpdGk,shadow_10,text_aHR0cHM6Ly9ibG9nLmNzZG4ubmV0L3FxXzI3NzI3MTQ3,size_16,color_FFFFFF,t_70)

- 安装结束之后还有一个很麻烦的地方就是每次远程连接都要去输密码。只要在虚拟机上的~/.ssh/authorized_keys文件中添加Windows的ssh公钥即可实现免密登录。

### 注意点

- ```shell
  type %USERPROFILE%\.ssh\id_rsa.pub
  ```

  这个指令可以查看ssh密钥。

  之前对ssh密钥的理解也不太深入。这个密钥看样子是和机器绑定的。所以如果将本机的ssh公钥放在github上，那么使用ssh进行clone的时候就会去检查github上已有的密钥是否与本机ssh密钥匹配。匹配成功了才会允许进行clone

- 关于rust-analyzer，他是通过识别当前工作区下面的Cargo.toml文件来提供rsut的语言分析的，所以如果出现当前工作区的toml文件并没有在rust-analyzer的工作区中，就会出现无法分析的情况

## 第一周

进入快乐的看文档学习时间。按照ppt上的文档顺序往下看吧

### Rust Futures

主要文档：[200⾏代码讲透RUST FUTURES](https://stevenbai.top/rust/futures_explained_in_200_lines_of_rust/#%E7%BA%BF%E7%A8%8B)

我的笔记永远都非常杂乱无章。在这里我就简单按照章节，然后记录一些我遇到的有意思的点和问题

#### 二 背景资料

- 忘记move关键字是干嘛用的了。。

  > 在Rust中，`move`关键字用于强制闭包获取其环境中的所有权

  也就是将闭包外的变量的所有权直接转移给闭包内部的变量（默认情况使用闭包环境中的变量就是借用）

- 在进程之下的线程也是承担一个任务的执行的，所以当任务数量多了，每一个任务都需要一个线程来执行，并且线程都会有自己的堆栈（虽然相较于进程小得多），这样就会使得线程的开销也很大（但是总比每个任务开一个进程好。。。）

- 关于cargo.toml文件：这个文件相当于是在直到rust编译器的行为。在cargo.toml中需要使用下面的格式指定lib和bin的构建目标：

  ```toml
  [lib]
  name = "my_library"
  path = "src/lib.rs"
  
  [[bin]]
  name = "my_binary"
  path = "src/main.rs"
  ```

  但是如果当前工作区中有一个src/lib.rs文件，那么cargo就会将其认为是lib的构建目标；如果有一个src

  /main.rs文件，就会将其认为是bin的构建目标。rust-analyzer也提示了：

  ```shell
  Caused by:
    no targets specified in the manifest
    either src/lib.rs, src/main.rs, a [lib] section, or [[bin]] section must be present
  ```

- **疑问**这里还是没看懂绿色线程是什么东西。按照查找的资料：

  > 在Rust中，"绿色线程"是指由**语言运行时而不是操作系统直接管理的线程**。这种线程模型在一些其他语言中也很常见，例如Go的goroutines和Erlang的轻量级进程。

  这么来看绿色线程的机制跟操作系统线程相同，但是并不是由操作系统直接管理的线程了（绿色线程是用户级线程？）

- 这个回调就很像数据库中的事务回滚，而数据库中的savepoint就是保存一个指向某一条指令的指针。

- 这里说闭包是一个指针是很合理的。因为创建闭包之后将他的值赋给某一个闭包变量实际上应该就是将闭包的地址（也就是指向一组指令的指针）赋给了闭包变量（这与函数不同，如果硬要说的话，闭包变量的作用应该跟函数名差不多？）

- 关于回调（之前都没有接触过。。。）

  > 回调（Callback）是一种编程模式，你可以把**函数A（回调函数）作为参数传递给另一个函数B**。当函数B在执行过程中达到某个点时，会调用（回调）函数A。
  >
  > 这种模式在异步编程中特别常见，因为它允许你指定一个操作完成或发生某个事件后应该执行的代码。例如，你可能会在发送一个网络请求后提供一个回调函数，以便在请求完成并收到响应后执行某些操作。

  在给出的示例中就是把program_main函数作为回调函数传入了run函数。这里说道：

  > 在 Rust 使用这种方法时，任务之间的状态共享是一个难题，因为它的所有权模型

  在一般的闭包调用中都需要使用move关键字来指明所有权是否需要转移，那在回调的时候自然也需要考虑了

  这么来看的话感觉回调跟普通的函数调用好像没有什么区别。。查看了网上的资料：

  > **在异步编程模型中**，回调函数通常在主调函数之外的某个时间点被调用。例如，当一个长时间运行的操作（如网络请求）完成时，回调函数可能会被调用。在这种情况下，主调函数和回调函数可能会并发执行，因为**主调函数不会等待回调函数**（而是等异步信号到来之后调用回调函数）

  这个时候通过回调函数就实现了异步，但是回调函数和主调函数都是在一个线程中运行的，**只有当异步信号到来的时候才会调用回调函数**。所以实际上主调函数和回调函数之间还是串行的关系（毕竟还是在一个线程中的），但是**是异步的**（异步的实现还是有调度程序实现的？**疑问**）

  在异步编程模型中，普通函数和回调函数的区别如下：

  - 调用方式：普通函数通常是直接调用的，即你在需要的地方写下函数名和参数，然后函数就会执行。而回调函数则是作为参数传递给另一个函数或方法，由这个函数或方法在适当的时候调用。
  - **执行时间**（感觉是最重要的区别）：普通函数在被调用的时候立即执行。而回调函数则是在某个特定事件发生时（例如，异步操作完成，定时器触发等）才被执行。这意味着回调函数的执行可能会被延迟，而且它们的执行顺序可能与代码的书写顺序不同。——就是回调函数是需要等待异步信号的
  - 上下文：普通函数的执行上下文通常是预定义的，而回调函数的执行上下文可能会根据它们被调用的方式而变化。例如，如果回调函数被作为事件处理器使用，那么它的`this`关键字可能会指向触发事件的对象。

- 关于js的异步

  ```js
  async function run() {
      await timer(200);
      await timer(100);
      await timer(50);
      console.log("I'm the last one);
  }
  ```

  在这个例子中任务是**异步**的（尽管他看起来是同步的），因为这里的timer是实打实的任务，而不是一个函数，run函数是在**等待timer子任务结束**而**不是直接执行timer中的代码**（虽然从结果上来看好像是一样的。后面应该能有更好的理解吧）

  使用回调函数本质上也是这样在等待异步信号的到来然后执行回调函数。

  由于我之前没有深入学过js，只是知道一些基本的语法，所以就只能不求甚解了

#### 三 Rust中的Futures

##### Future的三个阶段

-  轮询阶段(The Poll phase). 一个Future（也就是协程）被轮询后,会开始执行,直到被阻塞. 我们经常把轮询一个Future这部分称之为**执行器**(executor) 
- 等待阶段. 事件源(通常称为reactor)注册等待一个事件发生，并确保当该事件准备好时唤醒相应的Future
- 唤醒阶段. 事件发生,相应的Future被唤醒。

所以就相当于是rust**在语言层面实现了一个轮询调度器**

##### Leaf futures与non Leaf futures

future是一个协程，他的返回值相当于是一个异步信号，就是当前异步代码块等待的“未来”

- Leaf futures：它就像套接字一样,代表着一种资源。对**一个资源**进行操作的时候返回的就是一个Leaf futures，也就是一个叶子future（因为是最末端的future了）

- non Leaf futures：这是一种可暂停的计算。这是一个重要的区别，因为**这些Future代表一组操作**，通常，**这样的任务由await一系列leaf-future组成**。所以这个future是一个非叶子future，因为他下面还有许多的叶子future。只有当下级所有的叶子future执行结束之后，这个非叶子future才执行结束。
- Leaf futures与non Leaf futures的区别
  - **leaf代表的是一个资源**，而**non leaf代表**并不是一个资源，而更像**是一个资源的集合**，或者是对资源的一组操作。
  - non leaf 能够将控制权交给运行时的调度程序（当在一个non leaf中使用了await的时候就相当于是将控制权交给了调度器），而leaf不行。这也是为什么异步块一般都是一个non leaf future

##### 运行时

在文档中并没有介绍什么是运行时。实际上这里将的运行时应该是指程序的**运行时环境**

异步运行时环境可以分为两个部分

- 执行器(The Executor) ：主要负责Future**执行**工作的部分。也就是**负责工作的执行**
- reactor (The Reactor)：主要负责**通知**Future它可以做更多工作。也就是**处理服务请求**，或者说是执行调度相关的操作，如挂起唤醒等

copilot给出的解释是：

> Future代表一个可能还没有完成的计算结果
>
> Executor负责**运行**这些Future
>
> 而Reactor则负责在Future需要等待某个事件（如I/O操作完成）时将它们**挂起**，并在事件发生时**唤醒**它们。

##### rust标准库的支持

- 一个公共接口，`Future trait`
- 一个符合人体工程学的方法创建任务, 可以通过**async**和**await**关键字进行**暂停**和**恢复**`Future`
- `Waker`接口, 可以**唤醒**暂停的`Future`

**疑问**Waker接口与await有什么区别？应该都是唤醒协程吧

##### I/O密集型 VS CPU密集型任务

这里有一句话与一段代码：

> 两个`yield`之间的代码与我们的执行器在同一个线程上运行。
>
> 这意味着当我们分析器处理数据集时，执行器忙于计算而不是处理新的请求。

```rust
let non_leaf = async {
    let mut stream = TcpStream::connect("127.0.0.1:3000").await.unwrap(); // <-- yield
    
    // request a large dataset
    let result = stream.write(get_dataset_request).await.unwrap(); // <-- yield
    
    // wait for the dataset
    let mut response = vec![];
    stream.read(&mut response).await.unwrap(); // <-- yield

    // do some CPU-intensive analysis on the dataset
    let report = analyzer::analyze_data(response).unwrap();
    
    // send the results back
    stream.write(report).await.unwrap(); // <-- yield
};
```

先理解这里的两句话：

- 两个`yield`之间的代码与我们的执行器在同一个线程上运行：这一句话的意思是，是由执行器来负责执行两个`yield`之间的代码
- 这意味着当我们分析器处理数据集时，执行器忙于计算而不是处理新的请求：这里说的执行器忙于计算是因为如果两个yield代码之间是一个CPU密集型任务，那么这个CPU密集型任务就会占据执行器，而不会去调度其他的协程（但是此时执行器并没有执行任何操作，只是在等待这个操作的完成，就**相当于是阻塞了执行器**）。而此时可能又有其他的异步请求发出，导致执行器正在被这个CPU密集型的计算任务占据，而不会去处理新的请求，就相当于是执行器被阻塞在当前的异步块线程上了（只有当当前异步块需要等待资源的时候执行器才会去进行调度）。

解决执行器和分析器之间的矛盾（或者说解决执行器阻塞）有下面三个方法：

- > 我们可以创建一个新的`leaf future`，它将我们的任务发送到另一个**线程**（而不是协程，如果是协程的话就跟上面没有区别了，因为执行器还是会在新建的协程中卡住），并在任务完成时解析。 我们可以像等待其他Future一样等待这个`leaf-future`。 
  >

  **这个的意思就相当于是把CPU密集型任务交给了另一个线程来处理，然后在当前的异步块中去等待这个future。这样执行器就会在等待这个CPU密集型任务处理时转去执行其他的协程，就能进一步提高并发度。所以一般的逻辑应该是CPU密集型的任务不应该由协程来执行（因为会导致执行器阻塞），而应该交给线程来执行**

  **疑问**这样不是因为协程的需要，还是创建了一个线程，这样的开销不是跟使用纯线程没有什么区别吗？

  **解决**因为协程并不是为了CPU密集型的任务实现的，是为了IO密集型任务实现的。CPU密集型的任务确实需要线程来执行，因为执行过程中涉及到了太多CPU相关的信息，就只能由操作系统来负责了。虽然说有可能因为协程的需要来创建一个线程，但是使用协程时创建的线程数总是比之前少吧，因为之前甚至一个与外设沟通的任务都需要创建一个线程。

  **疑问**那这跟单纯使用线程池有什么区别？

  copilot的回答如下：

  > 1. **线程管理**：线程池通常为每个任务分配一个线程，当任务需要等待I/O时，线程会被阻塞。如果有大量的并发任务，就需要创建大量的线程，这会消耗大量的系统资源。相比之下，异步执行器可以在少量的线程（甚至只有一个线程）上运行大量的并发任务。当一个任务需要等待I/O时，执行器可以将其挂起，**并在同一个线程上开始执行另一个任务**。这样，**线程就可以在等待I/O时做其他的工作，而不是被阻塞**。
  > 2. **上下文切换**：在线程池中，线程切换通常由操作系统在任何时间点自动进行，这可能会导致大量的上下文切换开销。相比之下，异步任务的切换通常发生在明确的点（即`.await`调用），而不是在任何时间点。这使得执行器可以更好地控制线程切换，例如，它可以在一个任务完成一段计算后再切换到另一个任务，从而减少上下文切换的频率。

- > 运行时可以有某种类型的**管理程序**来监视不同的任务**占用多少时间**，并将执行器本身移动到不同的线程，这样即使我们的分析程序任务阻塞了原始的执行程序线程，它也可以继续运行。 

  这里分析占用多少时间本质上就是计算当前任务占用CPU的时间，对于个计算密集型的任务，这个时间自然就高了，那么由于这个任务占用时间多了，这个时候执行器自己就会跑到另一个线程上去执行。这样当前的异步块就在原有的线程上运行CPU密集型任务，而执行器就切换到另一个线程上去执行调度。所以跟第一种方式的区别就是——第一种是将计算密集型任务移动到一个新的线程上，而第二种就是将执行器移动到一个新的线程上（还是第一种方式好理解一点。。。）

- > 您可以自己创建一个与运行时兼容的`reactor`，以您认为合适的任何方式进行分析，并返回一个可以等待的未来。

所以经过这里对执行器的理解，rust中的future就不能简单理解为是在等待一个资源或者是一组资源了，而是在**等待一个事件**的发生。而在等待这个事件的时候（这个事件一般是由一个**新的线程来处理**或者是外设**给出异步的应答**），执行器就可以先去执行其他的future代码了（也就是执行器不应该被一些需要等待的事情阻塞，无论是等待一个外设的异步信号还是CPU的计算结果）。而在执行器运行的过程中，主线程由于与执行器并没有在一个线程上，所以主线程和执行器线程在**并发**执行

另外执行器好像是**通过线程池**来实现的协程，所以一定程度上还是有线程切换的开销，但是他并不是一个任务对应一个线程，而是多个任务对应一个线程。有一句话印证了我的想法：

> 大多数执行器都可以使用诸如 spawn blocking 之类的方法来完成#1。
>
> 这些方法将任务发送到运行时（运行时就包含了执行器）**创建的线程池**，在该线程池中，您可以执行 cpu 密集型任务，也可以执行运行时不支持的“阻塞”任务。

也就是说方法一中创建出来的线程也是一个future，所以他也会放在线程池中执行（即执行CPU密集型任务）。阻塞任务使用await的时候，也会将这些产生异步信号的任务放到线程池中去执行（即执行运行时不支持的“阻塞”任务）

从上面这一段话中又能看出点东西，就是异步执行器是为了**执行非阻塞**的任务设计的。如果出现了阻塞任务（比如计算密集或者阻塞IO），就需要使用await关键字不让执行器在这当前的阻塞任务上阻塞，而是将这个阻塞任务放到运行时的线程池中执行，然后去在当前线程上重新调度一个准备好了的任务。

到这里才对执行器有了一点点的了解。。。上面写的有点乱，稍微总结一下吧

- 执行器并不占用主线程，而是和主线程并发执行的
- 执行器主要用于执行非阻塞任务
- 执行器遇到阻塞任务时需要使用await关键字，以保证执行器线程不会等待当前的阻塞事件
- 阻塞事件包括计算密集和阻塞IO

#### 四 唤醒器和上下文

到这里是在是看不下去中文版的了，感觉有点机翻的意思，太难懂了，所以选择直接去看英文原文。。。

[原文](https://web.archive.org/web/20230203001355/https://cfsamson.github.io/books-futures-explained/introduction.html)

##### 唤醒器

##### Understanding the Waker

有一段话：

> It's useful to think of it as a `Trait`. It's not implemented as such since that would require us to treat it like a trait object like `&dyn Waker` or `Arc<dyn Waker>` which either restricts the API by requiring a `&dyn Waker` trait object, or would require an `Arc<dyn Waker>` which in turn requires a heap allocation which a lot of embedded-like systems can't do.

大致的意思就是，我只需要将唤醒器理解为一个trait就好了，但是实际上他并不是这样实现的，如果是这样实现的话就需要使用dyn关键字（因为唤醒器与执行器有关，他的行为与运行时有关，所以就很像是动态分发），但是使用动态分发加上引用的话就需要给需要唤醒器的API加上&dyn Waker这个参数（不考虑全局定义一堆唤醒器的情况，因为全局定义一堆唤醒器**不够灵活**，因为这样实现的唤醒器函数内容是不可变的，是在编译的时候由编译器创建的虚拟表决定的。并且这样实现的话当创建一个&dyn Waker时，就一定要在堆上分配空间），这就相当于是给API加上了参数的限制。（如果使用下面的那种方法的话，就只需要全局定义一个唤醒器胖指针，然后不需要给函数传递唤醒器对象，而是直接给唤醒器添加方法之后直接使用就好了）

如果使用动态分发再使用Arc又要需要堆分配的机制（因为Arc指针只能指向一个堆上的数据，这样实现的话每一个唤醒器trait的胖指针都需要分配在堆上），然而像嵌入式系统一样的系统是无法进行堆分配的（因为没有堆或者堆很小）

小小总结一下上面所说的两段话：

- 使用&dyn Waker会将胖指针存放在栈上，但是数据是存放在堆上的
- 使用Arc<dyn Waker\>会将胖指针存放在堆上，数据也存放在堆上

所以如果使用trait的话，胖指针和数据存放在什么位置是编译器给我们定死的。

##### Fat pointers in Rust

这里有一段代码：

```rust
trait SomeTrait { }

fn main() {
    println!("======== The size of different pointers in Rust: ========");
    println!("&dyn Trait:------{}", size_of::<&dyn SomeTrait>());
    println!("&[&dyn Trait]:---{}", size_of::<&[&dyn SomeTrait]>());
    println!("Box<Trait>:------{}", size_of::<Box<SomeTrait>>());
    println!("Box<Box<Trait>>:-{}", size_of::<Box<Box<SomeTrait>>>());
    println!("&i32:------------{}", size_of::<&i32>());
    println!("&[i32]:----------{}", size_of::<&[i32]>());
    println!("Box<i32>:--------{}", size_of::<Box<i32>>());
    println!("&Box<i32>:-------{}", size_of::<&Box<i32>>());
    println!("[&dyn Trait;4]:--{}", size_of::<[&dyn SomeTrait; 4]>());
    println!("[i32;4]:---------{}", size_of::<[i32; 4]>());
}
```

这段代码是没办法运行的，首先是没有size_of方法（size_of方法返回的是**字节数**），其次是，就像之前所说的一样，trait对象在使用的时候必须加上动态分发关键字dyn，这样才能在运行时进行动态分发找到这个trait的实例（就算这个trait对象已经通过Box指针变成了一个堆上的变量也是不行的）。

rust编译器报的错误如下：

```shell
error[E0782]: trait objects must include the `dyn` keyword
```

也就是上面所说的，trait对象一定要加上dyn关键字在运行时动态分发

修改之后的代码为：

```rust
use core::mem::size_of;
trait SomeTrait { }

fn main() {
    println!("======== The size of different pointers in Rust: ========");
    println!("&dyn Trait:------{}", size_of::<&dyn SomeTrait>());
    println!("&[&dyn Trait]:---{}", size_of::<&[&dyn SomeTrait]>());
    println!("Box<Trait>:------{}", size_of::<Box<dyn SomeTrait>>());
    println!("Box<Box<Trait>>:-{}", size_of::<Box<Box<dyn SomeTrait>>>());
    println!("&i32:------------{}", size_of::<&i32>());
    println!("&[i32]:----------{}", size_of::<&[i32]>());
    println!("Box<i32>:--------{}", size_of::<Box<i32>>());
    println!("&Box<i32>:-------{}", size_of::<&Box<i32>>());
    println!("[&dyn Trait;4]:--{}", size_of::<[&dyn SomeTrait; 4]>());
    println!("[i32;4]:---------{}", size_of::<[i32; 4]>());
    // 额外增加两行便于查看机器地址长度
    println!("i32--------------{}",size_of::<i32>());
    println!("&i32------------{}",size_of::<& i32>());
}
```

执行结果为：

```shell
======== The size of different pointers in Rust: ========
&dyn Trait:------16
&[&dyn Trait]:---16
Box<Trait>:------16
Box<Box<Trait>>:-8
&i32:------------8
&[i32]:----------16
Box<i32>:--------8
&Box<i32>:-------8
[&dyn Trait;4]:--64
[i32;4]:---------16
i32--------------4
&i32-------------8
```

按道理来说指针都应该是8个字节64位的（也就是&i32的大小，与机器的地址位数有关），但是这里出现了16个字节的指针（64个字节的就是四个16字节指针组合成一个数组的大小）。而16个字节的指针就是这里说道的胖指针

关于这里的胖指针文档上有介绍

> **Example `&[i32]` :**
>
> - The first 8 bytes is the actual pointer to the first element in the array (or part of an array the slice refers to)
> - The second 8 bytes is the length of the slice.
>
> **Example `&dyn SomeTrait`:**
>
> This is the type of fat pointer we'll concern ourselves about going forward. `&dyn SomeTrait` is a reference to a trait, or what Rust calls a *trait object*.
>
> The layout for a pointer to a *trait object* looks like this:
>
> - The first 8 bytes points to the `data` for the trait object
> - The second 8 bytes points to the `vtable` for the trait object

就是在说：对于&[i32]是16字节的是因为前8个字节保存的是指向数组（或者切片）第一个元素的指针，而**后面的8个字节就是切片的长度**。而对于&dyn SomeTrait，前八个字节保存的是指向trait对象数据的指针（应该就是指向在动态分发后找到的结构体对象），后面八个字节保存的是指向这个**trait对象vtable的指针**

关于vtable：

> Rust中，当你使用动态分发（也就是使用`dyn Trait`类型）时，Rust会使用一种叫做虚拟方法表（vtable）的机制来在运行时查找正确的方法。
>
> **每个实现了特性（trait）的类型**都有一个与之关联的vtable。**vtable是一个包含了特性方法的指针的数组**。当你创建一个`dyn Trait`类型的对象时，Rust会创建一个包含两个部分的胖指针：一个指向对象的数据，另一个指向vtable。
>
> 当你调用一个`dyn Trait`对象的方法时，Rust会使用胖指针中的vtable指针来查找正确的方法，并调用它。这就是动态分发的工作原理。

所以vtable是一张虚拟的方法表，存储的是在运行时确定的，实现了trait的结构体的方法指针，是为了实现动态分发而使用的。

文档中也有一句话与上面所说的差不多：

> The reason for this is to allow us to refer to an object we know nothing about except that it implements the methods defined by our trait. To accomplish this we use *dynamic dispatch*.

大致意思就是使用trait对象是为了能够让我们能引用一个我们在编译时无法确定但是一定实现了trait方法的对象。为了实现这个我们就使用了动态分发这个方法（也就是dyn关键字）。而动态分发又离不开vtable的支持

接下来文档中的一些代码都没有导库，这里就不一一赘述了

下面的这一段代码很有意思：

```rust
// A reference to a trait object is a fat pointer: (data_ptr, vtable_ptr)
trait Test {
    fn add(&self) -> i32;
    fn sub(&self) -> i32;
    fn mul(&self) -> i32;
}

// This will represent our home-brewed fat pointer to a trait object
#[repr(C)]
struct FatPointer<'a> {
    /// A reference is a pointer to an instantiated `Data` instance
    data: &'a mut Data,
    /// Since we need to pass in literal values like length and alignment it's
    /// easiest for us to convert pointers to usize-integers instead of the other way around.
    vtable: *const usize,
}

// This is the data in our trait object. It's just two numbers we want to operate on.
struct Data {
    a: i32,
    b: i32,
}

// ====== function definitions ======
fn add(s: &Data) -> i32 {
    s.a + s.b
}
fn sub(s: &Data) -> i32 {
    s.a - s.b
}
fn mul(s: &Data) -> i32 {
    s.a * s.b
}

fn main() {
    let mut data = Data {a: 3, b: 2};
    // vtable is like special purpose array of pointer-length types with a fixed
    // format where the three first values contains some general information like
    // a pointer to drop and the length and data alignment of `data`.
    let vtable = vec![
        0,                  // pointer to `Drop` (which we're not implementing here)
        size_of::<Data>(),  // length of data
        align_of::<Data>(), // alignment of data

        // we need to make sure we add these in the same order as defined in the Trait.
        add as usize, // function pointer - try changing the order of `add`
        sub as usize, // function pointer - and `sub` to see what happens
        mul as usize, // function pointer
    ];

    let fat_pointer = FatPointer { data: &mut data, vtable: vtable.as_ptr()};
    let test = unsafe { std::mem::transmute::<FatPointer, &dyn Test>(fat_pointer) };

    // And voalá, it's now a trait object we can call methods on
    println!("Add: 3 + 2 = {}", test.add());
    println!("Sub: 3 - 2 = {}", test.sub());
    println!("Mul: 3 * 2 = {}", test.mul());
}
```

他首先是定义了一个trait，然后定义了一个结构体，以及三个方法（虽然方法名和trait中的方法名是相同的）。然后他在这里是直接手动创建了一个胖指针结构体：先初始化一个“实现”了trait的结构体，然后再手动构建动态分发需要的虚拟表，然后将这个胖指针结构体强制转换为一个trait对象的胖指针，**结果是跟直接给Data结构体实现trait的效果是一样的**——方法都能直接进行动态分发调用。就证明了trait的胖指针就是这样子工作的

虚拟表中第一个方法是drop，第二个方法是sizeof，第三个方法是align_of，这三个方法应该都是rust对每一个对象都要实现的方法吧，所以才会在虚拟表中加上这几个函数的地址

对于trait而言，他的创建虚拟方法表的过程是由编译器执行的，并且由于不知道实现trait的具体类型是什么，所以只能在堆上分配空间

而对唤醒器而言，我们将其设计为一个胖指针，并手动为其创建虚拟方法表，就使得它的大小在编译时就能确定（因为是一个胖指针，并且他的数据部分也是我们自己赋值的，也能确定大小。所以唤醒器能在没有堆分配机制的系统上也能使用），并且能够像使用一个trait一样去使用一个唤醒器

我现在有一个疑问就是，这样来看的话唤醒器和一个直接实现方法的结构体好像没有区别（像trait一样使用。至于跟trait的区别前面也提到过了，就是灵活性的问题）。实际上唤醒器和一个直接实现了方法的结构体最大的区别也是**灵活性**：

> **灵活性**：`Waker`是一个非透明类型，它封装了一个`RawWaker`，这是一个胖指针，包含一个数据指针和一个指向虚拟方法表（vtable）的指针。这种设计允许`Waker`在运行时动态地处理任何实现了`RawWakerVTable`的类型。这意味着你可以为你的异步运行时创建自定义的`Waker`，只需要提供一个实现了`RawWakerVTable`的类型即可。相比之下，一个直接实现了这些方法的结构体在编译时就需要知道具体的类型，这限制了它的灵活性。

也就是说，我可以在需要的地方创建RawWakerVTable结构体，其中存放的函数是不同的，这样我**可以按照异步的需要切换函数的内容但是函数调用方式不发生变化**；而在一般的结构体中，**函数的内容是不可能替换的**。这样就使得唤醒器能针对不同的场景**实现动态的变化**（因为唤醒器确实需要依赖于执行器、异步任务的内容等，他是需要动态变化的）。

这么来看的话一个直接实现方法的结构体和一个trait好像没有什么区别，这俩最大的区别就是**trait可以动态分发**

文档上还说出了不将唤醒器定义成trait的另一个原因（一个是因为灵活性的问题）：

> The reason is flexibility. Implementing the Waker the way we do here gives a lot of flexibility of choosing what memory management scheme to use.

大致意思就是能更为自由的选择内存管理方案

这样子实现唤醒器的话就能够**实现将胖指针中的数据存放在栈上**，因为胖指针中的数据指针是我们自己设定的，也就是说我们能够自己实现内存的控制，决定唤醒器应该分配在栈上还是分配在堆上。

## 第二周

先把embassy放一放，先把两百行看完然后去看向勇老师的课程

发现还是杂记比较适合我。。。

### Rust Futures

这里先回去补一下在中文版中缺失的章节

#### Future与运行时工作模型

这一章还是很有必要看的，因为能让我有一个整体的把握。后面去学唤醒器啥的也会有底。

这一章主要是将伪代码和图形结合起来，来介绍Future与运行时的工作模型。

下面就介绍一下异步模型的全过程

- 阻塞当前线程

  ![image-20240531113946000](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240531113946000.png)

  这里是通过执行器的block_on方法将当前线程阻塞了。意思是当等到了fut之后，当前线程才会继续执行

- 生成状态机

  ![image-20240531114145401](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240531114145401.png)

  编译器会根据async关键字和await关键字生成一个状态机（这个状态机的状态转换条件就是await事件的发生）

- 执行future（也就是尝试执行传入的这个future）

  ![image-20240531114303558](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240531114303558.png)

  这里是通过调用fut的方法来进行轮询。这里还需要传递一个唤醒器，以便这个fut能被唤醒（在等待阻塞事件的时候）。

  这里说到的不能像一个实例方法一样去直接调用poll方法，是因为poll方法的函数签名如下：

  ```rust
  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>; 
  ```

  这里的self并不是fut本身，而是要把fut包装在Pin中（所以不能直接调用实例方法）；而唤醒器也是要包装在一个Context中的（下面也提到了，这里需要一些额外的信息，来唤醒fut。就后好像是进程切换的时候的上下文一样，是fut恢复所需要的一些额外的信息）。所以这里不能这样使用non_leaf_fut.poll（waker）。

  这里还提到了waker。这里的意思应该是poll函数会先把这个waker挂起，然后当这个fut到达了await点的时候，这个时候才会执行上面提到的non_leaf_fut.poll（waker）。

  这个意思应该是，如果代码是非阻塞的就先不需要管这个waker了，一直执行到需要阻塞的地方了之后才会开始进行waker相关的部分。

  这里再稍微提一下poll的作用：

  > 在Rust中，`Future::poll`方法用于尝试驱动`Future`向前

  也就是尝试执行传入的这个fut，一直到需要await的时候再将当前的fut挂起，然后执行器重调度。

  上面关于waker的部分也是这样说的，说到的是只有当fut执行到一个await点的时候才涉及到waker相关的操作（也就是poll函数在驱使当前fut向前执行直到await）

- 状态转换

  ![image-20240531120205678](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240531120205678.png)

  正如上面说的，poll函数是在执行当前的代码直到需要阻塞（也就是需要await）的时候才会停止。那么经过一次poll之后，状态机将转换到第一个await状态，也就是执行到第一个await之前（上图中的第一行）

- 创建叶子fut

  ![image-20240531120422425](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240531120422425.png)

  这里只需要知道叶子fut是通过reactor创建的即可（相当于是在reactor中**注册了一个fut**，相当于是在操作系统中的spwan操作？**疑问**）

- Reactor执行细节

  ![image-20240531120652739](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240531120652739.png)

  这里说道实际上就是将一个操作转换成了一个fut（所以就相当于是创建了一个fut，或者说注册了一个fut。正如Reactor旁边显示的伪代码一样）

- await fut

  ![image-20240531120822421](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240531120822421.png)

  这里文字没有含金量，下一个

- await执行细节

  ![image-20240531120931173](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240531120931173.png)

  首先先看执行器旁边的伪代码，可以发现实际上这里是一个树形的执行结构，就是先poll non leaf，等到遇到了leaf，再执行poll leaf。tip中提到的就没什么用了，就是上面说的不能直接像调用实例方法一样去调用

  关于waker：这里说道在poll一个non leaf的时候，唤醒器会通过non leaf中的leaf被传递给Reactor。

  这么来看的话，其实Reactor就持有了很多的Context（包含waker），用于唤醒fut；而执行器就持有了很多的fut，用于执行（poll）fut。所以正像上面所说的，执行器负责执行fut，而Reactor负责调度相关的（尤其是唤醒fut）

- 存储waker细节

  ![image-20240531121725048](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240531121725048.png)

  这里展示了leaf poll的执行细节。当执行了一个leaf poll的时候，就会给唤醒器分配一个编号，然后再将唤醒器存储在Reactor中。

  从这里其实就能看出来Reactor和执行器的解耦合了。因为如果没有waker的话，Reactor就需要直接去操作执行器中的fut，然后去唤醒他们；而如果使用了waker的话就使得Reactor只需要调用自己内部的waker的方法自然就能将执行器中的fut唤醒了

- poll的返回（阻塞）

  ![image-20240531122536575](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240531122536575.png)

  如果当叶子fut需要等待的话，就会返回一个Pending，而不是在那里阻塞等待。返回Pending的时候就会将当前这个fut置入执行器的等待队列了（相当于非阻塞等待。这个时候应该用了另一个线程来执行这个被等待的fut）。

  需要注意的是这里只将最外层的non leaf fut置入了等待队列，尽管实际上等待的是非叶子里面的叶子。从这里就能看出，在一个non leaf中的fut是不会异步执行的，而是串行同步执行的。因为等待队列中只会有最上层的non leaf，所以最上层的non leaf才是最小的调度对象。故内部的leaf之间不会异步执行（如果要让两个fut异步执行的话就一定要让这两个fut都是最上层的leaf了）

  这里也说道执行器一般都提供了方法来创建一个最上层的叶子（正如这里所说的，很像一个spawn，只不过这里是创建了另一个异步块，而不是一个线程或者进程）

- 执行器的调度

  ![image-20240531134014775](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240531134014775.png)

  当当前的fut被pending的时候，执行器就会回到就绪队列中选出一个fut来执行（不是在await状态，即被Reactor唤醒的fut）

  这里说道的是如果就绪队列中没有fut的话，执行器就会休眠（所以之后一定有一个唤醒机制，这个唤醒执行器的机制应该是由Reactor来执行的）

  在上图的例子中实际上就是执行器休眠的情况

- Reactor的唤醒

  ![image-20240531134349085](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240531134349085.png)

  就是当一个fut执行结束了之后，Reactor就会唤醒该fut

- Reactor唤醒细节

  ![image-20240531134439102](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240531134439102.png)

  也没什么，实际上就是根据保存waker时的id来获取对应的waker，然后将对应的fut唤醒

- 唤醒的结果

  ![image-20240531134551428](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240531134551428.png)

  最简单直白的就是将被唤醒的fut置入就绪队列，然后执行器就发现就绪队列中有一个fut可以执行了（正如右边的文字所说的，唤醒执行器也是这个时候进行的），就接着执行该fut

  这里还需要注意的一点是如果执行了wake方法的话，在Reactor中存储的waker就会被移出Reactor（因为之后的poll需要的唤醒器可能不同，所以唤醒器就好像是一次性的一样）

- fut的完成

  ![image-20240531134855876](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240531134855876.png)

  在执行结束之后，poll函数返回ready，表示当前的await执行结束。poll执行结束之后状态机就会再进行一次状态转换。

  如果当前的ready之后没有await了，那么就完成了当前的fut

- 继续执行当前线程

  ![image-20240531135126680](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240531135126680.png)

  之前通过block_on方法阻塞了当前线程。block结束的条件根据右边的文字猜测就是当前执行器的阻塞队列中已经没有fut了。

这里总结一下，首先创建了一个异步块之后就会生成一个状态机，这个状态机的状态转换条件就是await的执行。每次执行器对就绪队列中的某个fut调用poll方法之后就会执行当前fut，然后直到下一个await（是递归的），所以每次poll都会让状态机进行一次状态转换（只不过在最外层来看可能是阻塞在同一行上的）。直到最后一个await执行结束之后对当前的fut执行poll方法，就会将该fut执行完毕，然后返回一个最终的ready，让当前的线程不再阻塞。

而在这个过程中，创建fut是由Reactor执行的；执行fut是由执行器的poll方法进行的；将fut置入阻塞队列是执行器进行的；当异步信号发生时，唤醒fut是由Reactor执行的。Reactor与执行器之间通过唤醒器实现了解耦合

#### 五 Generators and async/await

这里有一个链接，讲的是生成器的motivation，之后有空可以看看：[The motivation for Generators](https://web.archive.org/web/20221206132735/https://github.com/rust-lang/rfcs/blob/master/text/2033-experimental-coroutines.md)

##### 学习生成器的必要性

作者在这里说了，生成器与yield与async/await非常像，并且生成器比较简单，可以给出示例，所以这里介绍了生成器

- async/await与生成器/yield的区别
  - 生成器返回的是一个generator，而async返回的是一个fut对象
- rust中三种实现并行（或者说并发）的机制
  - 拥有栈的协程，也就是绿色线程
  - 使用组合器
  - 没有栈的协程，也就是生成器（async也差不多。这也是现在rust中使用的机制）

##### combinators

作者说js中也有组合器，并且提供了一段rust中的组合器的代码。但是我不太会js，所以这里就不继续深究这个组合器了（反正现在rust里面使用的也不是组合器）

- 组合器的三个缺点

  - 错误信息冗长

  - 没有有效使用内存

  - 不允许跨组合器步骤（或者说终止点）进行借用（涉及到rust的语言特性）

    这个也是之前的fut最大的缺陷。正是由于这个缺陷，导致在不同的步骤（终止点）之间需要进行额外的空间分配，这也导致了第二个缺点——没有有效使用内存

作者在这里也说到了，使用组合器本质上跟回调差不多，这样就使得所有的函数闭包都需要存储所有的数据（这里可能指的是之前rust中的函数闭包，因为在现在的rust中，函数闭包已经能捕获环境中的变量，并且如果没有使用move关键字的话就会以引用的形式将变量传递给函数闭包）。而如果所有的闭包都需要存储该闭包所需数据的话（是数据实体而不是引用），那么就会使得内存的使用率随着代码的执行（也就是added step。每一个step都可以理解为是一个函数或者fut。在组合器模式中就是直接额外分配内存空间给新的step使用）越来越高。说白了就是上面的2、3两个缺点

##### Stackless coroutines/generators

标题的意思是：没有栈的协程/生成器（这也就是现在rust中使用的生成器，async创建的协程也是使用这种机制，所以开销很小，因为栈空间都没有了）

这里就将生成器和协程理解为一个包含了若干步骤的函数即可（生成器这个名字还是有点太容易误导人了）

- 无栈协程的优点

  - 使用关键字（async/await）能很便捷地创建一个无栈协程

    这里作者还提到了一句话： it can even be done using a macro，说的是无栈协程还可以通过宏来实现，之后可以去看看。**埋个坑**

  - 不需要进行CPU上下文切换

    对线程或者有栈协程而言，他们已经占据了一定的cpu资源，相当于是cpu可见的（当然用户线程是cpu不可见的，但是还是存在自己的用户栈，这个时候切换用户级线程的时候也是需要保存栈指针以及一些执行现场的）

  - 无需处理动态的栈分配

    动态的栈分配通常是在处理递归的时候出现的，因为编译器在编译的时候并不能确定递归函数需要多少的栈空间，这个时候就只能在运行时动态分配函数栈空间了。

    关于动态的栈空间分配，copilot给出的解释为：

    > 因此，当你使用无栈协程时，你不需要考虑动态栈分配。这是因为无栈协程不使用调用栈，所以不需要为每个协程分配和管理一个栈。这可以简化编程模型，并可能提高性能，特别是在创建和销毁大量协程时。
    >
    > 然而，无栈协程也有一些限制。例如，由于它们不使用调用栈，所以**不能使用某些依赖于栈的语言特性，如递归函数**。此外，由于上下文保存在堆上，所以无栈协程可能需要更多的堆内存。

    需要注意的是，这里只是介绍广义上的无栈协程。而在rust的语境下，这里使用更多的堆空间可以理解为要存储状态机等。另外在async块中是可以使用递归函数的，只不过这个时候rust编译器并不是在栈上为递归函数分配空间，而是把他当成了一个fut：

    > 当你在async块或async函数中调用一个递归函数时，**Rust编译器会为每个递归调用生成一个新的`Future`**。这个`Future`会保存递归调用的状态，包括局部变量和程序计数器。这些状态被存储在堆上，而不是在栈上。

    所以这个时候就没有涉及到栈的变化了。

    这里确实可以在堆上分配递归函数所需要的空间，这是通过状态机（堆上的）实现的。但是下面这个问题还是存在。

    **疑问**如果在这里能使用递归函数的话，那么怎么确定一个fut所需的大小？（因为如果不确定的话，后面所说的*the largest footprint that a single step requires*.也就没办法确定了）

    既然这个无栈协程是没有栈的，所以这里自然不会出现栈分配的相关问题

  - 内存使用率高

    因为是没有栈的，并且允许不同的await中去借用同一个变量，不会使得每一步的变量都必须是占有所有权的

  - 允许借用

    上面两个优点实际上就跟组合器的后两个缺点对上了

  关于最后一点，有一段代码：

  ```rust
  async fn myfn() {
      let text = String::from("Hello world");
      let borrowed = &text[0..5];
      somefuture.await;
      println!("{}", borrowed);
  }
  ```

  这里能发现，在这个异步函数中，就已经能够使用外部变量的引用了（如上面的text）。

  实际上现在在函数闭包中也能这样使用。使用函数闭包不加move关键字就是这个效果，可以直接在函数闭包内部使用环境中变量（在这里是text）的引用，

- async的实现

  这里有一句话

  > Async in Rust is implemented using Generators

  所以本质上来说，生成器和async没有什么区别，只不过async应该是多走了一步返回的是一个fut。所以要理解async的实现，就需要了解生成器的实现

  有一段话：

  > Generators in Rust are implemented as state machines

  这里说的是，**rust中生成器被实现为一个状态机**。这也对应了上面工作模型中所介绍的，这里的生成器状态机应该就是上面通过poll函数进行状态转换的状态机

- 使用无栈协程为什么能提高内存的利用率？（这里指的是async块）

  - 异步函数能捕获环境中的变量，并以借用的方式使用。这样就不会使得每一个异步的step都占有所需的所有数据（以copy形式），进而提高了内存的利用率

  - 在组合器模式下，随着代码的运行所需的内存空间也会变大（并且是不可预知的，只能在运行的时候确定）。而在无栈协程的模式下的机制为：

    > The memory footprint of a chain of computations is defined by *the largest footprint that a single step requires*.

    下面我就将这个机制称为**最大内存策略**

    即**在一系列计算中，整个计算链所需的内存量由单个步骤中最大的内存占用定义**。首先需要知道的是，每一个step都是独立的，需要自己的内存空间（所以在组合器模式中每一步相当于是一个函数或者fut）。在无栈协程的模式下，执行完一个fut，当需要执行下一个fut的时候，会直接使用上一个fut使用的内存空间。下面给出两个copilot给出的例子，感觉这两个例子还是不错的。

    使用组合器：

    > 在Rust的异步编程中，组合器是一种常用的模式，它允许我们将多个Future链接在一起，形成一个新的Future。每个Future可能需要存储一些状态，以便在被暂停和恢复时继续执行。
    >
    > 例如，假设我们有一个Future `f1`，它需要1MB的内存来存储状态。然后，我们使用组合器将`f1`和另一个Future `f2`链接在一起，`f2`需要2MB的内存。结果Future `f1_then_f2`需要3MB的内存，因为它需要存储`f1`和`f2`的状态。

    这里组合器使用的方式是：**链接**（所以是内存不断叠加，相当于是每一个步骤都需要有自己的空间。但实际上是没有必要的，因为一个fut执行结束之后他的信息已经不重要了）

    使用无栈协程：

    > 举个例子，假设你有一个计算链，包括三个步骤：步骤A需要1MB的内存，步骤B需要2MB的内存，步骤C需要1.5MB的内存。在这种情况下，整个计算链的内存占用将被定义为2MB，即使其他步骤可能需要更少的内存。

    这里看似使用无栈协程在执行A、C时有内存空间的浪费，但是如果使用的是组合器模式的话，这里就是将ABC三个fut链接在一起，所需的内存就是4.5MB了。

    而使用无栈线程的话，此时无论是多少个step（或者是fut），占用的内存空间就都是2MB（不一定是2，是由占用内存最多的step决定）

    总而言之，组合器模式中所有的fut都有自己的空间；而在无栈协程模式中，所有的fut使用的是同一个空间。这也是为什么在组合器模式中fut需要保存所有自己所需的数据（有了自己的空间就相当于有了一个内存隔离机制），而无栈协程可以直接使用外部引用了（因为所有的fut都是共享一片空间）。

    这也解释了为什么在一个async块中，所有的fut都是串行的了。因为如果不是串行的话就一定会有自己的执行现场，就不可避免的需要更多的存储空间。而如果是串行的话，一个fut执行结束了就是执行结束了，之后就不会再使用了，所以这个fut的内存空间可以重新利用给当前async块中的其他fut使用。

    再回过头来看，如果使用的是组合器模式的话，如果没有严格规定协程之间的同步关系的话（但是看样子好像是有规定同步关系的，因为是链接在一起了，这时候应该是有一个先后顺序的。从copilot提供的例子中的then中也能看出来应该是有同步关系的），其实并发度应该是比无栈协程更高的，因为在组合器模式中，每一个fut都有自己的空间，那么按道理来说他们就完全可以并发执行，而不用像无栈协程一样在一个async块中串行。

    到这里就大概能理解，如果使用了生成器（无栈协程）机制的话，需要两块空间：一个用于存储状态机（应该是分配在堆上），另一个是fut执行所需的空间。由于需要额外存储状态机，就不好说这个模式的内存占用率有多高了（但实际上状态机应该不会占用很多的内存，应该只是存储了一张图）。但是无论如何，使用生成器都有一个优点——所需的内存空间在编译之后就能确定（这里说的确定是甚至**栈空间都不会变化**。也就是**没有动态的栈分配**。组合器模式下可能需要）

##### How generators work

- yield关键字：yield关键字的作用就是**将一个函数闭包变成了一个生成器**

  为什么yield关键字能把一个闭包转换为一个生成器？——一个生成器最本质的特征就是他能暂停执行，之后还能恢复执行（而如果再将生成器包装成一个异步块的话，那么就有最大内存策略了）。而在函数闭包使用yield就能让函数闭包有这个特点，所以就将一个函数闭包转换为了一个生成器对象。

  到这里也能理解到生成器为什么叫生成器了，因为生成器是通过yield来**生成**一系列的值。每次执行生成器的resume函数的时候，都会获取生成器中通过yield关键字返回的值（并且此时生成器将在yield处停止）。

  这里还提供了一段关于yield代码经过编译之后的代码：

  ```rust
  fn main() {
      let mut gen = GeneratorA::start(4);
  
      if let GeneratorState::Yielded(n) = gen.resume() {
          println!("Got value {}", n);
      }
  
      if let GeneratorState::Complete(()) = gen.resume() {
          ()
      };
  }
  
  // If you've ever wondered why the parameters are called Y and R the naming from
  // the original rfc most likely holds the answer
  enum GeneratorState<Y, R> {
      Yielded(Y),  // originally called `Yield(Y)`
      Complete(R), // originally called `Return(R)`
  }
  
  trait Generator {
      type Yield;
      type Return;
      fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return>;
  }
  
  enum GeneratorA {
      Enter(i32),
      Yield1(i32),
      Exit,
  }
  
  impl GeneratorA {
      fn start(a1: i32) -> Self {
          GeneratorA::Enter(a1)
      }
  }
  
  impl Generator for GeneratorA {
      type Yield = i32;
      type Return = ();
      fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return> {
          // lets us get ownership over current state
          match std::mem::replace(self, GeneratorA::Exit) {
              GeneratorA::Enter(a1) => {
  
            /*----code before yield----*/
                  println!("Hello");
                  let a = a1 * 2;
  
                  *self = GeneratorA::Yield1(a);
                  GeneratorState::Yielded(a)
              }
  
              GeneratorA::Yield1(_) => {
            /*-----code after yield-----*/
                  println!("world!");
  
                  *self = GeneratorA::Exit;
                  GeneratorState::Complete(())
              }
              GeneratorA::Exit => panic!("Can't advance an exited generator!"),
          }
      }
  }
  ```

  从这里可以发现，实现函数的暂停执行以及继续执行实际上就是把使用了yield的函数按照yield使用的位置分成了若干个部分，然后根据一个枚举变量来决定生成器下一次resume的时候需要执行的是哪些部分（这里是只有一个yield，所以GeneratorA枚举中只有一个Yield1。如果有多个yield的话，枚举中就应该增加几个yield成员了）

  使用这种方式，虽然是使用栈进行resume的调用，但是这里通过GeneratorA枚举相当于是保存了后面的yield执行所需要的现场，也就是上面的*self = GeneratorA::Yield1(a);等操作。这个操作实现的效果就是将上一次yield中的a，或者说局部变量（所有的局部变量，包括引用变量），保存到了后面的yield块中。

  但是如果后面的代码中需要使用变量a（以上面的例子为例）的引用的话，就会出现问题，因为变量a在第一次退出resume函数的时候就被销毁了，但是这个时候在后面的代码中还有对a的引用，这个时候就会出现生命周期的问题（yield重写代码如果是简单拷贝的话就会出现这个问题）

  另外这里GeneratorState传入两个泛型YR分别表示yield的类型和返回值的类型

  可以看出来一个生成器是通过yield关键字重写代码实现的。然后每次的resume都会让生成器对应的状态机转换一次状态（**这个resume函数的实现就已经很像一个状态机了**。这个·resume就对应了fut的poll函数。后面也确实说了**这个resume就是一个状态机**）

  这里还有更多的关于yield关键字的信息：[RFC1823](https://web.archive.org/web/20221208133637/https://github.com/rust-lang/rfcs/pull/1823)、[RFC1832](https://web.archive.org/web/20221208133637/https://github.com/rust-lang/rfcs/pull/1832)

  这里还有一篇讲异步优化的文章：[Tyler Mandry's excellent article: How Rust optimizes async/await](https://web.archive.org/web/20221208133637/https://tmandry.gitlab.io/blog/posts/optimizing-await-1/)

- 使用yield的限制——借用

  正如上面所说的，在yield点之后如果再使用yield点之前变量的引用的话，就会出现引用的生命周期比变量本身还长的情况（因为变量本身在前一次resume中就被回收了，而该变量的引用在下一次resume中还在使用）。这甚至都没办法通过编译器中生命周期检查器的检查。

  而这个借用，就是要在await中去解决的一个点（async有一个优点就是能跨await借用）

  关于yield借用限制的详细说明

  ```rust
  let mut generator = move || {
          let to_borrow = String::from("Hello");
          let borrowed = &to_borrow;
          yield borrowed.len();
          println!("{} world!", borrowed);
      };
  ```

  在这里就体现出了yield的限制。上面这段代码中就是在后面使用了yield之前的变量to_borrow的引用。

  按照之前所提到的，这一段yield代码经过编译之后生成的代码应该为：

  ```rust
  enum GeneratorA {
      Enter,
      Yield1 {
          to_borrow: String,
          borrowed: &String, // uh, what lifetime should this have?
      },
      Exit,
  }
  
  impl Generator for GeneratorA {
      type Yield = usize;
      type Return = ();
      fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return> {
          // lets us get ownership over current state
          // 这里需要当前状态的所有权是因为后面要获取to_borrow等变量的所有权
          match std::mem::replace(self, GeneratorA::Exit) {
              GeneratorA::Enter => {
                  let to_borrow = String::from("Hello");
                  let borrowed = &to_borrow; // <--- NB!
                  let res = borrowed.len();
  				//这里应该也是有问题的，因为引用是不能直接取*的（这样就相当于是要通过引用来影响所有权了），这里应该直接赋值就好了
                  self = GeneratorA::Yield1 {to_borrow, borrowed};
                  GeneratorState::Yielded(res)
              }
              GeneratorA::Yield1 {to_borrow, borrowed} => {
                  println!("Hello {}", borrowed);
                  self = GeneratorA::Exit;
                  GeneratorState::Complete(())
              }
              GeneratorA::Exit => panic!("Can't advance an exited generator!"),
          }
      }
  }
  ```

  这里由于在yield之前有一个引用变量，所以在进行yield传递的时候，Yield1携带的变量信息就要带上borrowed这个引用变量。但是在一个结构体中使用了一个引用变量的时候就一定要使用生命周期语法了，或者说，他的生命周期是要确定的。这个变量本身应该随着resume的执行结束就已经被出栈销毁了，所以这个引用变量的生命周期应该在创建其引用的变量的yield块中有效，遇到下一个yield之后变量就被销毁了。所以这个引用的生命周期既不与GeneratorA相同，也不是静态的，而是在下一个yield的时候结束，**这个是没有办法通过rust生命周期语法来表达的**。但是实际上我们知道他引用的变量就是borrowed，并且borrowed的所有权在执行*self = GeneratorA::Yield1 {to_borrow, borrowed};时已经被转移到了Yield1 中，所以我们知道borrowed的生命周期与to_borrow相同，所以这里就可以使用裸指针来保存这个引用变量，从而绕过编译器的生命周期检查。即：

  ```rust
  enum GeneratorState<Y, R> {
      Yielded(Y),
      Complete(R),
  }
  
  trait Generator {
      type Yield;
      type Return;
      fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return>;
  }
  
  enum GeneratorA {
      Enter,
      Yield1 {
          to_borrow: String,
          borrowed: *const String, // NB! This is now a raw pointer!
      },
      Exit,
  }
  
  impl GeneratorA {
      fn start() -> Self {
          GeneratorA::Enter
      }
  }
  impl Generator for GeneratorA {
      type Yield = usize;
      type Return = ();
      fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return> {
          	//这里只需要传递引用即可，因为后面不需要获取to_borrow等变量的所有权
              match self {
              GeneratorA::Enter => {
                  let to_borrow = String::from("Hello");
                  let borrowed = &to_borrow;
                  let res = borrowed.len();
                  *self = GeneratorA::Yield1 {to_borrow, borrowed: std::ptr::null()};
  
                  // NB! And we set the pointer to reference the to_borrow string here
                  if let GeneratorA::Yield1 {to_borrow, borrowed} = self {
                      //这里是引用，所以没问题
                      *borrowed = to_borrow;
                  }
                  GeneratorState::Yielded(res)
              }
  			//这里是引用，所以borrowed应该是&&String，所以{&**borrowed}得到的是&String
              GeneratorA::Yield1 {borrowed, ..} => {
                  //这一行真的有必要吗。。。疑问（这里的..就表示了忽略其他的值，不论是在前面的还是后面的）
                  let borrowed: &String = unsafe {&**borrowed};
                  println!("{} world", borrowed);
                  *self = GeneratorA::Exit;
                  GeneratorState::Complete(())
              }
              GeneratorA::Exit => panic!("Can't advance an exited generator!"),
          }
      }
  }
  ```

  **解决**，如果是一个实现了copy的变量进行传递的话，执行*self = GeneratorA::Yield1 {to_borrow, borrowed};的时候就不会出现所有权的转移，而是重新分配一个变量，那么这个时候引用变量所引用的对象不就被销毁了吗？——所以这个时候是先将变量传进Yield1，然后再对borrowed进行赋值，这样就能保证borrowed引用的变量一定是Yield1中的变量了

  这里的borrowed指向的是本身结构体中的to_borrow，就构成了一个self-referential structs（自指结构体）

  到这里好像就能把这个生成器当成一个async状态机了。但是这里会出现一个问题——在安全的rust中可能会使用swap函数，也就是下面的代码：
  
  ```rust
  pub fn main() {
      let mut gen = GeneratorA::start();
      let mut gen2 = GeneratorA::start();
  
      if let GeneratorState::Yielded(n) = gen.resume() {
          println!("Got value {}", n);
      }
  
      if let GeneratorState::Yielded(n) = gen2.resume() {
          println!("Got value {}", n);
      }
  
      std::mem::swap(&mut gen, &mut gen2); // <--- Big problem!
      
      // This would now start gen2 since we swapped them.
      if let GeneratorState::Complete(()) = gen.resume() {
          ()
      };
}
  ```

  运行起来之后就出现了执行gen2.resume()（这里是gen2是因为已经经过了swap了，即这个时候gen2就是gen，所以）的时候没办法打印出Hello的情况：

  ![image-20240531214549325](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240531214549325.png)

  而且就是在print的时候跳转就出现了问题。所以大概率是因为borrowed这个引用经过swap之后出现了一点问题，导致引用变量的地址不正确了。这样就在安全的rust中引发了一个内存相关的问题，这个就与rust的设计理念背道而驰了（在安全的rust中绝对不容忍内存相关的问题，因为编译器都做了相关的检查）这里作者就埋了一个坑，说下一章会介绍为什么会出现这个问题（并且能通过Pin来解决）。

  这里出现问题应该是因为自指结构体，因为在gen.resume的时候，已经gen的值就变成了Yield1，而gen2的值是Enter，然后执行了swap之后，Yield1中的引用变量仍然是指向原来的位置，而原来的位置中是Enter，并没有Yield1中的字符串成员，所以会出现地址无法访问的问题。而如果把swap放在gen.resume和gen2.resume之后，这个时候就会因为两个gen都是Yield1，这个时候虽然结构体中的引用指向的还是原来的位置，但是还是能找到相应的值。
  
  所以出现问题的关键就是，自指结构体在移动的时候其中的引用变量仍然指向的是原来的位置，而该位置的结构体成员变量已经随着结构体的移动被移走了。即：
  
  ![image-20240601104215295](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240601104215295.png)
  
  这里岔开讲一句。这里我走进swap函数发现这里的传参跟ARM的规范很像：

  > 1. `55555555C6CA: 48 8D 7C 24 58 leaq 0x58(%rsp), %rdi`
>
  >    这条指令将栈指针（%rsp）加上偏移量（0x58）的结果加载到寄存器 %rdi 中。这通常用于准备函数调用，因为在 x86-64 架构的 System V ABI 中，%rdi 用于传递第一个参数。
>
  > 2. `55555555C6CF: 48 8D 74 24 78 leaq 0x78(%rsp), %rsi`
  >
  >    这条指令将栈指针（%rsp）加上偏移量（0x78）的结果加载到寄存器 %rsi 中。这同样可能用于准备函数调用，因为在 x86-64 架构的 System V ABI 中，%rsi 用于传递第二个参数。

  这里也是通过寄存器进行传参的。

  **解决**poll函数还有resume函数是不是就是一个异步块或者生成器的状态机？——resume函数就是一个状态机

  如果解决了上面那个问题，那么就相当于是实现了一个async了。因为async本质上也是生成一个类似的状态机，并且能够跨yield点进行引用（这也是上面一直在做的事情）。

##### Async and generators

这一节就是在说async和生成器是很像的

- 一个生成器：

  ```rust
  let mut gen = move || {
          let to_borrow = String::from("Hello");
          let borrowed = &to_borrow;
          yield borrowed.len();
          println!("{} world!", borrowed);
      };
  ```

- 一个async

  ```rust
  let mut fut = async {
          let to_borrow = String::from("Hello");
          let borrowed = &to_borrow;
          SomeResource::some_task().await;
          println!("{} world!", borrowed);
      };
  ```

这么一看就好像是把yield替换为了await，生成器替换为了async。他们的实现方式都是一样的（唯一的区别应该就是上面提到的几个无栈协程的优点，尤其是借用）

- 从生成器到async
  - 生成器调用的函数是 `Generator::resume` ，而在async中调用的函数是`Future::poll`（这个区别很大，因为resume函数中是不阻塞的，那么在poll块中要怎么处理阻塞事件？感觉这里就不阻塞的，如果当前的fut还没有返回的话，马上就返回pend，而不是阻塞，这也是一个递归的过程，就是下面的fut返回上来之后上面的fut就自然而然的返回了。**疑问**那么在叶子fut中是什么样的？他是如何判断当前事务是否已经执行结束然后相应地返回pend或者ready的？所以最关键的是leaf节点的线程执行是什么样的。**猜测**猜测叶子结点的poll也是不阻塞的，只不过是根据一个阻塞函数的非阻塞形式去决定返回的是pend还是ready）
  - 生成器的返回值为 `Yielded` or `Complete` ，而在async中返回值为 `Pending` or `Ready`. 
  - 在生成器中使用 `yield` 关键字，而在async块中使用的是`await`关键字。

#### 六 Pin

##### Definitions

- Pin本质：Pin本质上就是一个**智能指针**，只不过能给他指向的数据提供一些保证。他用来管理实现了!Unpin的类型的一些规则（说白了就是管理!Unpin）

- 作者有点小俏皮：

  > *This naming scheme is one of Rust's safety features where it deliberately tests if you're too tired to safely implement a type with this marker. If you're starting to get confused, or even angry, by `!Unpin` it's a good sign that it's time to lay down the work and start over tomorrow with a fresh mind.*

  说是!Unpin能用来判断我是不是太累了。。。

作者在后面也说道了使用Unpin和!Unpin是有一定原因的，因为这个命名能体现出来一些rust语言的小细节，所以后面就继续使用!Unpin这样双重否定的名字了

##### Pinning and self-referential structs

这里作者给出了一个自指更简单的例子（我在这里面加上了一些注释）：

```rust
use std::pin::Pin;

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
}

impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
        }
    }
	
    //构建自指（解决，这里为什么不用去解引用？这是因为这里自动解引用了）
    //在之前是因为在match块中，内部的self就是引用了
    fn init(&mut self) {
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }

    fn a(&self) -> &str {
        &self.a
    }

    fn b(&self) -> &String {
        unsafe {&*(self.b)}
    }
}
```

- 关于自动解引用：这里是进行了自动解引用的，但是在resume的match中就没有进行自动解引用

  这里rust进行了自动解引用而在resume的match中没有进行自动解引用是因为：

  > 然而，在 `resume` 方法中，你使用了 `match self`，这实际上是在对 `self` 的引用进行模式匹配，而不是对 `self` 本身进行模式匹配。在这种情况下，Rust 不会自动进行解引用。

  所以：

  > 总的来说，Rust 是否会自动解引用 `self` 取决于你如何使用 `self`。**如果你直接访问 `self` 的成员，Rust 会自动进行解引用。如果你在模式匹配中使用 `self`，Rust 不会自动进行解引用**。

- 自指的move异常原因

  如果对自指结构体进行swap的话，就会出现下面的问题：

  ![image-20240601110025569](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240601110025569.png)

  而期望输出应该是：

  ```
  a: test1, b: test1
  a: test1, b: test1
  ```

  这产生这个问题的原因就是自指结构体移动的时候，其内部的引用对象并没有发生变化，也就是上面出现过的那张图：

  ![image-20240601110131452](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240601110131452.png)

  作者的解释为：

  > The pointer to `test2.b` still points to the old location which is inside `test1` now. The struct is not self-referential anymore, it holds a pointer to a field in a different object. That means we can't rely on the lifetime of `test2.b` to be tied to the lifetime of `test2` anymore.

  大致意思就是移动自指结构体的时候其中的引用还是指向原来的位置，所以经过移动之后就不再是一个自指结构体了，而这也导致了我们**没办法通过这种方式去规定自指结构体中的引用的生命周期**。并且这个问题很容易导致段错误（也就是通过解引用悬垂指针产生段错误，经典C语言报错）

  作者在这里也给出了一张图片便于理解：

  ![image-20240601110854111](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240601110854111.png)

  跟我上面画的那张图是完全一致的

##### Pinning to the stack

这里就开始介绍Pin了。Pin的一个重要的作用就是能解决上面的自指结构体的移动问题

给出下面的代码：

```rust
use std::pin::Pin;
use std::marker::PhantomPinned;

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned, // This makes our type `!Unpin`
        }
    }
    fn init<'a>(self: Pin<&'a mut Self>) {
        let self_ptr: *const String = &self.a;
        let this = unsafe { self.get_unchecked_mut() };
        this.b = self_ptr;
    }

    fn a<'a>(self: Pin<&'a Self>) -> &'a str {
        &self.get_ref().a
    }

    fn b<'a>(self: Pin<&'a Self>) -> &'a String {
        unsafe { &*(self.b) }
    }
}
```

可以发现跟之前的代码的区别就是多了一个PhantomPinned，然后在所有的方法中都加上了生命周期变量。

有两句话：

> what we've done here is pinning an object to the stack. That will always be `unsafe` if our type implements `!Unpin`.

作者说这里使用了Pin就相当于是把对象**钉**在了栈上（就不让这个栈上的值移动了），并且!Unpin类型总是不安全的。

接下来作者是在两行跑printf之间加上了代码：

```rust
std::mem::swap(test1.get_mut(), test2.get_mut());
```

从这里看他确实报错了：

![image-20240601113911115](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240601113911115.png)

这里的报错原因是因为：

> 但是，这需要`Test`实现 `Unpin` trait，因为 `get_mut` 方法可能会改变内部值的内存地址。如果 `MyStruct` 没有实现 `Unpin` trait，你可以使用 `Pin` 的 `map` 方法来安全地访问内部值的字段。
>
> 对于 `Pin` 类型，`as_mut` 和 `get_mut` 方法有特殊的含义。`Pin::as_mut` 方法返回一个 `Pin<&mut T>`，它仍然保持 `Pin` 的不可移动性。`Pin::get_mut` 方法返回一个 `&mut T`，但这需要 `T` 实现 `Unpin` trait，因为返回的可变引用可能会被用来移动值。

所以对一个Pin变量使用get_mut是不正确的（这里报错并不是因为swap，swap本质上就是将两个地址中的内容重写了，出现问题主要是因为交换的两个地址指向的是自指对象，所以这个时候使用swap就会导致一些问题，而通过固定变量的位置或者移动之后将自指成员重写都是可以解决这个问题的，只不过rust选择的方法是固定位置）

但是如果我使用下面的代码：

```rust
std::mem::swap(&mut test1, &mut test2);
```

就不会报错，并且能正确执行：

![image-20240601113959963](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240601113959963.png)

这里是因为test1与test2已经是一个Pin智能指针了，如果使用的是&mut test1, &mut test2传参的话，就会直接交换两个Pin的值（相当于是交换指向结构体的指针，而结构体本身的值并没有被移动），而如果使用的是test1.get_mut(), test2.get_mut()的话，就会去尝试移动Pin中包裹的结构体的值。**用C语言的话来说，&mut test1, &mut test2是改变一级指针的内容，没有改变变量的内容；而test1.get_mut(), test2.get_mut()是改变变量的内容**。而我们在这里想要的是移动结构体在栈上的位置，所以这里应该使用的传参应该是作者指出的test1.get_mut(), test2.get_mut()。下面的图片也验证了这一点（即&mut test1是Pin的引用，是一个二级指针）：

![image-20240601154000393](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240601154000393.png)

- Pin的作用：从这里的报错以及Pin这个单词的意思可以大概直到，Pin这个智能指针的作用就是让其所指向的变量不可被移动（也就是钉在某个地方，可以是栈上，也可以是堆上）

- 栈Pin的限制（或者说易错点）

  其实就是正常的变量声明周期，只不过Pin这个东西影响比较大（Pin如果被销毁的话直接影响内部变量是否能移动，进而可能导致内存相关的问题），所以这里要强调一下

  作者在这里有提到：

  > It's important to note that stack pinning will always depend on the current stack frame we're in, so we can't create a self referential object in one stack frame and return it since any pointers we take to "self" are invalidated.

  大致意思就是，栈pin通常是依赖于当前的函数栈帧的（或者说生命周期是依赖于当前的函数栈帧的）。所以我们不能在一个函数中创建了一个自指对象，然后把他Pin在栈上，然后再返回这个自指对象。因为在返回自指对象的时候他的Pin已经被自动drop掉了（即在退出函数的时候自指对象已经是unpin的状态了）。就像下面这段代码一样：

  ```rust
  #[allow(dead_code)]
  pub fn limof_stack_pin() {
      let mut test1 = Test::new("test1");
      let mut test1_pin = unsafe { Pin::new_unchecked(&mut test1) };
      Test::init(test1_pin.as_mut());
      drop(test1_pin);    // simulate to exit the func frame
      
      let mut test2 = Test::new("test2");
      std:: mem::swap(&mut test1, &mut test2);
      println!("Not self referential anymore: {:?}", test1.b);
  }
  ```

  上图是在模拟返回一个被pin的结构体变量的，这个时候pin会被drop，但是结构体变量会被传出函数栈帧

  这个时候在进行swap时test1就是unpin的状态了，所以这个时候swap函数是能正常执行的。

  这个也是非常合理的，因为在使用栈Pin的时候如果在函数退出的时候还保留Pin的话，那么这个被这个PinPin住的对象在退出栈的时候就逃逸出了当前函数（相当于是被移动了），这个时候就大概率会报错

##### Pinning to the heap

堆Pin实际上就是将变量放在堆上然后再Pin住，所以堆pin和栈pin的唯一的区别就是在堆pin的时候需要将结构体变量同构Box分配在堆上，然后再Pin住

给出下面的代码：

```rust
use std::pin::Pin;
use std::marker::PhantomPinned;

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Test {
    fn new(txt: &str) -> Pin<Box<Self>> {
        let t = Test {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        };
        let mut boxed = Box::pin(t);
        let self_ptr: *const String = &boxed.as_ref().a;
        unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };

        boxed
    }

    fn a<'a>(self: Pin<&'a Self>) -> &'a str {
        &self.get_ref().a
    }

    fn b<'a>(self: Pin<&'a Self>) -> &'a String {
        unsafe { &*(self.b) }
    }
}

pub fn main() {
    let mut test1 = Test::new("test1");
    let mut test2 = Test::new("test2");

    println!("a: {}, b: {}", test1.as_ref().a(), test1.as_ref().b());
    println!("a: {}, b: {}", test2.as_ref().a(), test2.as_ref().b());
}
```

从上面的代码中也能看出，堆pin其实就比栈pin多了一行代码：

```rust
let mut boxed = Box::pin(t);
```

这里通过Box::pin()方法就创建了一个Pin<Box<>\>对象，这样的话就相当于是把Box指向的对象Pin在了堆上。

这里看着好像是Pin的对象是一个Box智能指针而不是Box指向的对象，但是实际上Box在其中**自动解引用**了，所以Pin住的还是实际的堆上变量，而不是Box指针本身。

同样的，Pin智能指针也像其他的智能指针一样实现了自动解引用（当然跟其他的智能指针一样，有的时候会解引用有的时候不会，汗流浃背了），所以可以这样使用一个Pin<Box<>\>：

```rust
let test1 = Test::new("test1");	//Pin<Box<>>
println!("a: {}, b: {}", test1.a, test1.b);
```

这个时候Pin和Box都会自动解引用。

另外需要注意的是，get_mut接收的参数类型为Pin<mut <T\>\>，所以不能直接对Pin<Box<>\>使用get_mut，需要先使用as_mut将Pin<Box<T\>>转换为Pin<mut <T\>\>（在这个过程中也出现了Box的自动解引用，也就是对Box<T\>进行处理时进行了自动解引用，就使得相当于是直接对T进行处理，最终返回的就是mut T，而不是mut Box<T\>），然后才能尝试使用get_mut。

对堆Pin进行swap操作如下：

![image-20240601155157126](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240601155157126.png)

与栈Pin同样是编译都没办法通过的情况

到这里其实就能理解这个Pin是怎么实现的了，实际上就是将所有可能改变自指对象的引用的方法全部都加上一个限制条件——实现了unpin，这样就不会出现上面的内存相关的问题了（只要不移动，就没有问题，可以的）

通过Pin，就完全不需要考虑自指结构体的细节了（因为自指对象已经通过pin被固定了）

有一句话看不太明白**疑问**：

> The fact that it's safe to pin heap allocated data even if it is `!Unpin` makes sense. Once the data is allocated on the heap it will have a stable address.

copilot给出的解释是：

> 这句话的意思是，即使数据类型没有实现 `Unpin` trait，将其分配到堆上并用 `Pin` 包装也是安全的。这是因为一旦数据被分配到堆上，它的内存地址就是稳定的。

即：

> 当我们说 "堆上的数据的内存地址是稳定的"，我们是指数据本身在内存中的位置不会改变。例如，如果你创建了一个 `Box<T>`，那么 `Box` 所指向的 `T` 类型的数据在内存中的位置是固定的，除非你显式地移动它（例如，通过调用 `std::mem::swap`）。

这个意思是，如果要交换堆上的数据没必要交换变量本身，只交换变量的智能指针就好了，这样并不会修改变量存储的地址，进而就不违背Pin的规则了

##### Practical rules for Pinning

- If `T: Unpin` (which is the default), then `Pin<'a, T>` is entirely equivalent to `&'a mut T`. in other words: `Unpin` means it's OK for this type to be moved even when pinned, so `Pin` will have no effect on such a type.

  即如果一个对象实现了UnPin，然后再把这个对象Pin住（即得到一个Pin<'a, T>类型的对象），那么这个在这个情况下，Pin<'a, T>与&'a mut T是没有区别的，因为这个对象实现了UnPin，就相当于是允许这个对象在被Pin的状态下移动位置（也就是&'a mut T）

- Getting a `&mut T` to a pinned T requires unsafe if `T: !Unpin`. In other words: requiring a pinned pointer to a type which is `!Unpin` prevents the *user* of that API from moving that value unless they choose to write `unsafe` code.

  意思是如果一个变量已经被Pin住了，那么就没有办法移动这个变量，除非使用unsafe（使用裸指针来移动这个变量，或者移动更关键的东西——自指指针）。如下面这段代码：

  ```rust
  let this = unsafe { self.get_unchecked_mut() };
  ```

  此时this就是一个Pin变量的可变引用了

- Pinning does nothing special with memory allocation like putting it into some "read only" memory or anything fancy. It only uses the type system to prevent certain operations on this value.

  意思是Pin没有将数据放到只读区（没办法写入，也就是没办法移动了），他只是通过Pin这个类型来防止某些操作

- Most standard library types implement `Unpin`. The same goes for most "normal" types you encounter in Rust. `Future`s and `Generator`s are two exceptions.

  大多数类型都实现了unpin，fut和生成器是两个例外（是因为他们需要保存现场，而为了保存现场又需要自指结构体，有了自指结构体就必须Pin住了）

  fut和生成器经过编译之后的变量实际上就是自动机状态（想想上一章），而自动机的状态中保存了现场，可能出现自指结构体，所以需要Pin

- The main use case for `Pin` is to allow self referential types, the whole justification for stabilizing them was to allow that.

  Pin存在的一个主要原因就是为了支持自指类型

  所以自己平常很少用是合理的

- The implementation behind objects that are `!Unpin` is most likely unsafe. Moving such a type after it has been pinned can cause the universe to crash. As of the time of writing this book, creating and reading fields of a self referential struct still requires `unsafe` (the only way to do it is to create a struct containing raw pointers to itself).

  主要意思就是大部分对Pin变量的操作都是不安全的。因为实现一个自指对象的方式就是通过裸指针来实现

- You can add a `!Unpin` bound on a type on nightly with a feature flag, or by adding `std::marker::PhantomPinned` to your type on stable.

  这里是在介绍如何实现一个!Unpin。首先需要知道的是，Unpin虽然是一个trait，倒不如说是一个标记，因为他没有要求任何方法：

  ```rust
  pub auto trait Unpin {}
  ```

  并且这里能看出Unpin是一个auto trait，也就是说这个trait可以**由编译器自动导出**（如果成员都实现了unpin，那么当前复合类型也会自动实现unpin。对于基本类型直接实现unpin）

  在同一个文件中，还能看见一个结构体：

  ```rust
  pub struct PhantomPinned;
  
  impl !Unpin for PhantomPinned {}
  ```

  这个就是在自指结构体中额外增加的成员（结构体没有内容不占空间，只起到标记作用）

  如果在一个结构体中引入了PhantomPinned成员，那么这个结构体就会因为有一个成员PhantomPinned没有实现unpin而使得整个结构体都没有实现unpin。

  这也就是作者所说的：让一个变量类型实现!unpin这个trait有两种方式，一个是使用feature flag，如下：

  ```rust
  //使用!Unpin的特性标志
  #![feature(negative_impls)]
  ```

  加上了这个feature之后就能这样写：

  ```rust
  impl !Unpin for GeneratorA { }
  ```

  另一个方法就是在需要!unpin的结构体中增加一个成员PhantomPinned（这个的实现逻辑就是上面所说到的）。

- You can either pin an object to the stack or to the heap.

  可以进行栈pin也可以进行堆pin

- Pinning a `!Unpin` object to the stack requires `unsafe`

  进行栈pin的时候需要unsafe，如：

  ```rust
  let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };	//test1 is Pin<&mut Test>
  ```

- Pinning a `!Unpin` object to the heap does not require `unsafe`. There is a shortcut for doing this using `Box::pin`.

  进行堆pin的时候不需要unsafe，而是可以直接使用方法Box::pin，如：

  ```rust
  let mut boxed = Box::pin(t);	//boxed is Pin<Box<Test>>
  ```

需要注意的一点是，上面所说的移动其实真正的操作就是写入操作，所以上面说的所有的**不允许移动**，都相当于是**不允许写入**。换一种说法就是，如果一个变量被Pin住了，并且没有实现unpin，那么就无法获取到这个变量的可变引用（就是这个变量没办法修改）

谁还记得这玩意是从生成器那边过来的。。。最开始讲的是编译生成器重写代码时需要自指结构体，然后就到了Pin

这么看自己好像很少使用Pin？因为这个是编译器生成代码的时候做的事情，编译器在生成代码的时候可能已经将自指结构体Pin住了，并且要修改值的时候通过unsafe实现。另外编译器显然也不会莫名其妙地将两个生成器的状态进行修改（这里指的是除了自动机修改以外的修改），除非我在我的源代码中尝试swap两个fut或者两个生成器（这样编译之后就会有对应的代码来交换了，这个时候显然编译都不会通过）

还有一个自己很少用Pin的原因就是自己很少写出来自指对象（Pin的主要目的就是用于自指对象）

##### Projection/structural pinning

原文比较短，直接截出来：

> In short, projection is a programming language term. `mystruct.field1` is a projection. Structural pinning is using `Pin` on fields. This has several caveats and is not something you'll normally see so I refer to the documentation for that.

这里就是在说投影和结构化pining，大致意思就是我可以对一个结构体成员Pin，这个操作就被称为结构化Pining

##### Pin and Drop

原文也比较短，这里也直接截出来：

> The `Pin` guarantee exists from the moment the value is pinned until it's dropped. In the `Drop` implementation you take a mutable reference to `self`, which means extra care must be taken when implementing `Drop` for pinned types.

这个意思是Pin会一直生效直到被Pin的值被drop掉（当然栈pin还需要考虑Pin本身的生命周期。但是总而言之，Pin的生命周期不会大于变量本身）。并且分的更细一点，应该是在变量本身被drop之前就会先将Pin drop掉（这也是合理的，指针不先drop的话不就出现悬垂指针了吗）。

既然在变量本身的drop之前该变量的pin已经被drop掉了，那么在变量本身的drop中就能够获取变量本身的可变引用了（就是可以移动变量了），所以在变量本身的drop函数中还需要额外考虑Pin之前已经帮我们考虑了的问题

在这一章的最后，作者提供了一个生成器的完整代码，与上面的区别就是：

```rust
impl !Unpin for GeneratorA { }
```

然后将GeneratorA封装成一个堆pin：

```rust
let mut pinned1 = Box::pin(gen1);
let mut pinned2 = Box::pin(gen2);
```

在resume中要对pin进行修改，就只能使用unsafe：

```rust
fn resume(self: Pin<&mut Self>) -> GeneratorState<Self::Yield, Self::Return> {
    // lets us get ownership over current state
    let this = unsafe { self.get_unchecked_mut() };
    match this {
    GeneratorA::Enter => {
    let to_borrow = String::from("Hello");
    let borrowed = &to_borrow;
    let res = borrowed.len();
    *this = GeneratorA::Yield1 {to_borrow, borrowed: std::ptr::null()};

    // Trick to actually get a self reference. We can't reference
    // the `String` earlier since these references will point to the
    // location in this stack frame which will not be valid anymore
    // when this function returns.
    if let GeneratorA::Yield1 {to_borrow, borrowed} = this {
    *borrowed = to_borrow;
    }

    GeneratorState::Yielded(res)
    }

    GeneratorA::Yield1 {borrowed, ..} => {
    let borrowed: &String = unsafe {&**borrowed};
    println!("{} world", borrowed);
    *this = GeneratorA::Exit;
    GeneratorState::Complete(())
    }
    GeneratorA::Exit => panic!("Can't advance an exited generator!"),
    }
}
```

作者在这里也提到了，如果改变pin的值而不是变量本身的值是不会有问题的：

```rust
// This won't work:
// std::mem::swap(&mut gen, &mut gen2);			//通过一级指针交换被pin变量
// This will work but will just swap the pointers so nothing bad happens here:
// std::mem::swap(&mut pinned1, &mut pinned2);	//通过二级指针交换一级指针
```

从这里可以看出，与我上面的猜测一致：

> 这么看自己好像很少使用Pin？因为这个是编译器生成代码的时候做的事情，编译器在生成代码的时候可能已经将自指结构体Pin住了，并且要修改值的时候通过unsafe实现。另外编译器显然也不会莫名其妙地将两个生成器的状态进行修改（这里指的是除了自动机修改以外的修改），除非我在我的源代码中尝试swap两个fut或者两个生成器（这样编译之后就会有对应的代码来交换了，这个时候显然编译都不会通过）

这里就是将生成器对象（或者说生成器状态机状态变量）Pin住，然后在resume函数中使用unsafe来修改生成器的状态。

有一段话：

> Now, as you see, the consumer of this API must either:
>
> 1. Box the value and thereby allocating it on the heap
> 2. Use `unsafe` and pin the value to the stack. The user knows that if they move the value afterwards it will violate the guarantee they promise to uphold when they did their unsafe implementation.

意思是用户要不然就通过Box将生成器或者fut放在堆上（这样他在进行swap的时候只需要交换智能指针的值即可，并不用移动数据本身。当然还是不能显式使用swap强行移动堆上的变量），要不然就是进行一个栈pin（栈上交换数据并不能像堆上一样直接交换变量的指针，因为栈上变量没有指针，这个时候交换就是不允许的，只能使用unsafe来强行修改栈上的被pin变量）

但无论如何，编译器都会将变量本身pin住。如果想要实现类似变量交换的操作，对堆上的数据就交换指针（也就是交换Pin<Box<T\>>中的Box），而栈上数据就只能使用unsafe块来强行修改了（也就是修改Pin<T\>中的T）

#### 七 Implementing Futures

这一章主要是作者带着看明白200行是在干嘛。这章看完之后就要去自己跑一下200行了

在这一章中就可以利用前面的前置知识来实现自己的fut了。作者在这里做的是一个假的Reactor还有一个比较简陋的执行器。

这里学到了一条指令：

```shell
cargo init
```

这条指令可以将当前的文件夹初始化为一个cargo包，也就是会为我自动创建一个Cargo.toml文件，一个src目录，还有一个main.rs文件

##### The Executor

- 执行器的功能：选一个ready（或者刚刚交给执行器的fut）的fut，然后开始poll

- 这里有一段话：

  > **When polled one of three things can happen:**
  >
  > - The future returns `Ready` and we schedule whatever chained operations to run
  > - The future hasn't been polled before so we pass it a `Waker` and suspend it
  > - The futures has been polled before but is not ready and returns `Pending`

  这里还没有很理解，之后结合代码应该能看懂，**疑问**

下面就是执行器的实现

执行器最重要的功能就是轮询并且执行poll函数。如果执行器没有将所有的任务都poll完的话，当前线程就需要阻塞。所以执行器的实现如下：

```rust
// Our executor takes any object which implements the `Future` trait
fn block_on<F: Future>(mut future: F) -> F::Output {

    // the first thing we do is to construct a `Waker` which we'll pass on to
    // the `reactor` so it can wake us up when an event is ready.
    let mywaker = Arc::new(MyWaker{ thread: thread::current() });
    let waker = mywaker_into_waker(Arc::into_raw(mywaker));

    // The context struct is just a wrapper for a `Waker` object. Maybe in the
    // future this will do more, but right now it's just a wrapper.
    let mut cx = Context::from_waker(&waker);

    // So, since we run this on one thread and run one future to completion
    // we can pin the `Future` to the stack. This is unsafe, but saves an
    // allocation. We could `Box::pin` it too if we wanted. This is however
    // safe since we shadow `future` so it can't be accessed again and will
    // not move until it's dropped.
    let mut future = unsafe { Pin::new_unchecked(&mut future) };

    // We poll in a loop, but it's not a busy loop. It will only run when
    // an event occurs, or a thread has a "spurious wakeup" (an unexpected wakeup
    // that can happen for no good reason).
    let val = loop {
        match Future::poll(future.as_mut(), &mut cx) {

            // when the Future is ready we're finished
            Poll::Ready(val) => break val,

            // If we get a `pending` future we just go to sleep...
            Poll::Pending => thread::park(),
        };
    };
    val
}
```

作者也指出了上面的注释很重要，并且在下面的文字中不会再重复说了，只会提一些重要的点

这个就是最重要的执行器函数了（正如上面一堆图片那一章所见的一样，首先执行的是block_on函数，然后在这里面就执行了很多的关于异步的操作。说白了就是block是在轮询之前的fut）。在这个函数中实现的逻辑就是执行一个fut，如果是ready状态的话，当前线程就能继续执行；如果还是Pending状态的话，执行器线程就需要休眠（也就是thread::park()）。

这实际上就是一个简单的执行器了，只不过因为当前系统中只有一个fut，所以这个时候只要看这个fut是什么状态即可。如果是Pending状态的话就表示这个fut还在阻塞队列中，所以执行器需要休眠（这里**并不是忙等状态**）；而如果是Ready状态的话，执行器线程就能继续向下走了。

从这里也能大概想一下如果有多个fut的话，那么这个时候要先修改一下传参（传一个数组或者向量都是可以的），然后会多很多的match块，并且前面的match块如果检查到了是Pending状态的话不能直接调用thread::park()，而是只有当所有的fut都在Pending的时候才需要进行thread::park()，所以又需要一个额外的标志位

- 关于thread::park：这个函数会将当前线程挂起，直到有其他的线程调用unpark来唤醒当前线程（对执行器而言，这个就是Reactor要通过waker干的事了。waker的wake方法唤醒的是执行器，而不是一个fut）

- 关于Context

  Context在之前讲waker的时候并没有详细介绍过，只是简单说了这个东西就是用来包裹一个waker的。这里作者给出了一点点额外的解释：

  > `Context` is just a wrapper around the `Waker`. At the time of writing this book it's nothing more. In the future it might be possible that the `Context` object will do more than just wrapping a `Waker` so having this extra abstraction gives some flexibility.

##### The Future implementation

fut实现的代码如下：

```rust
// This is the definition of our `Waker`. We use a regular thread-handle here.
// It works but it's not a good solution. It's easy to fix though, I'll explain
// after this code snippet.、
// waker的工作是唤醒执行器，所以这里作者简单粗暴，直接在wake里面放了一个代表执行器的线程
// (这里还没有带上虚函数表，只是单单定义一下唤醒器的数据段)
#[derive(Clone)]
struct MyWaker {
    thread: thread::Thread,
}

// This is the definition of our `Future`. It keeps all the information we
// need. This one holds a reference to our `reactor`, that's just to make
// this example as easy as possible. It doesn't need to hold a reference to
// the whole reactor, but it needs to be able to register itself with the
// reactor.
#[derive(Clone)]
pub struct Task {
    id: usize,
    reactor: Arc<Mutex<Box<Reactor>>>,
    data: u64,
}

// These are function definitions we'll use for our waker. Remember the
// "Trait Objects" chapter earlier.
// 定义了唤醒器最重要的方法：wake。这里能看出实际上就是拿到数据段，然后获取里面的线程
// 然后对他进行unpark，就唤醒了执行器
fn mywaker_wake(s: &MyWaker) {
    let waker_arc = unsafe {Arc::from_raw(s)};
    waker_arc.thread.unpark();
}

// Since we use an `Arc` cloning is just increasing the refcount on the smart
// pointer.
// 这个方法是实现了waker的clone.需要注意的是，这里并不是单单拷贝数据段，还拷贝了虚表
fn mywaker_clone(s: &MyWaker) -> RawWaker {
    let arc = unsafe { Arc::from_raw(s) };
    // forget方法能够阻止退出作用域时drop的执行，所以这里就可以人为实现唤醒器的引用计数增加操作
    // 这里的引用计数实际上是对数据段MyWaker的引用计数。当然也可以理解为对整个唤醒器的引用计数，
    // 因为数据段的引用计数与整个trait对象的引用计数是恒等的
    std::mem::forget(arc.clone()); // increase ref count
    // 返回的是一个裸指针，所以arc在退出的时候会被drop，所以要使用forget防止drop
    RawWaker::new(Arc::into_raw(arc) as *const (), &VTABLE)
}

// This is actually a "helper funtcion" to create a `Waker` vtable. In contrast
// to when we created a `Trait Object` from scratch we don't need to concern
// ourselves with the actual layout of the `vtable` and only provide a fixed
// set of functions
// 这个就是一个便于创建虚函数表的方法。需要注意的是，由于这里的wake、clone等方法都是
// 数据段结构体的方法，所以在经过编译之后这些方法都会传入数据段的变量引用，所以s在编译
// 之后就知道是什么类型的了，并且参数的传递是由编译器完成的，我们在这里只需要传递一个函数闭包就好了
const VTABLE: RawWakerVTable = unsafe {
    RawWakerVTable::new(
        |s| mywaker_clone(&*(s as *const MyWaker)),   // clone
        |s| mywaker_wake(&*(s as *const MyWaker)),    // wake
        |s| (*(s as *const MyWaker)).thread.unpark(), // wake by ref (don't decrease refcount)
        // 当drop一个唤醒器的时候就需要修改数据段的引用计数了
        // 这里的引用计数对象要跟上面clone的相同。上面是对MyWaker的引用计数
        // 那么这就要减少对MyWaker的引用计数.而减少引用计数就直接使用drop方法
        // 来drop一个指向MyWaker的arc指针即可
        |s| drop(Arc::from_raw(s as *const MyWaker)), // decrease refcount
    )
};

// Instead of implementing this on the `MyWaker` object in `impl Mywaker...` we
// just use this pattern instead since it saves us some lines of code.
// 这里作者也提到了唤醒器的实现模式和直接给结构体实现方法这两者。但是作者在这里只是说会省一点空间
// 但实际上好像确实只有这一个优点了，之前在第四章中所区别的好像都是错误的。。。
fn mywaker_into_waker(s: *const MyWaker) -> Waker {
    let raw_waker = RawWaker::new(s as *const (), &VTABLE);
    unsafe { Waker::from_raw(raw_waker) }
}

impl Task {
    fn new(reactor: Arc<Mutex<Box<Reactor>>>, data: u64, id: usize) -> Self {
        Task { id, reactor, data }
    }
}

// This is our `Future` implementation
impl Future for Task {
    type Output = usize;

    // Poll is the what drives the state machine forward and it's the only
    // method we'll need to call to drive futures to completion.
    // 看样子fut这个trait只是要求了poll函数
   	// 由于这个函数会返回fut的状态，所以这个task应该是一个叶子fut.
    // 因为非叶子fut的poll函数都由编译器生成，并且这些非叶子fut的返回值是直接返回叶子fut的返回值实现的
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // We need to get access the reactor in our `poll` method so we acquire
        // a lock on that.
        let mut r = self.reactor.lock().unwrap();

        // First we check if the task is marked as ready
        // 这个is_ready方法是由Reactor提供的，所以将任务的状态置为ready是由Reactor决定的
        // 当当前fut等待的事件发生时is_ready函数就会返回true。对叶子fut而言就是其本身代表的事件
        // 所以这里传递的是叶子结点本身的id
        // 那么这里最关键的又不是叶子fut如何实现的了，而是Reactor如何实现。
        // 因为从这个叶子的poll来看的话，就是纯纯调用Reactor的方法，并没有出现创建线程啥的操作
        // 或者说之前的理解都是错的？并不是由poll函数自身来确定到底是pend还是ready，而是由Reactor决定
        // 感觉确实是之前猜错了
        // 同时在这里并没有看见唤醒相关的步骤，所以猜测是Reactor发现了某个任务是ready之后就自己执行
        // wake函数，并且使得对应的is_ready函数的返回值为true
        if r.is_ready(self.id) {

            // If it's ready we set its state to `Finished`
            // 叶子fut如果是ready的话，那么就一定是finish
            *r.tasks.get_mut(&self.id).unwrap() = TaskState::Finished;
            Poll::Ready(self.id)

        // If it isn't finished we check the map we have stored in our Reactor
        // over id's we have registered and see if it's there
        // 如果任务不是ready状态的话，那么就只有两种可能，一种是任务正在Pending
        // 另一种是这个任务不在Reactor的任务队列中
        } else if r.tasks.contains_key(&self.id) {

            // This is important. The docs says that on multiple calls to poll,
            // only the Waker from the Context passed to the most recent call
            // should be scheduled to receive a wakeup. That's why we insert
            // this waker into the map (which will return the old one which will
            // get dropped) before we return `Pending`.
            // 大致意思就是，唤醒器一定要保持是最新的，所以每次就算是任务Pending状态
            // 都要更新一次唤醒器
            r.tasks.insert(self.id, TaskState::NotReady(cx.waker().clone()));
            Poll::Pending
        } else {
	
            // If it's not ready, and not in the map it's a new task so we
            // register that with the Reactor and return `Pending`
            // 这里就是任务还不在Reactor的任务队列中，这个时候需要在Reactor中注册任务
            // 并返回Pending。这也就是在本章最开始的那三点中的第二点所说的，如果从未被poll过
            // 就需要传递waker并且Pending，因为这个时候这个时候很可能Reactor已经将这个
            // 叶子任务交给一个线程来做了,而这个任务刚刚开始，就一定是pend的状态
            // 就算不是下一次poll的时候也会检测到是ready
            r.register(self.data, cx.waker().clone(), self.id);
            Poll::Pending
        }

        // Note that we're holding a lock on the `Mutex` which protects the
        // Reactor all the way until the end of this scope. This means that
        // even if our task were to complete immidiately, it will not be
        // able to call `wake` while we're in our `Poll` method.
		// 这一段话的意思是，在poll函数中因为我们一直占用着reactor这个互斥变量，那么在其他的
        // 地方就不会再获取到reactor并根据任务的ready状态去唤醒执行器
        
        // Since we can make this guarantee, it's now the Executors job to
        // handle this possible race condition where `Wake` is called after
        // `poll` but before our thread goes to sleep.
        // 需要结合上面那段话来看。这里就结合当前这个例子来看因为在poll函数中占有了reactor
        // 所以wake函数只有可能在返回了Pending之后被调用。如果返回Pending之后没有wake调用，
        // 那么这个系统中唯一的一个fut是Pending状态，那么执行器将要休眠;而如果在返回Pending
       	// 后，执行器休眠之前出现了wake，那么执行器就不应该休眠，而应该继续执行。并且如果执行器
        // 在此时决定要休眠的话，那么这个wake操作就会导致fut其实能继续向下执行，但是执行器已经
        // 无法被唤醒了,因为执行器等待的wake操作已经被执行器自己忽略了，这样就造成了死锁
    }
}
```

同样的，注释很重要。

我也像作者一样直接在代码上直接写注释了，就不在这里另外提了。这样方便边看代码边写笔记

这里作者也提到了为什么很少使用thread park/unpark：

> It could deadlock easily since anyone could get a handle to the `executor thread` and call park/unpark on our thread. I've made [an example with comments on the playground](https://web.archive.org/web/20230202235603/https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=b2343661fe3d271c91c6977ab8e681d0) that showcases how such an error could occur. You can also read a bit more about this in [issue 2010](https://web.archive.org/web/20230202235603/https://github.com/rust-lang/futures-rs/pull/2010) in the futures crate.

大致意思就是任意一个线程都可以随意地获取执行器线程并调用park或者unpark方法。这种不受限制的访问很容易出现死锁（如两个线程连续对执行器线程进行park，或者连续的unpark等）

##### The Reactor

这个应该算是代码的最后一个部分了（上面已经实现了执行器、唤醒器、fut），也是我最想知道的一个部分，因为经过了上面的介绍之后，好像推翻了我之前的一点想法。我一直以为执行器是最重要的函数，因为他要负责执行任务。但到这里就发现，他只是简单地调用fut的poll方法，然后调用Reactor的函数接口。

```rust
// The different states a task can have in this Reactor
// 可以发现这里是把叶子fut的状态机保存在了Reactor中。可以发现fut的执行逻辑就是，所有的fut，都对应了
// 一个状态机，只不过非叶子的状态机是与poll函数绑定的，而叶子的状态机是存储在Reactor中的
enum TaskState {
    Ready,
    NotReady(Waker),
    Finished,
}

// This is a "fake" reactor. It does no real I/O, but that also makes our
// code possible to run in the book and in the playground
struct Reactor {

    // we need some way of registering a Task with the reactor. Normally this
    // would be an "interest" in an I/O event
    // 这两个成员是用来注册的？看英文的意思好像是这样的。先往下看
    // 这么看的话注册的时候就已经将任务交给一个线程来执行了，并且将这个在线程上任务的执行结果抽象为Event
    dispatcher: Sender<Event>,
    handle: Option<JoinHandle<()>>,

    // This is a list of tasks
    // 表示的是当前Reactor中有的任务
    tasks: HashMap<usize, TaskState>,
}

// This represents the Events we can send to our reactor thread. In this
// example it's only a Timeout or a Close event.
#[derive(Debug)]
enum Event {
    Close,
    Timeout(u64, usize),
}

impl Reactor {

    // We choose to return an atomic reference counted, mutex protected, heap
    // allocated `Reactor`. Just to make it easy to explain... No, the reason
    // we do this is:
    //
    // 1. We know that only thread-safe reactors will be created.
    // 2. By heap allocating it we can obtain a reference to a stable address
    // that's not dependent on the stack frame of the function that called `new`
    fn new() -> Arc<Mutex<Box<Self>>> {
        let (tx, rx) = channel::<Event>();
        let reactor = Arc::new(Mutex::new(Box::new(Reactor {
            dispatcher: tx,
            handle: None,
            tasks: HashMap::new(),
        })));

        // Notice that we'll need to use `weak` reference here. If we don't,
        // our `Reactor` will not get `dropped` when our main thread is finished
        // since we're holding internal references to it.

        // Since we're collecting all `JoinHandles` from the threads we spawn
        // and make sure to join them we know that `Reactor` will be alive
        // longer than any reference held by the threads we spawn here.
        let reactor_clone = Arc::downgrade(&reactor);

        // This will be our Reactor-thread. The Reactor-thread will in our case
        // just spawn new threads which will serve as timers for us.
        let handle = thread::spawn(move || {
            let mut handles = vec![];

            // This simulates some I/O resource
            for event in rx {
                println!("REACTOR: {:?}", event);
                let reactor = reactor_clone.clone();
                match event {
                    Event::Close => break,
                    Event::Timeout(duration, id) => {

                        // We spawn a new thread that will serve as a timer
                        // and will call `wake` on the correct `Waker` once
                        // it's done.
                        let event_handle = thread::spawn(move || {
                            thread::sleep(Duration::from_secs(duration));
                            let reactor = reactor.upgrade().unwrap();
                            reactor.lock().map(|mut r| r.wake(id)).unwrap();
                        });
                        handles.push(event_handle);
                    }
                }
            }

            // This is important for us since we need to know that these
            // threads don't live longer than our Reactor-thread. Our
            // Reactor-thread will be joined when `Reactor` gets dropped.
            handles.into_iter().for_each(|handle| handle.join().unwrap());
        });
        reactor.lock().map(|mut r| r.handle = Some(handle)).unwrap();
        reactor
    }

    // The wake function will call wake on the waker for the task with the
    // corresponding id.
    fn wake(&mut self, id: usize) {
        self.tasks.get_mut(&id).map(|state| {

            // No matter what state the task was in we can safely set it
            // to ready at this point. This lets us get ownership over the
            // the data that was there before we replaced it.
            match mem::replace(state, TaskState::Ready) {
                TaskState::NotReady(waker) => waker.wake(),
                TaskState::Finished => panic!("Called 'wake' twice on task: {}", id),
                _ => unreachable!()
            }
        }).unwrap();
    }

    // Register a new task with the reactor. In this particular example
    // we panic if a task with the same id get's registered twice
    fn register(&mut self, duration: u64, waker: Waker, id: usize) {
        if self.tasks.insert(id, TaskState::NotReady(waker)).is_some() {
            panic!("Tried to insert a task with id: '{}', twice!", id);
        }
        self.dispatcher.send(Event::Timeout(duration, id)).unwrap();
    }

    // We simply checks if a task with this id is in the state `TaskState::Ready`
    fn is_ready(&self, id: usize) -> bool {
        self.tasks.get(&id).map(|state| match state {
            TaskState::Ready => true,
            _ => false,
        }).unwrap_or(false)
    }
}

impl Drop for Reactor {
    fn drop(&mut self) {
        // We send a close event to the reactor so it closes down our reactor-thread.
        // If we don't do that we'll end up waiting forever for new events.
        self.dispatcher.send(Event::Close).unwrap();
        self.handle.take().map(|h| h.join().unwrap()).unwrap();
    }
}
```

















**疑问**为什么要avoid thread::park? ——这里有一篇文章 [a proper way to park our thread](https://web.archive.org/web/20230202235603/https://cfsamson.github.io/books-futures-explained/6_future_example.html#bonus-section---a-proper-way-to-park-our-thread)

