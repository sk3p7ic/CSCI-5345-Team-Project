//
//  ProfessorView.swift
//  ScholarSync
//
//  Created by Joshua Ibrom on 3/18/25.
//

import SwiftUI

struct ProfessorView: View {
    var professor: Professor
    
    var body: some View {
        VStack(alignment: .leading) {
            VStack(alignment: .leading) {
                Text(professor.name)
                    .font(.title.bold())
                Text(professor.dept.capitalized)
                    .font(.title2.bold())
                Text(professor.desc)
            }
            .padding()
            Text("Papers")
                .font(.title3.bold())
                .padding(.leading)
            if professor.papers.isEmpty {
                Text("No papers found.")
                    .padding(.leading)
            } else {
                ForEach(professor.papers, id: \.self) { paper in
                    Text("- \(paper.title)")
                        .padding(.leading)
                }
            }
        }
        .navigationTitle(professor.name)
        Spacer()
    }
}

#Preview {
    ProfessorView(professor: testData.myProfile)
}
