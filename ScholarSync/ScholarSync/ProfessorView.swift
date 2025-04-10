//
//  ProfessorView.swift
//  ScholarSync
//
//  Created by Joshua Ibrom on 3/18/25.
import SwiftUI

struct ProfessorView: View {
    var professor: Professor
    @State private var showContactInfo = false

    var body: some View {
        VStack(alignment: .leading) {
            // Professor Header
            HStack {
                VStack(alignment: .leading) {
                    Image(systemName: "person.fill")
                        .resizable()
                        .frame(width: 72, height: 72)
                        .foregroundColor(.white)
                        .background(Color.blue)
                        .clipShape(Circle())
                        .padding(.leading, 10)
                    Text(professor.name)
                        .font(.title.bold())

                    Text(professor.dept.capitalized)
                        .font(.title2)
                        .foregroundColor(.blue)

                    Text(professor.desc)
                        .foregroundColor(.secondary)
                }
            }
            .padding()

            // Research Papers Section
            Text("Research Papers")
                .font(.title3.bold())
                .padding(.horizontal)

            if professor.papers.isEmpty {
                Text("No papers found.")
                    .foregroundColor(.gray)
                    .padding(.horizontal)
            } else {
                ForEach(professor.papers, id: \.self) { paper in
                    HStack {
                        Text("- \(paper.title)")
                            .padding(.leading)
                        Spacer()
                    }
                    .padding(.vertical, 5)
                }
            }

            // Contact Info Section
            VStack {
                Text("Interested in this research?")
                    .font(.headline)
                    .foregroundColor(.blue)
                    .padding(.top, 20)

                Button(action: {
                    showContactInfo.toggle()
                }) {
                    HStack {
                        Image(systemName: "envelope.fill")
                            .foregroundColor(.white)
                        Text("Contact Info")
                            .foregroundColor(.white)
                            .bold()
                    }
                    .padding()
                    .frame(maxWidth: .infinity)
                    .background(Color.blue)
                    .cornerRadius(10)
                    .shadow(radius: 2)
                }
                .padding(.horizontal)
                .padding(.top, 5)
            }

            Spacer()
        }
        .padding(.bottom, 20)
        .navigationTitle(professor.name)
        .sheet(isPresented: $showContactInfo) {
            ContactInfoView(professor: professor)
        }
    }
}

#Preview {
    ProfessorView(professor: myProfile)
}
