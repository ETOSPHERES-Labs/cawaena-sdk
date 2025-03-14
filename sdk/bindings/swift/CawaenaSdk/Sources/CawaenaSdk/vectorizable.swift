import CawaenaSdkBin
import SystemConfiguration

// Extend the converted Rust strings into Swift strings (RustString) to print them as error strings
extension RustString: @unchecked Sendable {}
extension RustString: Error {}

// Ensure that KycOpenDocument struct conform to the Vectorizable protocol so we can brige Vec<KycOpenDocument>
extension KycOpenDocument: Vectorizable {
    public typealias SelfRef = KycOpenDocument
    public typealias SelfRefMut = KycOpenDocument

    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        return UnsafeMutableRawPointer.allocate(
            byteCount: MemoryLayout<KycOpenDocument>.stride,
            alignment: MemoryLayout<KycOpenDocument>.alignment)
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        vecPtr.deallocate()
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: KycOpenDocument) {
        let valuePtr = vecPtr.bindMemory(to: KycOpenDocument.self, capacity: 1)
        valuePtr.initialize(to: value)
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> KycOpenDocument? {
        let valuePtr = vecPtr.bindMemory(to: KycOpenDocument.self, capacity: 1)
        defer { valuePtr.deinitialize(count: 1) }
        return valuePtr.pointee
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> KycOpenDocument
        .SelfRef?
    {
        guard index == 0 else { return nil }
        let valuePtr = vecPtr.bindMemory(to: KycOpenDocument.self, capacity: 1)
        return valuePtr.pointee
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
        -> KycOpenDocument.SelfRefMut?
    {
        guard index == 0 else { return nil }
        let valuePtr = vecPtr.bindMemory(to: KycOpenDocument.self, capacity: 1)
        return valuePtr.pointee
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
        KycOpenDocument.SelfRef
    > {
        let valuePtr = vecPtr.bindMemory(to: KycOpenDocument.self, capacity: 1)
        return UnsafePointer(valuePtr)
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        return 1
    }
}

// Ensure that KycAmlaQuestion conform to the Vectorizable protocol so we can brige Vec<KycAmlaQuestion>
extension KycAmlaQuestion: Vectorizable {
    public typealias SelfRef = KycAmlaQuestion
    public typealias SelfRefMut = KycAmlaQuestion

    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        return UnsafeMutableRawPointer.allocate(
            byteCount: MemoryLayout<KycAmlaQuestion>.stride,
            alignment: MemoryLayout<KycAmlaQuestion>.alignment)
    }
    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        vecPtr.deallocate()
    }
    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: KycAmlaQuestion) {
        let valuePtr = vecPtr.bindMemory(to: KycAmlaQuestion.self, capacity: 1)
        valuePtr.initialize(to: value)
    }
    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> KycAmlaQuestion? {
        let valuePtr = vecPtr.bindMemory(to: KycAmlaQuestion.self, capacity: 1)
        defer { valuePtr.deinitialize(count: 1) }
        return valuePtr.pointee
    }
    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> KycAmlaQuestion
        .SelfRef?
    {
        guard index == 0 else { return nil }
        let valuePtr = vecPtr.bindMemory(to: KycAmlaQuestion.self, capacity: 1)
        return valuePtr.pointee
    }
    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
        -> KycAmlaQuestion.SelfRefMut?
    {
        guard index == 0 else { return nil }
        let valuePtr = vecPtr.bindMemory(to: KycAmlaQuestion.self, capacity: 1)
        return valuePtr.pointee
    }
    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
        KycAmlaQuestion.SelfRef
    > {
        let valuePtr = vecPtr.bindMemory(to: KycAmlaQuestion.self, capacity: 1)
        return UnsafePointer(valuePtr)
    }
    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        return 1
    }
}

// Ensure that Order conform to the Vectorizable protocol so we can brige Vec<Order>
extension Order: Vectorizable {
    public typealias SelfRef = Order
    public typealias SelfRefMut = Order

    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        return UnsafeMutableRawPointer.allocate(
            byteCount: MemoryLayout<Order>.stride, alignment: MemoryLayout<Order>.alignment)
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        vecPtr.deallocate()
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Order) {
        let valuePtr = vecPtr.bindMemory(to: Order.self, capacity: 1)
        valuePtr.initialize(to: value)
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Order? {
        let valuePtr = vecPtr.bindMemory(to: Order.self, capacity: 1)
        defer { valuePtr.deinitialize(count: 1) }
        return valuePtr.pointee
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Order.SelfRef?
    {
        guard index == 0 else { return nil }
        let valuePtr = vecPtr.bindMemory(to: Order.self, capacity: 1)
        return valuePtr.pointee
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Order
        .SelfRefMut?
    {
        guard index == 0 else { return nil }
        let valuePtr = vecPtr.bindMemory(to: Order.self, capacity: 1)
        return valuePtr.pointee
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
        Order.SelfRef
    > {
        let valuePtr = vecPtr.bindMemory(to: Order.self, capacity: 1)
        return UnsafePointer(valuePtr)
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        return 1
    }
}

// Ensure that TxInfo conform to the Vectorizable protocol so we can brige Vec<TxInfo>
extension TxInfo: Vectorizable {
    public typealias SelfRef = TxInfo
    public typealias SelfRefMut = TxInfo

    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        return UnsafeMutableRawPointer.allocate(
            byteCount: MemoryLayout<TxInfo>.stride, alignment: MemoryLayout<TxInfo>.alignment)
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        vecPtr.deallocate()
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: TxInfo) {
        let valuePtr = vecPtr.bindMemory(to: TxInfo.self, capacity: 1)
        valuePtr.initialize(to: value)
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> TxInfo? {
        let valuePtr = vecPtr.bindMemory(to: TxInfo.self, capacity: 1)
        defer { valuePtr.deinitialize(count: 1) }
        return valuePtr.pointee
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> TxInfo.SelfRef?
    {
        guard index == 0 else { return nil }
        let valuePtr = vecPtr.bindMemory(to: TxInfo.self, capacity: 1)
        return valuePtr.pointee
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> TxInfo
        .SelfRefMut?
    {
        guard index == 0 else { return nil }
        let valuePtr = vecPtr.bindMemory(to: TxInfo.self, capacity: 1)
        return valuePtr.pointee
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
        TxInfo.SelfRef
    > {
        let valuePtr = vecPtr.bindMemory(to: TxInfo.self, capacity: 1)
        return UnsafePointer(valuePtr)
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        return 1
    }
}

// Ensure that WalletTxInfo conform to the Vectorizable protocol so we can brige Vec<WalletTxInfo>
extension WalletTxInfo: Vectorizable {
    public typealias SelfRef = WalletTxInfo
    public typealias SelfRefMut = WalletTxInfo

    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        return UnsafeMutableRawPointer.allocate(
            byteCount: MemoryLayout<WalletTxInfo>.stride,
            alignment: MemoryLayout<WalletTxInfo>.alignment)
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        vecPtr.deallocate()
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: WalletTxInfo) {
        let valuePtr = vecPtr.bindMemory(to: WalletTxInfo.self, capacity: 1)
        valuePtr.initialize(to: value)
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> WalletTxInfo? {
        let valuePtr = vecPtr.bindMemory(to: WalletTxInfo.self, capacity: 1)
        defer { valuePtr.deinitialize(count: 1) }
        return valuePtr.pointee
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> WalletTxInfo
        .SelfRef?
    {
        guard index == 0 else { return nil }
        let valuePtr = vecPtr.bindMemory(to: WalletTxInfo.self, capacity: 1)
        return valuePtr.pointee
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> WalletTxInfo
        .SelfRefMut?
    {
        guard index == 0 else { return nil }
        let valuePtr = vecPtr.bindMemory(to: WalletTxInfo.self, capacity: 1)
        return valuePtr.pointee
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
        WalletTxInfo.SelfRef
    > {
        let valuePtr = vecPtr.bindMemory(to: WalletTxInfo.self, capacity: 1)
        return UnsafePointer(valuePtr)
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        return 1
    }
}

// Ensure that Network conform to the Vectorizable protocol so we can brige Vec<Network>
extension Network: Vectorizable {
    public typealias SelfRef = Network
    public typealias SelfRefMut = Network

    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        return UnsafeMutableRawPointer.allocate(
            byteCount: MemoryLayout<Network>.stride,
            alignment: MemoryLayout<Network>.alignment)
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        vecPtr.deallocate()
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Network) {
        let valuePtr = vecPtr.bindMemory(to: Network.self, capacity: 1)
        valuePtr.initialize(to: value)
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Network? {
        let valuePtr = vecPtr.bindMemory(to: Network.self, capacity: 1)
        defer { valuePtr.deinitialize(count: 1) }
        return valuePtr.pointee
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Network
        .SelfRef?
    {
        guard index == 0 else { return nil }
        let valuePtr = vecPtr.bindMemory(to: Network.self, capacity: 1)
        return valuePtr.pointee
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Network
        .SelfRefMut?
    {
        guard index == 0 else { return nil }
        let valuePtr = vecPtr.bindMemory(to: Network.self, capacity: 1)
        return valuePtr.pointee
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
        Network.SelfRef
    > {
        let valuePtr = vecPtr.bindMemory(to: Network.self, capacity: 1)
        return UnsafePointer(valuePtr)
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        return 1
    }
}
