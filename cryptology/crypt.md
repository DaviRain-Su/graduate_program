#  图解密码技术

## ch1 环游密码世界

### 1.2 密码

#### 发送者 接受者 窃听者

发送者 Sender: Alice 

接受者 receiver: Bob

消息 message: 被发送的消息

窃听者 eavesdropper: 窃听消息

#### 加密与解密

明文 plaintext: 加密之前的消息

密文 ciphertext: 加密之后的消息

解密 decrypt: 对密文解密

#### 破译

解密： 正当的接受者将密文还原为明文

密码破译 cryptanalysis， 破译，密码分析 : 接受者以外的其他人试图将密文还原为明文

破译者 cryptanalyst ,进行破译的人

### 1.3 对称密码与公钥密码

#### 密码算法

算法 algorithm, 用于解决复杂问题的步骤

加密算法， 从明文生成密文的步骤

解密算法， 解密的步骤

密码算法， 加密，解密的算法合在一起统称

#### 密钥

#### 对称密码与公钥密码

对称密码 symmetric cryptography

(公共密钥密码common-key cryptography，传统密码convention cryptography， 私钥密码secret-key cryptography， 共享密钥密码shared-key cryptography)： 指在加密和解密时使用同一密钥的方式

公钥密码 public key cryptography （非对称密码 asymmetric cryptgraphy）： 指在加密和解密时使用不同的密钥的方式。 

#### 混合密码系统

混合密码系统 bybrid cryptosystem 将对称密码与公钥密码结合起来的密码方式

### 其他密码技术

#### 单向散列函数

散列值 为了防止软件被篡改 有安全意识的软件发布者会在发布软件的同时发布该软件的散列值

单向散列函数(one-way hash function) 散列值就是用单向散列函数计算出来的数值。

完整性integrity， 指的是“数据正牌的而不是伪造的" 

使用单向散列函数，就可以检测出数据是否被篡改过。

#### 消息认证码  message aurhentication code 

认证 authentication 

#### 数字签名 ditital signature 

伪装 spoofing 

否认 repudiation

验证 verify 



#### 伪随机数生成器 Pseudo Random Number Generator, PRNG

能够模拟产生随机数列的算法 

### 1.5 密码学家的工具箱

- 对称密码
- 公钥密码
- 单向散列函数
- 消息认证码
- 数字签名
- 伪随机数生成器

### 1.6 隐写术与数字水印

密码隐藏的是内容，隐写术隐藏的是消息本身



## ch2 历史上的密码 



## 内容



- 卡撒密码
- 简单替换密码
- Enigma 

破解密码的方法：

- 暴力攻击
- 频率分析



