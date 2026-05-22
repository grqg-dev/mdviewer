import AppKit
import Foundation

final class LaunchDelegate: NSObject, NSApplicationDelegate {
    var paths: [String] = []
    private var didStop = false

    func application(_ sender: NSApplication, openFiles filenames: [String]) {
        paths = filenames
        stopApp()
    }

    func applicationDidFinishLaunching(_ notification: Notification) {
        let cliArgs = Array(CommandLine.arguments.dropFirst())
        if !cliArgs.isEmpty {
            paths = cliArgs
            stopApp()
            return
        }

        // Give Launch Services a moment to deliver openFiles for `open -a App file.md`.
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.35) { [weak self] in
            self?.stopApp()
        }
    }

    private func stopApp() {
        guard !didStop else { return }
        didStop = true
        NSApp.stop(nil)
    }
}

let app = NSApplication.shared
app.setActivationPolicy(.accessory)

let delegate = LaunchDelegate()
app.delegate = delegate
app.run()

guard let bundleBin = Bundle.main.executableURL else {
    fputs("mdviewer: missing bundle executable\n", stderr)
    exit(1)
}

let realBin = bundleBin.deletingLastPathComponent().appendingPathComponent("mdviewer-bin")

let task = Process()
task.executableURL = realBin
if !delegate.paths.isEmpty {
    task.arguments = delegate.paths
}

do {
    try task.run()
    task.waitUntilExit()
    exit(task.terminationStatus)
} catch {
    fputs("mdviewer: failed to launch viewer: \(error)\n", stderr)
    exit(1)
}
