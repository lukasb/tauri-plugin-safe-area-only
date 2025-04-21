import SwiftRs
import Tauri
import UIKit
import WebKit

class SafeAreaOnly: Plugin {
    @objc open override func load(webview: WKWebView) {
        guard let rootVC = UIApplication.shared.keyWindow?.rootViewController else { return }
        let rootView = rootVC.view!
        
        // Ensure the webview is added to the view hierarchy before adding constraints
        // Assuming the webview might already be added by Tauri, but if not, uncomment:
        // rootView.addSubview(webview) 
        
        // Disable autoresizing mask translation into constraints
        webview.translatesAutoresizingMaskIntoConstraints = false

        // Set content inset adjustment behavior (keep this line)
        webview.scrollView.contentInsetAdjustmentBehavior = .always
        
        // Constrain the webview
        NSLayoutConstraint.activate([
            webview.leadingAnchor.constraint(equalTo: rootView.safeAreaLayoutGuide.leadingAnchor),
            webview.trailingAnchor.constraint(equalTo: rootView.safeAreaLayoutGuide.trailingAnchor),
            webview.topAnchor.constraint(equalTo: rootView.safeAreaLayoutGuide.topAnchor),
            webview.bottomAnchor.constraint(equalTo: rootView.bottomAnchor)
        ])
        
        // Optional: Set background colors if needed for debugging or style
        // webview.backgroundColor = .clear
        // webview.scrollView.backgroundColor = .clear
        // UIApplication.shared.keyWindow?.backgroundColor = .white
    }
}

@_cdecl("init_plugin_safe_area_only")
func initPlugin() -> Plugin {
  return SafeAreaOnly()
}
