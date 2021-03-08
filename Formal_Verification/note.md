# 形式化验证


## 验证方法分类 Page4

- 模拟： 动态验证， 简单易行， 主流方法
- 仿真： 依赖于硬件
- 形式化验证：静态验证， 完备性高，重要补充
- 半形式化验证：两者结合，越来越重要的方法
    - 基于断言的验证
 

对于工具封锁


## 形式化验证
 - 形式化验证
    - 从硬件设计上， 是指从数学上完备地证明或验证电路的实现方案是否是实现了电路设计描述的功能
    - 从软件设计上， 是指从数据上完备地证明或验证系统的软件是否满足系统的需求规范

## 形式化验证的分类

按照验证的内容或需要分类
- 性质验证（PROPERTY CHECKING) 
    - 一个设计是否包含某些设计和功能
- 等价性验证
    两个设计在功能上是否等价？
    - 硬件设计： 功能及性质说明 - 行为设计
    - 软件设计

按照验证方法分类
- 定理证明 Theory Proving 最难的
- 模型验证 model checking
- 定价性验证 equivalence proving

### 等价性验证

- 问题
    - 给定两个组个电路，检验在所有输入下，这两个电路的输出是否一致
- 输入分类： 使输出为0， 输出为1， 不管项
- 等价条件：使他们输出不同的每个输入向量，至少是其中一个电路的不管项。
- 工具
    - Affirma
    - Logic 
    - 
    - 
- 途径： 符号方法和增量方法
- 符号方法：
    - 将问题形式化成为符号表示， 然后用特定的问题求解，如BDD（二叉判定图）， SAT（）
    - 适用：系统缺乏结构相似性，不能得到系统设计的内部情况时
- 增量方法：
    - 
    - 
## 定理证明
- 使用定理证明器
- 方法：
    - 首先从原始设计中抽取模型，表示成形式逻辑的命题，谓词，定理，推理规划等
    - 需要验证的形式被表示成定理的形式
    - 在验证者的引导下，定理证明器不断地对公理、已证明的定理施加推理规则，产生新的定理，直至导出所需要的定理
- 形式逻辑
    - 一阶逻辑
    - 高阶逻辑
    - 时态逻辑
- 优点
    - 高度抽象， 强大的逻辑表达能力
    - 应用不受限制，可以表示和验证几乎所有的系统行为的特性
- 缺点
    - 需要人工引导
    - 需要验证者有良好的数学训练和经验
    - 

## 定理证明系统
- 建立在某种形式逻辑基础上
- 内置各种推理规划，推理对策， 元对策等， 成为验证复杂系统的有力工具
- 从公理开始，寻找一个证明序列
- 改序列宏的每个元素都是从前面的公理和定理推导出来的定理
- 证明序列最终以证明的结果出现而结束
- PVA， LCF， ACL2， HOL， Isabelle, coq, 


## 不同的形式逻辑直接亲啊的比较

- 命题逻辑： 传统布尔代数，变量 
- 一阶逻辑（谓词逻辑）： 包含 。。。 量词
- 高阶逻辑： 包括对集合和函数的推导
- 时态逻辑： 包括关于时间操作符

## 模型检验

- 模型检验： 模型检验是针对状态并发系统的一种自动验证技术
- 由美国的Clarke, Emerson, and Sistla等和法国的Queille and Sifakis 等1981年初期分别独立提出
- 特点
    - 系统用有限状态结构表示
    - 被验证的性质采用时态逻辑公式表示
    - 验证过程就是对设计的状态空间的全搜索过程，确定被验证的性质在状态空间中的可达性、不可达性

- 模型检验的优点和缺点
    - 优点
        - 快速
        - 无人机交互，自动运行
        - 对于给定的性质不存在bug
        - 断定性质不满足时， 能给出反例
    - 挑战
        - 如何解决状态爆炸问题



## 模型检验发展

- 显示模型检验 EMC
    - 基于直线搜索
    - 1981， clarke， queille
- 符号模型检验（SMC）
    - 基于OBDD隐式搜索
    - 1987， McMillan SMV
- 定界模型检验（BMC）
    - 基于SAT
    - 1999， Biere， Clarke
- 无界模型检验（UMC）
    - 基于SAT
    - 2002 

## 模型检验技术
- 离散系统
    - 基于kRIPKE结构模型检验
        - SMV， muSMV， SPIN， VIS
    - 基于时间自动机的模型检验
        - UPPALL
    - 基于概率（统计）的模型检验
        - UPPAAL
        - Prism
- 混成系统
    - 离散、连续模型

## 形式化验证研究的发展

- 70年代
    - 
- 80年代
    - 
- 90年代
    - 门级
- 2000年以来
    - RTL等价验证与模型检验
    - 出现专门的验证公司

## 形式化验证相关的图另加奖得主

- 
- 
-

## validation methods

- validation 
    - testing / simulation 
        - testing
            -   
        - simulating
            - 不做研究
    - verfuation
        - deductive  
            - semi-automatic with theorem prover
        - algorithmic 
            - fully automatic with modek checker 

## 课程教授内容
    - testing
    - deductive
    - algorithmic


## Model checking 

- Verify finite state concurrent system
- Perform automatically
- Given sufficient resoureces, terminate with a yes/ not answer
- Application 
    - operating systems
    - control systems for machines, hardware systems
    - communoication protocols
- Restricting Unbounded data structure tp sepcific instances

## The process of module checking 
- Moduling
    - Compilation task: convent a design into a formalism
    - The use of abstraction to eliminate irrelevant or unimportant
- Specification 
    - The property that the design must satisfy , in some logical formalism 
    - Temporal logic
    - Completeness: specification can not cover all properties that the system should satisfy
- Verification
    - Whether the system odel satisfies the properties
    - The analysia of the verfiction result - human assistance 
    - For a negative result , provide an error trace(counter example)
        - incorrect modeling of the system
        - incorrect specification = false negative 
    - Due to the size of the model, the verfication will fail to terminate normally
        - using abstractions
        - Adjusting the model
## state explosion
- Given n processes represent ed but program graphs P1, ..., Pn with 2 locations each 
- on the set of variable 
- 
- 

## Temporal logic and model checking 
- Temporal logic
    - Linear structure
    - Branching structute
- Temporal lOGIC 
    - dedcribe the ordering of events in time without introducting time explicitly 
    - Burstall Kroger, Pnueli using temporal

    - By Clarke and Emerson -CTL
        - A single model satisfies a formula
        - Complexity: 
        - 
        - 
        - 
    - Model checing for a variety for tempporal logics
        - LTL is 
        - LTL
        - CTL*
    - Other techniques for verifying concurrent system 
        - Automata theory 
            - use automata for the spec and an impl 

        - 

## Symbolic algorithm
- McMillan used a symbolica representation for the state transition graphs 
    - OBDD 
    - 
    - 
- Tool -> 

## 总结
- 形式化方法概述
- 模型验证技术概述

# Chapter 2 Modeling System

## Formal Model
- Comstruct a formal model of s system
    - Specifying key preperties of the system
    - Abstract away details 
    - For digital circuits
    - For 

- State
    - 捕获某一个时候的所有状态的值
- Trasition
    - 状态的跃迁 变化
- Computation
    - 无限状态序列

- 状态转移系统 Kripke structtue
- FIRST order logic 
- extract the kripke structure

## kS 定义


