// swift-tools-version: 5.10
import PackageDescription
import Foundation

let isDevelopment = ProcessInfo.processInfo.environment["YTTRIUM_DEVELOPMENT"] == "false"

let rustBinaryTarget: Target = {
    guard isDevelopment else {
        return rustBinaryTarget = .binaryTarget(
            name: "RustXcframework",
            url: "$REPO_URL/releases/download/$PACKAGE_VERSION/$RUST_XCFRAMEWORK_ZIP",
            checksum: "$RUST_CHECKSUM"
        )
    }
    return .binaryTarget(
        name: "RustXcframework",
        path: "crates/ffi/YttriumCore/RustXcframework.xcframework"
    )
}()

let package = Package(
    name: "yttrium",
    platforms: [
        .macOS(.v14),
        .iOS(.v13),
        .watchOS(.v10),
        .tvOS(.v17)
    ],
    products: [
        .library(
            name: "Yttrium",
            targets: ["Yttrium"]
        ),
    ],
    dependencies: [
        .package(url: "https://github.com/thebarndog/swift-dotenv.git", from: "2.0.0")
    ],
    targets: [
        .target(
            name: "Yttrium",
            dependencies: [
                "YttriumCore",
                .product(name: "SwiftDotenv", package: "swift-dotenv")
            ],
            path: "platforms/swift/Sources/Yttrium"
        ),
        .target(
            name: "YttriumCore",
            dependencies: [
                "RustXcframework"
            ],
            path: "crates/ffi/YttriumCore/Sources/YttriumCore"
        ),
        rustBinaryTarget,
        .testTarget(
            name: "YttriumTests",
            dependencies: [
                "Yttrium" ,
                .product(name: "SwiftDotenv", package: "swift-dotenv")
            ],
            path: "platforms/swift/Tests/YttriumTests"
        ),
    ]
)
