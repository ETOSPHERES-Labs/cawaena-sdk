import ETOPaySdkBin
import SystemConfiguration

// Extend the converted Rust strings into Swift strings (RustString) to print them as error strings
// This is not really safe, but it makes it compile.
// See: https://github.com/chinedufn/swift-bridge/issues/309
extension RustString: @unchecked Sendable {}
extension RustString: Error {}
