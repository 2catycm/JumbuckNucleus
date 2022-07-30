# 基于 RISC-V 架构与Rust语言的16KiB页面操作系统

# ——操作系统 Project 报告

[TOC]

## 项目简介

### 项目协作

| 姓名   | 学号     | 负责内容                                                     |
| ------ | -------- | ------------------------------------------------------------ |
| 叶璨铭 | 12011404 | 阅读RISCV手册，进行QEMU与地址空间设计。用Rust实现并测试物理页分配、进程切换算法。实验测定16KiB方案的优劣。 |
| 邓值仁 | 12012029 | 掌握Rust语言。深入理解并文档化rCore，深度优化 rCore 设计，使得每个部分都比较稳定可靠、简单清晰、符合rust规范。 |

---

### 为什么页面大小选择为16KiB，而不是传统甚至是默认的4KiB？

#### 从缓存（按需分页）的角度来看

​	从按需分页(Demand paging)的角度来看，内存是磁盘的缓存。所谓分页，就是缓存中的分 block ， 也就是一个传输单位。同一个block或者page的地址(访问需求), 由于访问的内容处于硬件上的同一个传输单位，因此只需第一次访问遍将内容缓存，加速大量的后续访问。从上面的原理描述可以看出，block越大，理论上block越大，Miss Rate便越低。极端情况下

<img src="Project Report.assets\image-20220729190213362.png" alt="image-20220729190213362" style="zoom:33%;" />

当然，Block太大也会出现以下的问题:

- Miss Penalty 增大。即 Page Replacement 完成所需时间增加。
- 

#### 从虚拟化的角度来看

上面我们考虑了“物理内存不够，需要磁盘来补充”的早期分页想法之一，然而如今物理内存未必小于虚拟内存。另一个分页的想法来自于 虚拟化为不同进程带来的保护。那么，我们应当注意到，不同进程的地址空间不同。

- 假设进程A, B, C, D分别连续访问了4KiB个字节
  - 由于四个进程的地址空间不同，他们分别申请了四个不同的16KiB页面而无法互相利用。
  - 如果进程之间频繁切换而内存不够大（比如只有16KiB左右）的话，就会出现页面的频繁替换。

- 假设进程A连续访问了16KiB个字节，
  - 我们使用16KiB的分页方案，只有一次compulsory miss。
  - 这一次就非常合理地利用了内存空间。

可见，使用多大的页面，和各种进程平均需要访问的内容多少有关系。

#### 从分段的角度来看

过大的block会导致内存碎片化的加剧，但是另一方面也减少了内存碎片化。

从外部碎片化的角度来看，

从内部碎片化的角度来看

#### 从硬件接口的角度来看

传统的硬盘设备的一个 Sector 往往就是4KiB，也就是说磁盘一次只能支持 4KiB 大小的传输，这就限制了操作系统的设计也将页面大小设计为4KiB。

然而，近年来，硬盘设备技术发展，16KiB的传输大小并不是什么难事。

---


### 为什么选择用Rust语言做Project？

本次 Project 我们没有使用Lab中的C语言代码，而是根据rCore的指导重新构建出一个操作系统。Rust是一门赋予每个人构建可靠且高效软件能力的语言，具有**高性能、可靠性、生产力**这三大优势。[^7]本次我们使用Rust开发操作系统Project的过程中，充分运用了这三大优势。

#### 生产力。

1. 可拔插的语言特性。

   比起C语言和C++写操作系统没有任何标准库，Rust语言在设计时便考虑了裸机环境、嵌入式设备下没有标准库的问题。使得我们在编写操作系统的时候，仍然可以使用core核心库。

   此外，只要在具体平台上编写相应的语言特性，便可以逐步恢复标准库的功能。

2. 优质的包管理器。

   比如，riscv库。

   相比用C语言做这次Project

3. 出色的文档

   比如，cargo doc

   相比用C语言做这次Project

4. 清晰的错误提示

   比如，

   相比用C语言做这次Project

#### 可靠性。

Rust 丰富的类型系统和所有权模型保证了内存安全和线程安全，让您在编译期就能够消除各种各样的错误。[^7]

举个例子，Frame Tracker的设计。

#### 高性能。

Rust 和C++语言一样，提供了“零开销”的抽象。我们用Rust写操作系统，不仅不会比C语言慢，还可能因为更加清晰的类型系统和所有权机制使得导致变慢的操作在编译时就消除，从而获得更好的内存利用率。

---



## 应用程序与基本执行环境

### 16KiB下的QEMU

### Rust 执行环境



## 16KiB下内存布局与地址空间的设计

### 

### 实验：cargo bench 测试16KiB页面的性能



## 实现多道程序、分时多任务与进程。





# 参考文献

[^1]: R. H. Arpaci-Dusseau and A. C. Arpaci-Dusseau, *Operating systems: Three easy pieces*, 1.00. Arpaci-Dusseau Books, 2018.
[^2]: “rCore-Tutorial-Book-v3 3.6.0-alpha.1 文档.” http://rcore-os.cn/rCore-Tutorial-Book-v3/index.html (accessed Jul. 27, 2022).
[^3]: “rCore-Tutorial V3 实验指导书” http://rcore-os.cn/rCore-Tutorial-deploy/ (accessed Jul. 21, 2022).
[^4]: “Rust 语言圣经 - Rust语言圣经(Rust Course).” https://course.rs/about-book.html (accessed Jul. 21, 2022).
[^5]: “uCore-Tutorial-Guide-2022S 0.1 文档.” https://learningos.github.io/uCore-Tutorial-Guide-2022S/index.html (accessed Jul. 27, 2022).
[^6]: "RISCV 中文手册." https://riscvbook.com/chinese/RISC-V-Reader-Chinese-v2p1.pdf (accessed Jul. 21, 2022).
[^7]: Rust 程序设计语言.” https://www.rust-lang.org/zh-CN/ (accessed Jul. 29, 2022).

