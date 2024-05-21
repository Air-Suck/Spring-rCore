我是把rust安装在当前的用户目录下了（也就是把环境配在当前用户的根目录下了，所以打开config的时候是直接打开：~/.cargo/config）

[关于mut](https://blog.csdn.net/hbuxiaofei/article/details/108471806)

```rust
fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}
```

let实际上是创建了一个不同的变量。在rust中变量和值是绑定的，想要修改的话要不然是使用mut（但是类型要一样），要不然就要重新let一个变量

```rust
const NUMBER:i32 = 3;
fn main() {
    println!("Number {}", NUMBER);
}
```

常量一定要指明类型

![image-20240413104638441](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240413104638441.png)

**在函数签名中，必须声明每个参数的类型**。这是 Rust 设计中一个经过慎重考虑的决定：要求在函数定义中提供类型注解，意味着编译器再也不需要你在代码的其他地方注明类型来指出你的意图。而且，在知道函数需要什么类型后，编译器就能够给出更有用的错误消息

```rust
for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
```

for的写法

```rust
fn is_even(num: i32) -> bool {
    num % 2 == 0
}
```

函数返回值的写法

使用 `return` 关键字和指定值，可从函数中提前返回；但大部分函数**隐式的返回最后的表达式**。这是一个有返回值的函数的例子：

```rust
fn square(num: i32) -> i32 {
    num * num
}
```

这样才能返回，因为表达式才有返回值，但是语句是没有返回值的

即上面的代码与下面的代码等价：

```rust
fn square(num: i32) -> i32 {
    return num * num;
}
```

```rust
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    let your_character='C';//What's your favorite character?
    // Try a letter, try a number, try a special character, try a character
    // from a different language than your own, try an emoji!
    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
```

这里不会出现所有权冲突是因为字符是分配在栈上的，因为字符的大小是确定的。字符串是分配在堆上的这是因为字符串的长度不一定知道，也就是字符串的大小不确定

```rust
  let test="1";
  let test2="1";
```

这样也不会出问题，是因为字符串字面量是固定大小的，后续不能更改。因为字符串字面量是被硬编码进程序的，因为程序总不能变化，所以就导致字面量实际上就是不能变的，既然大小确定的话，就完全可以分配在栈上

> 我们已经见过字符串字面值，即被硬编码进程序里的字符串值。字符串字面值是很方便的，不过它们并不适合使用文本的每一种场景。原因之一就是它们是不可变的。

> 为了演示所有权的规则，我们需要一个比第三章 [“数据类型”](https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html#数据类型) 中讲到的都要复杂的数据类型。前面介绍的类型都是**已知大小的，可以存储在栈中，并且当离开作用域时被移出栈**（字面量也是存储在栈上的），如果代码的另一部分需要在不同的作用域中使用相同的值，可以快速简单地复制它们来创建一个新的独立实例。不过我们需要寻找一个存储在堆上的数据来探索 Rust 是如何知道该在何时清理数据的。
>
> 我们会专注于 `String` 与所有权相关的部分。这些方面也同样适用于标准库提供的或你自己创建的其他复杂数据类型。

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

数组的创建

```rust
let a = [3; 5];
```

创建一个初始值为3，长度为5的数组

“字符串 slice” 的类型声明写作 `&str`

数组slice 的类型是 `&[i32]`

并且数组的slice也是经典包前不包后

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

元组取成员是直接使用.

```rust
   let v = vec![1, 2, 3];
```

这个就是使用宏指令创建了一个包含初始值的vec

这个vec就相当于是cpp中的变长数组了吧，这个只能在运行时确定大小，所以这玩意不能在进入函数时被分配在栈上

![image-20240413132412975](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240413132412975.png)

需要注意，因为这里是创建了向量元素的可变引用（使得向量内部的元素可以改变），所以这个时候变量本身一定要是可变的。所以不难发现，所有返回可变引用的方法一定要使用在可变的变量上

需要注意的是这里返回的是引用（可变是因为mut），所以需要解引用

不可变变量只能赋值一次

```rust
for element in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        *element*=2;
    }
```

所以需要这样写，iter_mut()返回的是引用

迭代器本身是惰性的，v.iter().map方法是返回了一个迭代器，相当于说map方法根据某些规则消费一个迭代器，返回了一个迭代器。要让这个迭代器有用就要去消费这个迭代器，也就是调用collect方法

需要注意的是，map方法就相当于是用后面的替换了前面的，所以不需要像1里面那样去修改元素的值，而是直接写出需要替换成什么即可，也就是注释中说的

> you can just return the new number!

```rust
//1
fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        *element*=2;
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}
//2
fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| {
        // TODO: Do the same thing as above - but instead of mutating the
        // Vec, you can just return the new number!
        element*2
    }).collect()
}
```

```rust
fn main() {
    let vec0 = Vec::new();

    //是可以将可变的vec赋给不可变的vec的
    //这里是因为在函数内部创建了一个新的可变vec，所以能在函数内部修改vec的值，返回的也是一个可变的引用。但是在函数退出的时候使用权就移交给了vec1，但是原来vec1是不可变的，还尝试对其进行push，就出问题了
    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
```

> 可变引用有一个很大的限制：如果你有一个对该变量的可变引用，你就不能再创建对该变量的引用。这些尝试创建两个 `s` 的可变引用的代码会失败

也就是说如果变量有一个可变引用，那么变量本身一定要是可变的，并且只能有一个可变引用。但是如果只需要使用变量的不可变引用的话，那么变量本身应该也是不可变的，并且能创建多个不可变引用。这个感觉上就已经做到了内存保护

> GPT：在Rust中，不可以为一个不可变变量创建可变引用。这是因为Rust的借用规则（Borrowing Rules）确保了内存安全和线程安全。

> 让我们概括一下之前对引用的讨论：
>
> - 在任意给定时间，**要么** 只能有一个可变引用，**要么** 只能有多个不可变引用。
> - 引用必须总是有效的。

```rust
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
```

需要注意的是，函数调用的时候，本质上就是将当前传入的参数拷贝给形参（除非拷贝的是需要传入参数的clone，这个时候就是深拷贝了），而在rust中就相当于是把使用权交给了形参，所以在函数退出的时候，这个参数的值就消失了，除非像上面这样将形参返回并且使用另一个变量来接管这个使用权

```rust
fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0.clone());

    println!("{} has length {}, with contents: `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {}, with contents `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
```

这个时候就相当于是进入函数时只是将vec0.clone()的使用权交给了形式参数

vec.to_vec()这个方法就相当于根据vec创建了一个新的vec

> `to_vec()` 是 Rust 中的一个方法，用于将一个可迭代对象（如数组、切片、字符串等）转换为一个新的 `Vec`。

```rust
fn fill_vec(vec:&mut Vec<i32>) ->Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    *vec
}
```

这样写本质上就是想要通过引用来获取值的使用权，因为这个时候我就相当于是在通过引用将值赋给返回参数，因为引用没有使用权，所以是不能这样做的

> **注意一个引用的作用域从声明的地方开始一直持续到最后一次使用为止**（并不是简单的以大括号为界）。例如，因为最后一次使用不可变引用（`println!`)，发生在声明可变引用之前，所以如下代码是可以编译的：

```rust
let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);
```

```rust
let your_order =Order{
            name: String::from("Hacker in Rust"),
            year: order_template.year,
            made_by_phone: order_template.made_by_phone,
            made_by_mobile: order_template.made_by_mobile,
            made_by_email: order_template.made_by_email,
            item_number: order_template.item_number,
            count: order_template.count,
        }
```

结构体更新语法会创建一个新的结构体，不会出现使用权的问题

因为每次进入函数的时候都会出现将实参拷贝给形参的情况，所以调用函数的时候最好还是传引用

assert函数是断言，如果里面的条件为假的话就会出现断言错误

```rust
 enum IpAddr {
        V4(String),
        V6(String),
    }
```

可以像上面一样，直接在枚举成员中指定类型

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

更加复杂的例子，这里展示了如何在枚举的内部定义一个类似结构体的成员

```rust
enum Message {
    ChangeColor(255, 0, 255),
    Echo(String::from("hello world")),
    Move (Point),
    Quit
}
```

这里就完整的展示了如何在枚举的内部定义一个结构体成员，其中还point是一个结构体

```rust
impl Stat
```

这个是在为结构体定义方法，所以这些方法一定是要指定结构体的，所以才会出现找不到函数的问题

```rust
enum Message {
    ChangeColor((u8, u8, u8)),
    Echo(String),
    Move (Point),
    Quit
}
```

这里展示了如何为一个枚举变量成员绑定一个元组类型。前面的只是给一个成员绑定若干个值

```rust
blue".to_string()
```

toString方法将当前的&str转换为一个String类型。其中&str应该表示的是字符串字面量的类型，而String就是一个可以编辑的（如果是可变变量的话）、分配在堆上的变量

为什么字符串字面量是一个&str

因为字符串字面量是被硬编码进程序的，所以这个本质上就是一个程序的切片，而切片返回的是一个部分引用

> 这里 `s` 的类型是 `&str`：它是一个指向二进制程序特定位置的 slice。这也就是为什么字符串字面值是不可变的；`&str` 是一个不可变引用

&str和&String的区别

> `&str` 和 `&String` 都是字符串的引用类型，但它们有一些区别。
>
> 1. **不可变性：** `&str` 是不可变的引用，而 `&String` 是对 `String` 的不可变引用。这意味着不能通过 `&str` 或 `&String` 修改字符串的内容。但是，可以通过 `&mut String` 来修改 `String` 的内容。
> 2. **所有权关系：** `&str` 是对字符串数据的借用，它不拥有字符串数据的所有权。它可以引用不同来源的字符串，例如字符串字面量或 `String` 对象。相反，`&String` 是对 `String` 对象的引用，它也不拥有 `String` 对象的所有权，但它仅能引用 `String` 对象。
> 3. **转换成字符串：** `&str` 可以通过 `.to_string()` 方法转换为 `String` 对象。这会创建一个新的 `String` 对象，其中包含了 `&str` 引用的字符串内容。相反，`&String` 可以通过 `&*` 运算符转换为 `&str`。这只是将 `&String` 引用的字符串数据作为 `&str` 引用的方式，而没有进行复制或转移。
>
> 总结来说，`&str` 通常用于接收字符串参数或引用字符串数据，而 `&String` 则用于接收 `String` 对象的引用。在需要不可变的字符串引用时，使用 `&str` 更为常见，而在需要引用一个特定的 `String` 对象时，使用 `&String`。

rust是会自动解引用的：[rust自动解引用](https://zhuanlan.zhihu.com/p/554306452)

如果一个东西的类型是S，那么他是能匹配带有参数类型&S的方法的（最少在结构体里面是可以的），可以想想，因为结构体的方法里面都是传入的引用，总不能因为调用一个自己的方法，然后传入自己本身，导致所有权没了吧，所以这个时候结构体的方法里面会执行一层自动引用

需要注意的是，只有在结构体的方法调用中才会出现自动引用和自动解引用，也就是那个要构造list的方法

**普通函数是不会出现自动引用的**，有时会通过**解引用强制多态（Deref coercion）**来实现自动解引用

```rust
temp.push_str(" world!");
```

这个函数是没有返回值的

&str函数的replace方法返回的类型是String

> 在 Rust 中，`to_string()` 方法和 `to_owned()` 方法都可以将字符串切片（`&str`）转换为拥有所有权的字符串类型（`String`）

into（）方法用于&str上与to_string没有什么区别，只不过into方法本身是支持泛型的，但是to_string只支持字符串切片

只要注意到&str是不可变引用就好了

需要注意的是trim对于&str和String都是有的，对于String而言就是新建一个String，而对&str而言，就是对这个引用再次进行切片

rust的项目管理是不是很麻烦，实际上就是文件夹的一步步递归向下表示了不同文件之间模块的树状关系

模块不仅能用来当一个头文件，还可以用来管理代码

也就是说，如果模块不内联并且加上了**pub参数实际上就跟头文件差不多**；模块如果内联了又只是一个代码分类一样的东西

现在就只要认为crate就是一个文件就好了

> 在 Rust 中，默认所有项（函数、方法、结构体、枚举、模块和常量）对父模块都是私有的。

注意这里只是子模块对父模块私有，但是父模块对子模块不私有

> 但是 hosting 的 内容（contents）仍然是私有的；这表明使模块公有并不使其内容也是公有的。模块上的 pub 关键字只允许其父模块引用它，而不允许访问内部代码。因为模块是一个容器，只是将模块变为公有能做的其实并不太多；同时需要更深入地选择将一个或多个项变为公有

> use crate::front_of_house::hosting，现在 hosting 在作用域中就是有效的名称了

也就是说只是将最后一个将最后一个字段定义为全字段，以上面的为例，只是把hosting定义为crate::front_of_house::hosting

需要注意的是use相当于只是起了一个别名，并没有实现pub的功能，也就是说use之前还是要pub的

一般而言use只是指定函数的完整路径，也就是指定到最深的模块，如上面的use crate::front_of_house::hosting，只是指定到最后的一个模块hosting

```rust
mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
```

同级的模块应该是相互可见的

所以说如果想要使用一个mod中的某个方法，只要从下至上pub即可

> 使用 use 关键字，将某个名称导入当前作用域后，这个名称在此作用域中就可以使用了，但它对此作用域之外还是私有的。如果想让其他人调用我们的代码时，也能够正常使用这个名称，就好像它本来就在当前作用域一样，那我们可以将 pub 和 use 合起来使用。

相当于在当前作用域扩展了名称的作用域

因为rust的库里面都是pub的，所以在自己的文件中是可以通过use关键字来导包的

prelude是一个特殊的模块，它的内容会被自动use

hashmap是分配在堆上的，因为是可以一直插入，是可变的

use关键字的路径中也是可以使用通配符*的

将String传入hashmap也会使得String失去所有权

> 对于像 i32 这样的实现了 Copy trait 的类型，其值可以拷贝进哈希 map。对于像 String 这样拥有所有权的值，其值将被移动而哈希 map 会成为这些值的所有者

hashmap的键值对的类型是由插入的第一个元素决定的，这是因为hashmap接受的是泛型

> unwrap 是 Rust 中 Option 和 Result 类型的一个方法。当调用 unwrap 时，如果 Option 是 Some 或者 Result 是 Ok，它会返回内部的值。但是，如果 Option 是 None 或者 Result 是 Err，它会引发一个 panic，导致程序崩溃。

```rust
assert_eq!(*basket.get(&Fruit::Apple).unwrap(), 4);
        assert_eq!(*basket.get(&Fruit::Mango).unwrap(), 2);
        assert_eq!(*basket.get(&Fruit::Lychee).unwrap(), 5);
```

所以这里的unwrap就相当于类型检查

```rust
let mut scores: HashMap<String, Team> = HashMap::new();
```

可以这样指定hashmap的类型

rust也可以使用加号链接字符串

```rust
let mut scores = HashMap::new();
scores.insert("Blue", 10);

if scores.contains_key("Blue") {
    println!("Found the team Blue!");
} else {
    println!("Didn't find the team Blue.");
}
```

contains_key可以判断当前的hashmap中有没有这个key

hashmap的get方法只能接收一个引用类型

get返回的是一个不可变引用，如果要返回一个可变引用的话就需要使用get_mut

是不能把一个不可变引用赋值给一个可变引用的，也不能把一个可变引用赋值给不可变引用，这是因为只能有一个可变引用的规则。但是按照这个规则的话是能把一个不可变引用赋值给一个不可变引用

```rust
let x = 5;
let y = &x;
let z = y; // 把一个不可变引用赋值给另一个不可变引用
```

这里就可以把一个不可变引用赋值给一个不可变引用

需要注意的是正常的for循环遍历也是包前不包后的

```rust
Command::Append(usize)=>{
                for i in 0..usize {
                  string+=*("bar".to_string());
                }
                output.push(*string);
              }
```

注意到这里传进来的时候传进来的usize是一个引用

注意有的库函数会返回对象本身，是因为函数内部实现了clone；有的函数就只会返回对象的引用

但是大部分函数的传参都是引用，这是为了防止用户自己定义的变量经过库函数调用之后而失去使用权

```rust
let mut s = String::from("foo");
    s.push_str("bar");
```

使用 `push_str` 方法向 `String` 附加字符串 slice（也就是&str类型）

option里面装的是一个none和一个some，是为了应对结果可能为空的情况。如果结果为空，option枚举就会是一个none，如果是有值的话，就是一个some

> 消除了错误地假设一个非空值的风险，会让你对代码更加有信心。为了拥有一个可能为空的值，你必须要显式的将其放入对应类型的 `Option<T>` 中。接着，当使用这个值时，必须明确的处理值为空的情况。只要一个值不是 `Option<T>` 类型，你就 **可以** 安全的认定它的值不为空。这是 Rust 的一个经过深思熟虑的设计决策，来限制空值的泛滥以增加 Rust 代码的安全性。

上面这段话就是option的作用。显式地将值放在option内实际上就是已经保证了值的安全，因为在使用之前一定要判断是否为空

```rust
pub fn unwrap(self) -> T
```

unwrap方法可以取出some里面的值

```rust
 let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
```

if let的用法，可以发现if let是可以直接取出来some里面的值的

```rust
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
```

也就是这样使用

需要注意的是if let和while let中只能使用Some或者None，而不能使用Option，意思是如果Option是Some类型的话就执行相应的操作

> // TODO: make this a while let statement - remember that vector.pop also
>         // adds another layer of Option<T>. You can stack `Option<T>`s into
>         // while let and if let.

这里的意思是pop出来的实际上又套了一层Option

也就是vec的pop函数返回的是vec元素类型的一个Option，为了防止返回的元素是空值。

```rust
while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }
```

所以上面optional_integers.pop()返回的类型应该是Option套了一个Option，非空的返回应该就是一个Some（Option），也就是一个Some（Some（int））

需要注意的是，有一些类型是实现了Copy trait的，所以有的时候会自己clone；但是自己实现的类型就一般没有实现copy trait，就只能将所有权转移了。所以对自己的类型一般就只能传引用

> 需要注意的是，`ref` 关键字只能用于可变绑定。如果要创建不可变引用绑定，可以使用 `ref` 关键字的变体 `ref immut`

[关于ref关键字与&](https://blog.csdn.net/quicmous/article/details/120489008)

可以看出ref和&的主要区别就是在函数传参上。如果是像下例这样没办法声明类型的情况，就只能使用ref关键字（当然，将match y修改为match &y也是可行的）

```rust
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
```

这里在进行match的时候也会使y的所有权转移，所以要不然就要在p前面加上ref关键字表示当前只是引用

注意option的ok_or方法传入的参数是返回的Result中err的值

```rust
pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>();

    Ok(qty * cost_per_item + processing_fee)
}
```

这里的parse函数就是把字符串转换为一个i32类型，返回的值是一个i32，err的Result

```rust
pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>();
    match &qty{
        Ok(num)=>Ok(num * cost_per_item + processing_fee),
        Err(err)=>qty
    }
}
```

这样就可以返回Result<i32, ParseIntError>类型了，如果返回的是一个Ok也可以直接转换为Result<i32, ParseIntError>

所以ref一般就是用在match里面防止变量的所有权转移的

需要注意的是？表达式返回的一个err（对于Result类）或者none（对于Option类），这就要求函数的返回值就必须是Result或者Option

match使用花括号的时候是不用跟上,的

注意主函数返回的是（），也就是一个空值，但是又不是一个None，详见元组那一章

```rust
impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm...? Why is this only returning an Ok value?
        if value<0 {
            Err(CreationError::Negative)
        }else if value==0{
            Err(CreationError::Zero)
        }
        else{
            Ok(PositiveNonzeroInteger(value as u64))
        }
    }
}
```

这相当于是实现一个方法的规范，就是将所有的返回值都封装成一个Result，也就是一定要处理一下空值的问题。当然这是一种方法来处理空值。这一种方法实际上跟？的操作是一样的，也是判断当前是否是符合条件的，只不过这种方式的自己的定制性更强，而？就只能识别err或者是none

box的作用实际上就是：因为一个结构体或者说一个枚举实际上都是存在在栈上的（在c语言中也是这样的，每次想使用一个分配在堆上的结构体的时候就需要malloc一下），因为这些类型是用户自己定义的，如果用户要使用的话，编译器完全可以计算出一个结构体需要多大的空间，而对于枚举：

> 在 Rust 中，枚举类型的实例在默认情况下也是分配在栈上的。枚举类型的大小是其所有成员中最大的成员的大小。

所以如果出现下面这种情况

```rust
enum List {
    Cons(i32, List),
    Nil,
}
```

编译器还是会把当前的枚举当成是一个普通的枚举而尝试把他分配在栈上，但是当开始解析ons(i32, List),的时候就发现这个类型期待被分配一个无穷的栈空间，所以编译器会报错

所以说，分配在堆上的只有那些在运行时大小会变化的类型，如只有在运行时执行了vecpush，这个vec的大小才会变大

```rust
trait MyTrait {
    fn my_method(&self);
}

struct MyStruct;

impl MyTrait for MyStruct {
    fn my_method(&self) {
        println!("Calling my_method from MyStruct");
    }
}

fn main() {
    let my_trait_object: &dyn MyTrait = &MyStruct;
    my_trait_object.my_method();
}
```

> 在上述示例中，我们定义了一个名为 `MyTrait` 的 trait，其中包含一个方法 `my_method`。然后，我们为 `MyStruct` 实现了 `MyTrait`。在 `main` 函数中，我们创建了一个 `&dyn MyTrait` 的 trait 对象 `my_trait_object`，并将其指向 `MyStruct` 的实例。使用 `dyn` 关键字，我们标识了 `my_trait_object` 的动态类型为 `MyTrait`。
>
> 在运行时，当调用 `my_trait_object.my_method()` 时，编译器会进行动态分发，并根据实际类型（`MyStruct`）调用正确的方法。
>
> 需要注意的是，`dyn` 关键字只能用于 trait 对象类型，不能用于普通的具体类型。它是用于在运行时进行动态分发的类型标识。

说白了就是dyn应该就是实现了多态。就好像java一样，可以用一个接口的类型来接一个实体类，然后如果调用了这个接口的方法，那么在编译的时候编译器就将根据右边的类型**自动分发**，根据右边实际的类型调用正确的方法，进而实现了多态

```rust
fn main() -> Result<(), Box<dyn ???>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}
```

稍微理解了一下box和dyn之后，就不难发现，这里的box实际上就是希望将某个东西放在堆上，这样的话main函数结束之后就不会把这个东西当成一个栈上的变量而释放

```rust
impl error::Error for CreationError {}
```

注意到这里下面有一个为CreationError实现了一个trait，而ParseIntError也应该也实现了这个trait，就相当于这两个结构体都实现了这个接口，也就是这两个结构体都可以赋值给error::Error，所以就可以使用error::Error来当做这两个err

这一题的main函数如下：

```rust
fn main() -> Result<(), Box<dyn error::Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}
```

```rust
fn process_data(data: &str) -> Result<(), String> {
    let parsed_number = data.parse::<i32>().map_err(|err| err.to_string())?;
    let result = 100 / parsed_number;
    println!("Result: {}", result);
    Ok(())
}
```

上面这个map_err函数的参数也可以看做是一个函数，这里面的err就是原来的错误，而err.to_string就是返回值（也就是返回的另一个错误的类型）

> 当您使用 `map_err` 方法时，可以传递一个函数作为参数，该函数需要接受原始错误类型并返回一个新的错误类型。这样，您可以使用自定义的错误处理函数来转换或映射错误

即：

```rust
impl ParsePosNonzeroError {
    //这个函数就是将CreationError通过枚举ParsePosNonzeroError封装成了一个ParsePosNonzeroError
    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)
    }
    // TODO: add another error conversion function here.
    fn from_parseint(err:ParseIntError)->ParsePosNonzeroError{
        ParsePosNonzeroError::ParseInt(err)
    }
}

fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
    // TODO: change this to return an appropriate error instead of panicking
    // when `parse()` returns an error.
    let temp=s.parse().map_err(ParsePosNonzeroError::from_parseint);
    let x: i64 = temp.unwrap();
    PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation)
}
```

```rust
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
```

这个就是问号的作用，？表达式如果检测到不是none或者err的话就会直接把some或者ok解开。这个也是一个很符合常理的操作，如果？已经检测到了当前的Result是none或者err了，就会直接返回。如果没有直接返回的话就说明当前类型一定是一个some或者ok，一定是一个有效值。所以这个时候完全可以直接顺便把some或者ok解开。

```rust
fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
```

Vec是可以存储一个泛型的

trait实际上就跟java中的接口一样，只不过java中的接口是需要类实现的，而trait是给结构体实现的

> 同理，当在函数签名中使用一个类型参数时，必须在使用它之前就声明它。为了定义泛型版本的 largest 函数，类型参数声明位于函数名称与参数列表中间的尖括号 <> 中，像这样：

```rust
fn largest<T>(list: &[T]) -> &T {
```

泛型的函数需要在参数列表之前声明泛型变量

```rust
struct Point<T> {
    x: T,
    y: T,
}
```

上面是结构体的泛型

![image-20240419165225799](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240419165225799.png)

T定义了之后，所有的T变量就应该是一样的

需要注意的是如果加上了泛型的话，那么结构体的名字就应该是Point<T>（以上面的例子为例）

```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

还可以指定泛型的类型，所以只需要将<>理解为是一个泛型声明的参数

> error[E0412]: cannot find type `T` in this scope
>   --> exercises/generics/generics2.rs:15:14
>    |
> 15 | impl Wrapper<T>{
>    |              				^ not found in this scope
>    |

注意如果在imp后面没有去声明这个T的话，那么Rust就无法识别结构体Wrapper<T>中的T。或者换一种理解，这里的Wrapper<T>是一个结构体的名字，使用这个结构体你需要告诉编译器，T到底是一个什么类型，所以需要先在impl后面去再声明一次这个泛型

trait里面很像接口，只是声明了一个函数的签名（就是不带函数体的）。

```rust
trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    fn append_bar(mut self) -> Self{
        self+="bar";
        self
    }
    // TODO: Implement `AppendBar` for type `String`.
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}
```

这一段代码实际上就是一直在玩所有权转移，将不可变变量的所有权转移给一个可变变量，再将一个可变变量的所有权转移给不可变变量。但是需要注意的是，引用是不能这样的，这是由rust的所有权规则规定的，一个变量只能有一个可变引用或者若干个不可变引用

impl trait for后面需要跟上的是一个类型（结构体的名字在声明之后也是算做一个类型）

```rust
trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement trait `AppendBar` for a vector of strings.
//这个的意思是要在后面加上一个Bar成员，而不是在所有的向量成员后面加上一个Bar
impl AppendBar for Vec<String>{
    fn append_bar(mut self) -> Self{
        for string in self.iter_mut(){
            //需要注意的是这里返回的是里面元素的可变引用
            *string+="Bar";
        }
        self
    }
}
```

这个代码还是能看一下的，这里iter_mut方法只能作用于可变的变量

> 其他依赖 aggregator crate 的 crate 也可以将 Summary 引入作用域以便为其自己的类型实现该 trait。需要注意的限制是，只有在 trait 或类型至少有一个属于当前 crate 时，我们才能对类型实现该 trait。例如，可以为 aggregator crate 的自定义类型 Tweet 实现如标准库中的 Display trait，这是因为 Tweet 类型位于 aggregator crate 本地的作用域中。类似地，也可以在 aggregator crate 中为 Vec<T> 实现 Summary，这是因为 Summary trait 位于 aggregator crate 本地作用域中。
>
> 但是不能为外部类型实现外部 trait。例如，不能在 aggregator crate 中为 Vec<T> 实现 Display trait。这是因为 Display 和 Vec<T> 都定义于标准库中，它们并不位于 aggregator crate 本地作用域中。这个限制是被称为 相干性（coherence）的程序属性的一部分，或者更具体的说是 孤儿规则（orphan rule），其得名于不存在父类型。这条规则确保了其他人编写的代码不会破坏你代码，反之亦然。没有这条规则的话，两个 crate 可以分别对相同类型实现相同的 trait，而 Rust 将无从得知应该使用哪一个实现。

在impl trait for Type这样的trait实现中，trait和type中至少有一个属于当前的编译块

> 这条规则确保了其他人编写的代码不会破坏你代码

这是因为如果可以随便实现trait的话，那别人就可以在他的代码里面为你随便实现一个外部的trait

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

这个是默认方法，这个时候trait又相当于是一个java中的父类。所以trait应该是一个java中的父类加上接口的概念

trait既然是一个接口加父类的概念，那自然是可以用trait类型来接收所有实现了trait的类

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

如上，这里的引用不是必须的，但是impl关键字是必须的，必须告诉编译器后面的是一个trait，不然编译器会强制想要确定这个trait的大小（但是实际上trait的大小是取决于实现它的结构），最终导致编译器无法在编译阶段确定一个trait的大小

![image-20240419201500221](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240419201500221.png)

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

实际上就是把泛型和trait参数结合在一起了

```rust
pub fn notify(item: &(impl Summary + Display)) 
pub fn notify<T: Summary + Display>(item: &T) 
```

上面就是两个+语法的使用

![image-20240419203018609](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240419203018609.png)

有的时候需要对泛型进行限制。这里就是编译器检查的时候发现format里面使用了一个类型变量T但是这个变量并没有实现打印需要实现的trait，所以这里需要对泛型进行限定以保证format函数可以使用。也就是给泛型加上一个bound：

```rust
pub fn notify<T: Summary + Display>(item: &T)
```

> 当我们定义这个函数的时候，并不知道传递给函数的具体值，所以也不知道到底是 `if` 还是 `else` 会被执行。我们也不知道传入的引用的具体生命周期，所以也就不能像示例 10-17 和 10-18 那样通过观察作用域来确定返回的引用是否总是有效。借用检查器自身同样也无法确定，因为它不知道 `x` 和 `y` 的生命周期是如何与返回值的生命周期相关联的。为了修复这个错误，我们将增加泛型生命周期参数来定义引用间的关系以便借用检查器可以进行分析。

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

这是因为生命周期检查是看引用和数据本身声明周期的长短，因为上面这个例子中只是返回一个引用，这个时候在检查这个返回值（也就是检查这个返回值引用是否有效）的时候，就会因为不知道数据本身是谁而报错（因为需要比较引用和数据本身的声明周期）

生命周期注解只是表明了生命周期的关系，并没有改变引用的生命周期

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

> 我们希望函数签名表达如下限制：也就是这两个参数和返回的引用存活的一样久。（两个）参数和返回的引用的生命周期是相关的。就像示例 10-21 中在每个引用中都加上了 `'a` 那样
>
> 例如如果函数有一个生命周期 `'a` 的 `i32` 的引用的参数 `first`。还有另一个同样是生命周期 `'a` 的 `i32` 的引用的参数 `second`。这两个生命周期注解意味着引用 `first` 和 `second` 必须与这泛型生命周期存在得一样久

所以可以看到生命周期注解实际上要一起用才能表示引用之间的生命周期关系，这里就相当于是告诉借用检查器，你在检查生命周期的时候不用管返回的结果的生命周期是x还是y，因为我已经告诉你，他们的声明周期是一样的了

需要注意的是这个规则只是回去检查引用类型的声明周期

rust中的取反也是！

[关于rust的自动测试](https://kaisery.github.io/trpl-zh-cn/ch11-01-writing-tests.html)

这里甚至能自己测试性能

```rust
#[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
```

这里传给了断言函数更多的参数

> 所以可以传递一个包含 `{}` 占位符的格式字符串和需要放入占位符的值

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

如果测试是为了测试代码的panic的话就要加上#[should_panic]属性（这个叫做代码的属性）

> 另外需要注意到从 `next` 调用中得到的值是 vector 的不可变引用。`iter` 方法生成一个不可变引用的迭代器。如果我们需要一个获取 `v1` 所有权并返回拥有所有权的迭代器，则可以调用 `into_iter` 而不是 `iter`。类似的，如果我们希望迭代可变引用，则可以调用 `iter_mut` 而不是 `iter`

这里说的next调用得到的值的是vector的不可变引用，实际上就表明了对向量元素的引用本质上就是对向量的引用，这里返回的就是向量元素的引用

即：

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // 此处省略了方法的默认实现
}
```

这里的item就已经被处理成向量（或者说向量元素）的不可变引用了

```rust
c.as_str()
```

一个迭代器是可以直接将剩下的元素拼在一起的，上面这个方法就是拼成一个str

```rust
to_uppercase()
```

这个方法就是返回一个大写，但是返回的类型是可以使用to_string的

```rust
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|element|{
        capitalize_first(element)
    }).collect()
}
```

上面是一个比较骚的方法，可以看出collect方法返回的是一个向量

collect方法会自己推断应该生成一个什么类型，当然也可以直接指定泛型参数来指定他要返回的类型

```rust
pub fn capitalize_words_string(words: &[&str]) -> String {
    words.iter().map(|element|{
        capitalize_first(element)
    }).collect::<String>()
}
```

上面是指定泛型参数的，指定collect方法转换为一个String

```rust
pub fn capitalize_words_string(words: &[&str]) -> String {
    words.iter().map(|element|{
        capitalize_first(element)
    }).collect()
}
```

上面的这个例子是不指定泛型参数的，collect直接根据上下文推断需要生成的类型

```rust
fn result_with_list() -> Result<Vec<i32>,DivisionError> {
    let numbers = vec![27, 297, 38502, 81];
    numbers.into_iter().map(|n| divide(n, 27)).collect()
}
```

需要注意的是，vec和数组元素都是用[]包起来的

```rust
struct Num{
    number:u64
}

impl Iterator for Num{
    type Item=u64;
    fn next(&mut self) -> Option<Self::Item>{
        if self.number>1{
            self.number-=1;
            Some(self.number)
        }else if self.number ==1{
            self.number-=1;
            Some(1)
        }else{
            None
        }
    }
}

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    let temp=Num{
        number:num+1
    };
    let result=temp.fold(1, |acc,x| acc*x);
    print!("the num is {}",result);
    result
}
```

这段代码挺有意义的，为一个结构体实现了迭代器，需要注意的是不能直接给u64实现一个迭代器，这是因为孤儿规则（trait和类型都不在当前的作用域中）

需要注意的是vec或者hashmap这种数据结构调用方法返回的都是引用

这个fold方法是真TM牛逼

![image-20240420102644022](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240420102644022.png)

![image-20240420102653902](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240420102653902.png)

这个能很直观的表明智能指针的作用

Box的new方法是创建一个指向new中对象的指针

Rc<T>就是为了启用多所有权的（相当于是不知道数据的所有者应该是谁），也就是下面这一段话

Rc的一个经典例子就是图，当一个节点有多条边的时候，只有当所有的边都被释放的时候才会释放节点，但是在编译的阶段是没办法确定哪一条边是最后释放的，所以这个时候就要所有的边共用所有权

> `Rc<T>` 用于当我们希望在堆上分配一些内存供程序的多个部分读取，而且无法在编译时确定程序的哪一部分会最后结束使用它的时候。如果确实知道哪部分是最后一个结束使用的话，就可以令其成为数据的所有者，正常的所有权规则就可以在编译时生效

> 注意 `Rc<T>` 只能用于单线程场景

```rust
fn main() {
    let a = Cons(5,
        Box::new(Cons(10,
            Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
}
```

这个例子表明在使用Box的new方法的时候会发生所有权的转移

```rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```

使用Rc的例子（这个智能指针就是来看引用的数量进而判断什么时候需要释放堆上的空间）与Box相同，这个智能指针也是在堆上分配了一个空间，然后在栈上使用指针指向堆上的数据

```rust
drop(mars);
```

drop函数用于销毁一个引用，这个时候就已经有c的味道了，这个drop就相当于是一个对引用的free函数（需要注意的是c的free函数是对堆上的数据进行的，而这个drop就是对引用而言的）

> 在其他一些语言中，我们不得不记住在每次使用完智能指针实例后调用清理内存或资源的代码。如果忘记的话，运行代码的系统可能会因为负荷过重而崩溃。在 Rust 中，可以指定每当值离开作用域时被执行的代码，编译器会自动插入这些代码。于是我们就不需要在程序中到处编写在实例结束时清理这些变量的代码 —— 而且还不会泄漏资源
>
> 指定在值离开作用域时应该执行的代码的方式是实现 `Drop` trait。`Drop` trait 要求实现一个叫做 `drop` 的方法，它获取一个 `self` 的可变引用。为了能够看出 Rust 何时调用 `drop`，让我们暂时使用 `println!` 语句实现 `drop`

这里说drop方法在退出作用域的时候会被编译器自动添加，所以也不会造成内存的泄露

arc实际上就是线程安全版本的rc

```rust
        let child_numbers = Arc::clone(&shared_numbers);
```

需要注意的是，使用rc或者是arc的clone方法传入的应该是原数据的引用

所以cow指针实际上就是检查当前的cow值是一个引用（也就是还有其他引用或者说并没有所有权）还是一个拥有所有权的独占Owned

```rust
let numbers: Vec<_> = (0..100u32).collect();
```

这是什么鸡毛

> 第一次使用 `String` 值调用 `example_closure` 时，编译器推断 `x` 和此闭包返回值的类型为 `String`。接着这些类型被锁定进闭包 `example_closure` 中，如果尝试对同一闭包使用不同类型则会得到类型错误。

```rust
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5);
```

```rust
fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}
```

> 这里，即便 `x` 并不是 `equal_to_x` 的一个参数，`equal_to_x` 闭包也被允许使用变量 `x`，因为它与 `equal_to_x` 定义于相同的作用域。

> `JoinHandle` 提供了一些方法来等待线程完成和获取其返回值。其中最常用的方法是 `join()`，用于等待线程完成并获取其返回值，返回一个 `Result` 类型。其他方法还包括 `thread()`、`id()`、`name()` 等，用于获取与线程相关的信息。

所以可以使用handle.join().unwrap()，这是因为join之后就变成一个Result了

> Rust 会 **推断** 如何捕获 `v`，因为 `println!` 只需要 `v` 的引用，闭包尝试借用 `v`。然而这有一个问题：Rust 不知道这个新建线程会执行多久，所以无法知晓 `v` 的引用是否一直有效。

在线程的闭包中使用环境中的变量的时候就会出现这种情况，编译器认为只需要引用，但是并不知道数据本身什么时候被释放，所以这个时候需要将数据的所有权转移给线程闭包

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    drop(v); // oh no!

    handle.join().unwrap();
}
```

如这种情况

锁的所有权是不能随意在不同的线程之间转移的，需要注意的是锁执行lock操作的时候会获取锁的所有权

```rust
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
```

> 一旦获取了锁，就可以将返回值（在这里是`num`）视为一个其内部数据的可变引用了

> 正如你所怀疑的，`Mutex<T>` 是一个智能指针。更准确的说，`lock` 调用 **返回** 一个叫做 `MutexGuard` 的智能指针。这个智能指针实现了 `Deref` 来指向其内部数据；其也提供了一个 `Drop` 实现当 `MutexGuard` 离开作用域时自动释放锁，这正发生于示例 16-12 内部作用域的结尾。为此，我们不会冒忘记释放锁并阻塞互斥器为其它线程所用的风险，因为锁的释放是自动发生的。

这里返回的应该是一个Result枚举，ok分支是`MutexGuard` 智能指针（可以自动解引用，因为实现了Deref，所以可以不用手动解引用而直接将这个智能指针当做一个变量来使用），所以可以使用unwrap方法

并且这个锁是实现了drop方法可以自动释放，这个就很巧妙，因为由于类型不同一定要申请锁，而释放锁是让编译器自己做的

> 一旦获取了锁，就可以将返回值（在这里是`num`）视为一个其内部数据的可变引用了

说明lock函数返回值中的`MutexGuard` 的智能指针中包含的是一个被加锁内容的可变引用

> 这里使用 `mpsc::channel` 函数创建一个新的通道；`mpsc` 是 **多个生产者，单个消费者**（*multiple producer, single consumer*）的缩写。简而言之，Rust 标准库实现通道的方式意味着一个通道可以有多个产生值的 **发送**（*sending*）端，但只能有一个消费这些值的 **接收**（*receiving*）端。想象一下多条小河小溪最终汇聚成大河：所有通过这些小河发出的东西最后都会来到下游的大河。目前我们以单个生产者开始，但是当示例可以工作后会增加多个生产者。

消息的send函数返回的也是一个Result。这里就做一个统一吧，就是所有函数的返回值都使用Result或者是Option（可以再去看看这两个枚举的不同适用场景）

> 通道的发送端有一个 `send` 方法用来获取需要放入通道的值。`send` 方法返回一个 `Result<T, E>` 类型，所以如果接收端已经被丢弃了，将没有发送值的目标，所以发送操作会返回错误。在这个例子中，出错的时候调用 `unwrap` 产生 panic。不过对于一个真实程序，需要合理地处理它：回到第 9 章复习正确处理错误的策略。

下面是一个线程信息通信的常用函数：

```rust
tx.send();//返回值是一个Result，表示的是发送是否成功
rx.recv();//这个方法会阻塞当前线程直到收到数据，返回值是一个Result，表示接受是否成功，成功的话数据放在ok分支中，err表示的是发送通道关闭
rx.try_recv();//这个方法不会阻塞当前线程，而是立即返回Result，其中ok表示当前已经接收到的信息，而err表示的是没有接收到信息
```

> `send` 函数获取其参数的所有权并移动这个值归接收者所有。这可以防止在发送后再次意外地使用这个值；所有权系统检查一切是否合乎规则

也就是使用消息进行信息传递的时候就顺便将所有权转移过去了

消息传递可以用在同步里面，而锁可以用在互斥里面

> 在主线程中，不再显式调用 `recv` 函数：而是将 `rx` 当作一个迭代器。对于每一个接收到的值，我们将其打印出来。当通道被关闭时，迭代器也将结束。

rx也可以当做是一个迭代器！（rx是通道声明的时候指定的接收端）

```rust
let tx1 = tx.clone();
```

这样可以创建多个生产者

> 例如，可以选择为 `Point<f32>` 实例实现方法，而不是为泛型 `Point` 实例。示例 10-10 展示了一个没有在 `impl` 之后（的尖括号）声明泛型的例子，这里使用了一个具体类型，`f32`：

```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

这里就是指定了只为Point<f32>实现了一个方法，而不是为Point<T>实现的

> 一个函数标签必须声明函数参数个数和类型。相比之下，宏能够接受不同数量的参数：用一个参数调用 `println!("hello")` 或用两个参数调用 `println!("hello {}", name)` 。而且，宏可以在编译器翻译代码前展开，例如，宏可以在一个给定类型上实现 trait 。而函数则不行，因为函数是在运行时被调用，同时 trait 需要在编译时实现

这里说的trait在编译时要实现是因为可能我当前想为一个结构体实现一个trait，可以使用宏的方式，因为在编译的时候就会展开。但是不能在函数中进行，因为函数是在运行的时候才会执行，在运行的时候实现一个trait就会使得在编译阶段编译器不知道这个trait的是为谁实现的，或者说大小是多少

正如trait那一章所说的：

> 接着每一个实现这个 trait 的类型都需要提供其自定义行为的方法体，编译器也会确保任何实现 `Summary` trait 的类型都拥有与这个签名的定义完全一致的 `summarize` 方法。

编译器在编译阶段就需要去检查trait中的方法是否被实现trait的结构体实现（并且需要保证所有的结构体都要有一个trait中的方法能调用，也就是trait需要在编译的时候确定）。但是如果在函数中去实现一个trait的话，这个时候编译器就没办法检查了，因为函数在运行的时候才会去实现这个trait，实现了之后如果再去调用这个trait中的方法的话，编译器就应该会找不到这个在函数中实现的方法

> `#[macro_export]` 标注说明，只要将定义了宏的 crate 引入作用域，宏就应当是可用的。如果没有该标注，这个宏就不能被引入作用域。

> 单边模式 `( $( $x:expr ),* )`

这个单边模式是什么（需要去看rust圣经的18章）

宏跟函数的区别：

> 从根本上来说，宏是一种为写其他代码而写代码的方式，即所谓的 **元编程**（*metaprogramming*）。在附录 C 中会探讨 `derive` 属性，其生成各种 trait 的实现。我们也在本书中使用过 `println!` 宏和 `vec!` 宏。所有的这些宏以 **展开** 的方式来生成比你所手写出的更多的代码。
>
> 元编程对于减少大量编写和维护的代码是非常有用的，它也扮演了函数扮演的角色。但宏有一些函数所没有的附加能力。
>
> 一个函数标签必须声明函数参数个数和类型。相比之下，宏能够接受不同数量的参数：用一个参数调用 `println!("hello")` 或用两个参数调用 `println!("hello {}", name)` 。而且，宏可以在编译器翻译代码前展开，例如，宏可以在一个给定类型上实现 trait 。而函数则不行，因为函数是在运行时被调用，同时 trait 需要在编译时实现。
>
> 实现一个宏而不是一个函数的缺点是宏定义要比函数定义更复杂，因为你正在编写生成 Rust 代码的 Rust 代码。由于这样的间接性，宏定义通常要比函数定义更难阅读、理解以及维护。
>
> 宏和函数的最后一个重要的区别是：在一个文件里调用宏 **之前** 必须定义它，或将其引入作用域，而函数则可以在任何地方定义和调用。

一定要先声明宏（或者定义宏）然后才能使用！，不然编译器是没办法进行替换的

> `#[macro_export]` 标注说明，只要将定义了宏的 crate 引入作用域，宏就应当是可用的。如果没有该标注，这个宏就不能被引入作用域。

这个标注可以理解为是一个宏的pub，在宏上是没办法直接使用pub关键字的

```rust
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
```

这波是if大杂烩，所以不用把if let当成一个固定搭配，这只不是告诉我，在if的时候是可以进行赋值的，而此时if的条件就是赋值是否成功

> 在 Rust 中，`enumerate()` 是一个迭代器适配器（Iterator Adapter），它提供了对迭代器元素进行编号的功能。该方法返回一个新的迭代器，其中每个元素都是原始迭代器元素及其对应的索引

```rust
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}
```

> 第二个匹配分支中的模式引入了一个新变量 `y`，它会匹配任何 `Some` 中的值。因为我们在 `match` 表达式的新作用域中，这是一个新变量，而不是开头声明为值 10 的那个 `y`。这个新的 `y` 绑定会匹配任何 `Some` 中的值，在这里是 `x` 中的值。因此这个 `y` 绑定了 `x` 中 `Some` 内部的值。这个值是 5，所以这个分支的表达式将会执行并打印出 `Matched, y = 5`。

这个有点骚，match里面的y就相当于是一个新的变量（声明了但是还没有被赋值，但是在进行match的时候就会进行模式匹配而匹配成5），用于匹配Some中的值，而不会把他认为是外部作用域中的y

> 在 `match` 表达式中，可以使用 `|` 语法匹配多个模式，它代表 **或**（*or*）的意思。

```rust
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

> `..=` 语法允许你匹配一个闭区间范围内的值。

```rust
let x = 5;

match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
}
```

> 只需列出结构体字段的名称，则模式创建的变量会有相同的名称

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}
```

```rust
match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h,
                s,
                v
            )
        }
        _ => ()
    }
```

这里解构了一个嵌套的枚举（应该使用嵌套的match应该也是能处理的）

> 如果你创建了一个变量却不在任何地方使用它, Rust 通常会给你一个警告，因为这可能会是个 bug。但是有时创建一个还未使用的变量是有用的，比如你正在设计原型或刚刚开始一个项目。这时你希望告诉 Rust 不要警告未使用的变量，为此可以用下划线作为变量名的开头。示例 18-20 中创建了两个未使用变量，不过当运行代码时只会得到其中一个的警告

```rust
fn main() {
    let _x = 5;
    let y = 10;
}
```

这里只会得到y未使用的警告（没有使用就报警告，有点像java，再严重一点直接报错就变成go了）

> 对于有多个部分的值，可以使用 `..` 语法来只使用部分并忽略其它值，同时避免不得不每一个忽略值列出下划线。

```rust
match origin {
    Point { x, .. } => println!("x is {}", x),
}
```

> 匹配守卫 `if n == y` **并不是一个模式**所以没有引入新变量。这个 `y` **正是** 外部的 `y` 而不是新的覆盖变量 `y`，这样就可以通过比较 `n` 和 `y` 来表达寻找一个与外部 `y` 相同的值的概念了。

```rust
match msg {
    Message::Hello { id: id_variable @ 3..=7 } => {
        println!("Found an id in range: {}", id_variable)
    },
    Message::Hello { id: 10..=12 } => {
        println!("Found an id in another range")
    },
    Message::Hello { id } => {
        println!("Found some other id: {}", id)
    },
}
```

这个@就好像是一个简写的模式守卫

在宏的匹配中，每一个匹配末尾都需要加上一个分号

```rust
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

这个是一个while let的使用场景

下面是如何引入一个clippy

> Clippy 是 Rust 社区中一个流行的 lint 工具，旨在帮助开发者发现和纠正潜在的代码问题。它是一个基于 lint 的插件，为 Rust 编译器提供了额外的代码检查功能。
>
> Clippy 提供了一系列 lint 规则，这些规则在编译过程中分析代码，并给出关于潜在问题的建议或警告。它可以帮助开发者编写更健壮、更符合 Rust 最佳实践的代码。
>
> Clippy 的规则涵盖了多个方面，包括但不限于：
>
> - 潜在的错误使用，如空指针解引用、除零操作等。
> - 不规范的代码风格，如不必要的类型转换、冗余的代码、命名约定等。
> - 潜在的性能问题，如不必要的复制、不必要的循环等。
> - 可能导致 bug 的逻辑问题，如无效的模式匹配、误用的 API 等。
>
> 通过在代码中使用 Clippy，开发者可以在编译过程中获得更多的静态检查和警告，帮助发现潜在的问题，并提供改进代码质量的建议。
>
> 要在 Rust 项目中使用 Clippy，您需要在 `Cargo.toml` 文件中添加以下依赖：
>
> ```toml
> [dev-dependencies]
> clippy = "版本号"
> ```
>
> 然后，您可以使用 `cargo clippy` 命令来运行 Clippy。它将分析您的代码，并输出与潜在问题相关的警告和建议。
>
> 总结：Clippy 是一个 Rust 社区中流行的 lint 工具，提供了一系列 lint 规则，帮助开发者发现和纠正潜在的代码问题。通过在编译过程中提供静态检查和警告，Clippy 可以帮助改进代码质量、减少错误和提高代码风格的一致性。

之后写的时候可以去用一下这个插件，这个能帮我很快的定位bug的位置并且提供修改的建议

> - `as` - 强制类型转换，消除特定包含项的 trait 的歧义，或者对 `use` 和 `extern crate` 语句中的项重命名

as的作用，上面说了两个

> 因为需要安全和方便移植，**rust**是可以跨平台编程的。 另外，在 Rust 中，数组索引通常使用 `usize` 类型。 `usize` 是一个无符号整数类型，它的大小可以根据系统架构自动调整。 在 64 位系统上， `usize` 的大小为 8 字节，在 32 位系统上， `usize` 的大小为 4 字节。

所以usize是一个跟机器有关的数据类型

> `From` trait 是 Rust 标准库中的一个 trait，用于定义从一个类型到另一个类型的转换。它提供了一种通用的转换机制，允许开发人员定义自定义类型之间的转换规则。

> 不需要再手动实现 `Into` trait。在 Rust 中，如果实现了 `From<A> for B`，那么编译器会自动为您实现 `Into<B> for A`。

> 在 Rust 中，`&str` 类型具有名为 `split()` 的方法，用于将字符串拆分为多个子字符串。该方法接受一个分隔符参数，并返回一个迭代器，该迭代器会生成拆分后的子字符串

split方法返回的是一个迭代器

```rust
fn main() {
    let my_struct: MyStruct = MyStruct::default();
    println!("Default value of MyStruct: {:?}", my_struct);

    let default_i32: i32 = i32::default();
    println!("Default value of i32: {}", default_i32);

    let default_bool: bool = bool::default();
    println!("Default value of bool: {}", default_bool);
}
```

default方法是这样调用的，而不是通过一个结构体进行点（其实想想也是，因为默认方法就是为了直接返回一个结构体实例，我怎么会再创建一个结构体去调用这个default方法嘞）

空字符串不算是None

省略return的情况只能是在同一个if else语句中，如：

```rust
                    if temp>1{
                        Person::default()
                    }else if let Ok(age)=num.parse::<usize>(){
                        Person{name:string.to_string(),age:age}
                    }
                    else{
                        Person::default()
                    }
```

上面这样是可以返回的，但是：

```rust
                    if temp>1{
                        Person::default()
                    }
                    if let Ok(age)=num.parse::<usize>(){
                        Person{name:string.to_string(),age:age}
                    }
                    else{
                        Person::default()
                    }
```

这样就不行了，编译器会提示要加上return，这就是因为不是在同一个if else语句中造成的

可以去看看19章的关联数据类型

&str方法返回的是：

> 需要注意的是，返回的 `words` 向量中的每个元素都是原始字符串 `sentence` 的切片，而不是克隆的新字符串。

也就是返回一堆&str

> 注意 `v1_iter` 需要是可变的：在迭代器上调用 `next` 方法改变了迭代器中用来记录序列位置的状态。换句话说，代码 **消费**（consume）了，或使用了迭代器。每一个 `next` 调用都会从迭代器中消费一个项。使用 `for` 循环时无需使 `v1_iter` 可变因为 `for` 循环会获取 `v1_iter` 的所有权并在后台使 `v1_iter` 可变。

也就是迭代器如果需要使用，就要是可变的，除非是在for循环中

> Basically, this is the same as From. The main difference is that this should return a Result type instead of the target type itself.

From trait的from方法返回的是一个目标类型的属性，而TryFrom和FromStr返回的是一个Result，注意到FromStr这个trait只能输入一个字符串然后输出别的类型，而其他两个trait可以输入任意的类型，只不过可能需要自己实现一下from方法还有try_from方法

并且主要函数的名字也不太一样

From ：from（）->target type（也就是他一定要返回一个目标类型的值，这也是为什么题里面选择返回默认值）

TryFrom：try_from（）->Result

FromStr：from_str（）->Result

```rust
pub trait From<T> {
    fn from(T) -> Self;
}
```

注意From是一个泛型的trait，而TryFrom也是同样的，这样就使得这两个trait可以接受不同的输入类型然后转换成需要的类型，而FromStr就相当于指定了输入的类型就一定是一个str

> Note that the implementation for tuple and array will be checked at compile time

这个的意思是，在编译的时候就会检查传参的时候数组或者元组的数量是不是对的，不会出现一个(arg1,arg2)赋值给一个(arg1,arg2,arg3)的情况

```rust
struct MyType {
    value: u32,
}

impl From<u32> for MyType {
    fn from(value: u32) -> Self {
        MyType { value }
    }
}

fn main() {
    let num: u32 = 42;

    let my_type: MyType = MyType::from(num);
    let my_type_again: MyType = num.into(); // 自动调用 Into<MyType> 实现

    println!("MyType value: {}", my_type.value);
    println!("MyType again value: {}", my_type_again.value);
}
```

实现了from方法之后就会自动创建into方法，但是需要注意的是这两个方法结果都是一样的，都是将类型A转换为类型B

```rust
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        let (red,green,blue)=tuple;
        if (red<0||red>255)||(green<0||green>255)||(green<0||green>255){
            Err(IntConversion)
        }
        Ok(Color{red:red as u8,green:green as u8,blue:blue as u8})
    }
}
```

一定要完全一致才能实现结构体的简写，就算是像上面这样使用了as进行类型转换也不行

> 1. `AsRef` trait：
>    - `AsRef` trait 允许将一个类型转换为另一个类型的引用。
>    - 它定义了一个方法 `as_ref(&self) -> &T`，该方法返回一个目标类型 `T` 的引用。
>    - 通常用于将一种类型的引用视为另一种类型的引用，而无需进行实际的拷贝或所有权转移。
>    - 例如，`String` 可以被视为一个 `&str`，所以可以使用 `as_ref()` 将 `String` 转换为 `&str` 的引用。
> 2. `AsMut` trait：
>    - `AsMut` trait 允许将一个类型的可变引用转换为另一个类型的可变引用。
>    - 它定义了一个方法 `as_mut(&mut self) -> &mut T`，该方法返回一个目标类型 `T` 的可变引用。
>    - 通常用于将一种类型的可变引用视为另一种类型的可变引用，而无需进行实际的拷贝或所有权转移。
>    - 例如，一个 `Vec<u8>` 可以被视为一个 `&mut [u8]`，所以可以使用 `as_mut()` 将 `Vec<u8>` 转换为 `&mut [u8]` 的可变引用。

注意一个是不可变引用，另一个是可变引用

```rust
fn byte_counter<T:AsRef<str>>(arg: T) -> usize {
    //这里的asref是将一个&str转换为&str
    arg.as_ref().as_bytes().len()
}

// Obtain the number of characters (not bytes) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
fn char_counter<T:AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Squares a number using as_mut().
// TODO: Add the appropriate trait bound.
fn num_sq<T:AsMut<u32>>(arg: &mut T) {
    // TODO: Implement the function body.
    //arg是一个智能指针类型，这里需要将他转换为i32的可变引用直接去操作堆上的数字
    let mut temp=arg.as_mut();
    *temp*=*temp;
}
```

这个真是有点神经

```rust
as_ref(&self) -> &T
```

上面是asref的实现（相信asmut也是一样的），这里如果传入的是一个&str的话，按照自动引用和自动解引用规则，首先需要进行一次自动引用，就产生类型&str、&&str、&&mut str，这个时候就直接匹配&str了，那么就表示当前的self类型应该是str，所以报错里面提示的都是没有为str实现AsRef<String>而不是没有为&str实现AsRef<String>

![image-20240420201511072](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240420201511072.png)

指针无法实现运算，所以在这里需要将Box转换为u32

但是在直接使用Box的时候又会自动解引用去获取堆上的实际值

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

> 当你使用 `as_ref` 方法将类型转换为引用时，编译器不会进行自动解引用

实际上就是在函数传参的时候才会进行自动强制解引用，在函数内部就不会再自动解引用了。而对于Box指针有的时候能自动解引用，有的时候又不能。Box自动解引用的例子如下：

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

> 裸指针与引用和智能指针的区别在于：
>
> - 允许忽略借用规则，可以同时拥有不可变和可变的指针，或多个指向相同位置的可变指针
> - 不保证指向有效的内存
> - 允许为空
> - 不能实现任何自动清理功能

> 可以在安全代码中 **创建** 裸指针，只是不能在不安全块之外 **解引用** 裸指针，稍后便会看到。

> 必须在一个单独的 `unsafe` 块中调用 `dangerous` 函数。如果尝试不使用 `unsafe` 块调用 `dangerous`，则会得到一个错误

```rust
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    assert!(mid <= len);

    (&mut slice[..mid],
     &mut slice[mid..])
}
```

> Rust 的借用检查器不能理解我们要借用这个 slice 的两个不同部分：它只知道我们借用了同一个 slice 两次。本质上借用 slice 的不同部分是可以的，因为结果两个 slice 不会重叠，不过 Rust 还没有智能到能够理解这些。当我们知道某些事是可以的而 Rust 不知道的时候，就是触及不安全代码的时候了

也就是说对rust而言，他就是把整个vec看成一个，如果有两个可变引用指向这个vec的成员，就算指向的是不同的也是不允许的

```rust
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.add(mid), len - mid))
    }
}

```

不安全代码的安全抽象实质上就是给不安全的代码套了层壳

> `extern` 块中声明的函数在 Rust 代码中总是不安全的。因为其他语言不会强制执行 Rust 的规则且 Rust 无法检查它们，所以确保其安全是开发者的责任：

不安全rust那一章中有介绍如何在rust中引入c，也介绍了如何在c中引入rust

```rust
Box::into_raw
Box::from_raw
```

这两个函数实现了Box和裸指针的转换

```rust
"hello".to_owned()
```

to_owned方法会根据&str的变量克隆一个String类型的变量

tests7中介绍了build.rs的作用，就相当于是C语言中的cmake一样，是用来构建程序（或者说指定编译？）的。

> `build.rs` 文件是一个 Rust 源文件，位于项目的根目录下，并且命名为 `build.rs`。当你使用 Cargo 构建项目时，Cargo 会自动检测到 `build.rs` 文件并在构建过程中执行它。

```rust
env::set_var("TEST_FOO", timestamp.to_string());
```

为什么不能再build脚本中使用这个函数

> 构建脚本可以通过发出 `rustc-cfg` 指令来开启编译时的条件检查。在本例中，一起来看看 [openssl](https://crates.io/crates/openssl) 包是如何支持多版本的 OpenSSL 库的。

上面这个是启用条件编译的指令

```rust
    let your_command = "rustc-cfg=feature=\"pass\"";
    println!("cargo:{}", your_command);
```

这个指令在命令行上要出现“”，所以在这里需要在pass外面再套一层“”，然后转移

```rust
#[no_mangle]
```

这个属性告诉rust的编译器不要修改这个函数的名称，这样才能在extern块中直接指定这个函数签名（如果在名字相同并且extern块就是rust的时候甚至不需要去加linkname，但是函数名不一样的话就需要加上linkname）。

也就是下面这个例子，其中extern块中的两个函数都是指定为my_demo_function

```rust
extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    #[link_name="my_demo_function"]
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    #[no_mangle]
    // No `extern` equals `extern "Rust"`.
    fn my_demo_function(a: u32) -> u32 {
        a
    }
}
```

也就是说no_mangle属性是在想要把rust代码给别人用的情况时添加的，而link_name属性就是放在rust代码想要引入其他语言的代码时候使用的，一个是提供代码（no_mangle），另一个是调用其他代码（link_name）

> as_ptr 是 NonNull<T> 类型的一个方法，它返回一个裸指针，指向 NonNull<T> 所指向的内存。

在rust中解引用运算符的优先级好像不是很高，应该是比点运算低的

> 在 Rust 中，裸指针本身并不具有裸指针所指向内存的所有权。裸指针只是一个指向内存地址的原始指针表示，它没有所有权的概念

裸指针就相当于是一个引用，就像引用变量没有所有权一样

```rust
fn sort<T>(array: &mut [T]){
	//TODO
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
```

为什么这里可以把一个向量传递给一个[]

> 在 Rust 中，`Vec<T>` 类型可以通过 `&mut` 操作符转换为可变切片 `&mut [T]`。这是因为 `Vec<T>` 内部存储的元素是连续排列的，所以可以通过引用 `&mut` 来访问和修改这些元素。

![image-20240422122403494](C:\Users\Lenovo\AppData\Roaming\Typora\typora-user-images\image-20240422122403494.png)

这种情况最简单的方法就是加上一个Copy trait的bound。如果不加要怎么处理？

> 要通过可变引用交换堆上的数据，你可以使用 `std::mem::swap` 函数来交换两个可变引用所指向的值。该函数定义在标准库的 `std::mem` 模块中，用于交换两个值。
>
> 以下是使用 `swap` 函数交换两个可变引用所指向值的示例：

```rust
use std::mem;

fn main() {
    let mut x = Box::new(10);
    let mut y = Box::new(20);

    println!("Before swap: x = {}, y = {}", x, y);

    mem::swap(&mut x, &mut y);

    println!("After swap: x = {}, y = {}", x, y);
}
```

```rust
array.swap(j+1, j);
```

对于一个数组，或者说切片（从形式上看就是一个数组的可变引用），需要使用上面的方法，这样就可以通过一个可变引用交换堆上的数据了（其中array是一个可变切片引用）

Q

看看什么时候会进行自动引用，什么时候会进行自动解引用，什么时候会进行解引用强制转换

我只有一个指向某一个空间的裸指针，如何改变所有权？--可以先转换成Box然后在转换所有权









