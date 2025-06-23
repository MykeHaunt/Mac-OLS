import Foundation

class RustIntegration {
    static func loadBinary(path: String) -> BinaryFile? {
        let result = ecutil_load_file(path)
        return result?.pointee
    }
    
    static func detectAxes(file: BinaryFile) -> [Axis1D] {
        var axes = [Axis1D]()
        let count = ecutil_detect_axes_count(file)
        for i in 0..<count {
            if let axis = ecutil_get_axis(file, i) {
                axes.append(axis.pointee)
            }
        }
        return axes
    }
}
