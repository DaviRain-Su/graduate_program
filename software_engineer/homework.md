1 简答题：什么是软件危机？它有什么典型表现？为什么会出现软件危机？

- 什么是软件危机？
  - 软件危机是一种现象，是指由软件复杂程度越来越高，在计算机软件开发和维护时所遇到的一系列问题
- 它有什么典型表现？
  - 软件开发成本高，成本难以控制
  - 研制周期长，软件开发进度难以控制，周期拖得很长
  - 正确性难保证，软件质量差，可靠性难以保证
  - 软件维护困难，维护人员和维护费用不断增长
  - 软件发展跟不上硬件的发展和用户的要求
- 为什么会出现软件危机？
  - 因为软件的数量急剧膨胀，软件需求日益复杂，维护的难度越来越大，开发成本令人吃惊地高，而失败的软件开发项目却屡见不鲜，不同的原因导致了软件危机的出现。

2 简答题： 什么是软件过程？它与软件工程方法学有何关系？

- 什么是软件过程？
  - ·软件过程是产生一个软件系统的一系列活动。软件过程模型是这些过程的抽象表示。
- 他与软件工程方法学有何关系
  - 软件过程：是一个为了获得高质量软件所需完成的任务框架，也就说说软件过程规定了软件产品开发时完成各项任务的一系列工作步骤，包括中间产品，资源、角色及过程中采取的方法、工具等范畴。
  - 软件工程方法学：通常把在软件生命周期的全过程中的一整套技术方法的集合称为方法学，
  - 软件工程方法学包含三个要素：方法、工具和过程
  - 总结：软件过程式软件工程方法学的一个要素。软件过程是软件工程方法学的三个重要组成部分

3 简答题： 什么是软件生命周期模型？试比较瀑布模型，原型模型，增量模型和螺旋模型的优缺点，说明每种模型的适用范围？

- 什么是软件生命周期模型

  - 软件生命周期：如同任何事物都有一个发生、发展、成熟直至衰亡的全过程一样，软件系统或软件产品也有一个定义、开发、运行维护直至被淘汰这样的全过程，我们把软件要经历的这个全过程称为软件的生命周。它包含：软件定义、软件开发、软件运行维护三个时期，并可以细分为可行性研究、项目计划、需求分析、概要设计、详细设计、编码实现与单元测试、系统集成测试、系统确认验证、系统运行与维护等几个阶段。
  - 为了是软件生命周期中各项任务能够有序地按照规程进行，需要一定的工作模型对各项任务给以规程约束，这样的工作模型被称为软件过程模型，或者软件生命周期模型。它是一个有关项目的结构框架，规定了软件生命周期内各项任务的执行步骤与目标。

- 试比较瀑布模型、原型模型、增量模型、和螺旋模型的优点和缺点，说明每种模型的适用范围

  - |          | 优点                                                         | 缺点                                                         | 使用范围                                                     |
    | -------- | ------------------------------------------------------------ | ------------------------------------------------------------ | ------------------------------------------------------------ |
    | 瀑布模型 | 1. 有利于大型软件开发过程中人员的组织、管理。2，有利于软件开发方法和功能的研究，从而提高大型软件项目开发的质量和效率 | 1.开发过程一般不能逆转，否则代价太大，2.实际的项目开发很难严格按该模型进行3.客户往往很难清除地给出所有的需求，而该模型却要求如此，4.软件的实际情况必须到项目开发的后期客户才能看到，这要求客户有足够的耐心。 | 1.用户的需求非常清楚全面，且在开发过程中没有货很少变化2.开发人员对软件的应用领域很熟悉3.用户的使用环境非常稳定4.开发工作对用户参与要求很低 |
    | 原型模型 | 1.原型法在得到良好的需求定义上比传统生命周期法好得多，可处理模糊需求，开发者和用户可充分通信。2原型系统可作为培训环境，有利于用户培训和开发同步，开发过程也是学习过程。3.原型给用户以机会更改心中原先设想的，不合理的最终系统。4原型可低风险开发柔性较大的计算机系统。5原型改善了系统维护性、对用户友好性，降低了维护成本。 | 1.用户有时误解了原型的角色例如他们可能误解原型应该和真实系统一样可靠。2.缺少项目标准，进化原型法有点像编码修正。3缺少控制，由于用户可能不断提出新要求，因而原型迭代的周期很难控制。4额外的花费，研究结果表明构造一个原型可能需要10%额外花费。5，运行效率可能会受到影响6原型法要求开发者与用户密切接触，有时这是不可能的，例如外包软件。 | 1. 对所开发的领域比较熟悉而且有快速原型开发工具2目招投标时，可以以原型模型作为软件的开发模型3.进行产品移植或升级时，或对已有产品原型进行客户化工作，原型模型是非常适合的 |
    | 增量模型 | 1. 人员分配灵活，刚开始不同投入大量人力资源，当核心产品很受欢迎时，可增加人力实现下一增量。2. 当配备的人员不能在设定的期限内完成产品时，它提供了一种先退出核心产品的途径，这样就可以先发布部分功能给客户，对客户起到镇定剂的作用3.增量能够有计划地管理技术风险。 | 1由于各个构建是逐渐并入已有的软件体系结构，所以加入构件必须不破坏已构造好的系统部分，这需要软件具备开放式的体系结构。2.在开发过程中，需求的变化是不可避免的，增量模型的灵活性可以使其适应这种变化的能力大大优于瀑布模型和快速原型模型，但也很容易退化为"编码修正"模型，从而是软件过程的控制失去整体性。3.如果增量包之间存在相交的情况且未很好处理，则必须做全盘系统分析，这种模型将功能细化后分别开发的方法较适应于需求经常改变的软件开发过程4.自始至终需要客户和开发者在一起，直到完全版本出来，这个条件比较难做到。 | 1.进行已有产品升级或新版本开发，增量模型是非常适合的。2.对完成期限严格要求的产品，可以使用增量模型3.对所开发的领域比较熟悉而且已有原型系统，增量模型也是非常适合的 |
    | 螺旋模型 | 1.对于大型系统及软件的开发，这种模型是一个很好的方法。开发者和客户能够较好的对待和理解每一个演化级别上的风险。2.以小的分段来构建大型系统，使成本计算变得简单容易。3客户始终参与每个阶段的开发，保证了项目不偏离正确方向以及项目的可控性。4随着项目推进，客户始终掌握项目的最新信息，从而她或她能够和管理层有效地交互。5客户认可这种公司内部的开发方式带来的良好的沟通和高质量的产品 | 1.需要相当的风险分析评估的专门技术，且成功依赖于这种技术。2.很明显一个大的没有被发现的风险问题，将会导致问题的发生，可能导致演化的方法失去控制。3这种模型相对比较新，应用不广泛，其功效需要进一步的验证。 | 只适合大规模的软件项目                                       |

4 简答题 ： 软件过程的通用过程框架包含哪两种活动？

- 框架活动和保护性活动