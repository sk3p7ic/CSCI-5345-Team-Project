//
//  ScholarSyncApp.swift
//  ScholarSync
//
//  Created by Joshua Ibrom on 3/18/25.
//


import SwiftUI

@main
struct ScholarSyncApp: App {
    @AppStorage("isLoggedIn") private var isLoggedIn: Bool = false

    var body: some Scene {
        WindowGroup {
            if isLoggedIn {
                ContentView()
            } else {
                LoginView()
            }
        }
    }
}

