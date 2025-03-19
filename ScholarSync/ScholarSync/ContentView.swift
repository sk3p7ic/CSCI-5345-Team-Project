//
//  ContentView.swift
//  ScholarSync
//
//  Created by Joshua Ibrom on 3/18/25.
//

import SwiftUI

struct ContentView: View {
    let profile = testData.myProfile
    let professors = testData.professors
    
    var body: some View {
        NavigationStack {
            VStack(alignment: .leading) {
                VStack(alignment: .leading) {
                    Text(profile.name)
                        .font(.title)
                    Text(profile.dept)
                        .font(.title2)
                    Text(profile.desc)
                }
                .padding()
                
                List(professors) { professor in
                    NavigationLink(destination: ProfessorView(professor: professor)) {
                        VStack(alignment: .leading) {
                            Text("\(professor.name), \(professor.dept)")
                                .font(.headline)
                            Text(professor.desc)
                        }
                    }
                }
            }
            .navigationTitle(Text("ScholarSync"))
        }
    }
}

#Preview {
    ContentView()
}
