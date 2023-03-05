//UART 线性控制寄存器
register! {
    UartLineControl,
    u32,
    RW,
    Fields [
        //指出发送接收每帧的数据位数 00 = 5-bits 01 = 6-bits 10 = 7-bits 11 = 8-bits
        WordLength          WIDTH(U2) OFFSET(U0) [
            // 5bits = Clear
            Bits6 = U1,
            Bits7 = U2,
            Bits8 = U3
        ]
        //定义度搜按个停止位用于帧末信号 0 =每帧一个停止位 1 =每帧两个停止位
        NumberOfStopBit     WIDTH(U1) OFFSET(U2),
        ParityMode          WIDTH(U3) OFFSET(U3),
        //决定是否使用红外模式 0=正常模式操作 1=红外接收发送模式
        InfraredMode        WIDTH(U1) OFFSET(U6), 
        Reserved            WIDTH(U1) OFFSET(U7)
    ]
}

//UART 控制寄存器
register! {
    UartControl,
    u32,
    RW,
    Fields [
        ReceiveMode                     WIDTH(U2) OFFSET(U0) [
            Polling = U1,
            DMA = U2, // DMA0 请求 (Only for UART0)  DMA3 请求 (Only for UART2)
            DMA1 = U3 //  DMA1 请求 (Only for UART1)
        ],
        TransmitMode                    WIDTH(U2) OFFSET(U2) [
            Polling = U1,
            DMA = U2, // DMA0 请求 (Only for UART0)  DMA3 请求 (Only for UART2)
            DMA1 = U3 //  DMA1 请求 (Only for UART1)
        ]
        SendBreakSignal                 WIDTH(U1) OFFSET(U4),
        LoopbackMode                    WIDTH(U1) OFFSET(U5),
        RxErrorStatusInterruptEnable    WIDTH(U1) OFFSET(U6),
        RxTimeoutEnable                 WIDTH(U1) OFFSET(U7),
        RxInterruptType                 WIDTH(U1) OFFSET(U8),
        TxInterruptType                 WIDTH(U1) OFFSET(U9),
        ClockSelection                  WIDTH(U2) OFFSET(U10) [
            UEXTCLK = U1,
            PCLK = U2,
            FCLK = U3
        ],
        FCLKDivider                     WIDTH(U4) OFFSET(U12)
    ]
}

//UART FIFO 控制寄存器
register! {
    UartFIFOControl,
    u32,
    RW,
    Fields [
        FIFOEnable                      WIDTH(U1) OFFSET(U0),
        RxFIFOReset                     WIDTH(U1) OFFSET(U1),
        TxFIFOReset                     WIDTH(U1) OFFSET(U2),
        Reserved                        WIDTH(U1) OFFSET(U3),
        RxFIFOTriggerLevel              WIDTH(U2) OFFSET(U4), 
        TxFIFOTriggerLevel              WIDTH(U2) OFFSET(U6)
    ]
}

//UART MODEM 控制寄存器
register! {
    UartModemControl,
    u32,
    RW,
    Fields [
        RequestToSend                   WIDTH(U1) OFFSET(U0),
        Reserved1                        WIDTH(U3) OFFSET(U1),
        AutoFlowControl                 WIDTH(U1) OFFSET(U4),
        Reserved2                        WIDTH(U3) OFFSET(U5)
    ]
}

//UART 接收发送状态寄存器
register! {
    UartTxRxStatus,
    u32,
    RW,
    Fields [
        ReceiveBufferDataReady          WIDTH(U1) OFFSET(U0),
        TransmitBufferEmpty             WIDTH(U1) OFFSET(U1),
        TransmitterEmpty                WIDTH(U1) OFFSET(U2)
    ]
}

//UART 错误状态寄存器
register! {
    UartErrorStatus,
    u32,
    RW,
    Fields [
        OverrunError                    WIDTH(U1) OFFSET(U0),
        ParityError                     WIDTH(U1) OFFSET(U1),
        FrameError                      WIDTH(U1) OFFSET(U2),
        BreakDetect                     WIDTH(U1) OFFSET(U3)
    ]
}

//UART FIFO 状态寄存器
register! {
    UartFIFOStatus,
    u32,
    RW,
    Fields [
        RxFIFOCount                     WIDTH(U6) OFFSET(U0),
        RxFIFOFull                      WIDTH(U1) OFFSET(U6),
        Reserved1                       WIDTH(U1) OFFSET(U7),
        TxFIFOCount                     WIDTH(U6) OFFSET(U8),
        TxFIFOFull                      WIDTH(U1) OFFSET(U14),
        Reserved2                       WIDTH(U1) OFFSET(U15)
    ]
}

//UART MODEM 状态寄存器
register! {
    UartModemStatus,
    u32,
    RW,
    Fields [
        ClearToSend                     WIDTH(U1) OFFSET(U0),
        Reserved                        WIDTH(U3) OFFSET(U1),
        DeltaCTS                        WIDTH(U1) OFFSET(U4)
    ]
}

//UART 发送缓存寄存器
register! {
    UartTransmitBuffer,
    u32,
    WO,
    Fields [
        TXDATA                          WIDTH(U8) OFFSET(U0)
    ]
}

//UART 接收缓存寄存器
register! {
    UartReceiveBuffer,
    u32,
    RO,
    Fields [
        RXDATA                          WIDTH(U8) OFFSET(U0)
    ]
}

//UART 波特率除数寄存器
register! {
    UartBaudRateDivisor,
    u32,
    RW,
    Fields [
        UBRDIV    WIDTH(U16) OFFSET(U0) [
            B26 = U26 // 40hmz
        ]
    ]
}