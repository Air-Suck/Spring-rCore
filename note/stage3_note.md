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

#### 一 引言

- 提到了：

  > 不需要使用Rust中的 futures或async/await

  之后有时间可以去看看这些内容

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

这一节虽然短，但是没看懂，感觉需要后面的知识，先当成黑盒

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
&i32------------8
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

#### 五 生成器和async/await

