import SwiftUI

struct ProfileView: View {
    @State private var profile = testData.myProfile
    @State private var newPaperTitle = ""

    var body: some View {
        NavigationStack {
            VStack {
                // Profile Header
                VStack {
                    Image(systemName: "person.crop.circle.fill")
                        .resizable()
                        .frame(width: 100, height: 100)
                        .foregroundColor(.blue)
                        .padding(.top, 20)

                    Text(profile.name)
                        .font(.title.bold())
                        .padding(.top, 5)

                    Text(profile.dept)
                        .font(.title2)
                        .foregroundColor(.gray)
                    
                    Text(profile.desc)
                        .font(.subheadline)
                        .foregroundStyle(.primary)
                }
                .padding(.bottom, 20)

                // Papers Section
                Text("Your Research Papers")
                    .font(.headline)
                    .padding(.leading)
                    .frame(maxWidth: .infinity, alignment: .leading)

                ScrollView {
                    VStack(spacing: 10) {
                        ForEach(profile.papers, id: \.id) { paper in
                            HStack {
                                VStack(alignment: .leading) {
                                    Text(paper.title)
                                        .font(.body)
                                        .foregroundColor(.primary)
                                    Text("Research Paper")
                                        .font(.caption)
                                        .foregroundColor(.secondary)
                                }
                                Spacer()
                                Button(action: {
                                    deletePaper(paper)
                                }) {
                                    Image(systemName: "trash")
                                        .foregroundColor(.red)
                                }
                            }
                            .padding()
                            .background(Color.white)
                            .cornerRadius(10)
                            .shadow(radius: 2)
                            .padding(.horizontal)
                        }
                    }
                }
                .padding(.bottom, 20)

                // Add New Paper Section
                VStack {
                    TextField("Enter paper title", text: $newPaperTitle)
                        .textFieldStyle(RoundedBorderTextFieldStyle())
                        .padding()

                    Button(action: addPaper) {
                        HStack {
                            Image(systemName: "plus.circle.fill")
                                .foregroundColor(.white)
                            Text("Add Paper")
                                .foregroundColor(.white)
                        }
                        .padding()
                        .frame(maxWidth: .infinity)
                        .background(Color.blue)
                        .cornerRadius(10)
                    }
                    .padding(.horizontal)
                }
                .padding(.bottom, 20)

                Spacer()
            }
            .navigationTitle("My Profile")
            .background(Color(.systemGroupedBackground))
        }
    }

    // Add New Paper
    func addPaper() {
        if !newPaperTitle.isEmpty {
            let newPaper = Paper(id: profile.papers.count + 1, title: newPaperTitle)
            profile.papers.append(newPaper)
            saveProfile()
            newPaperTitle = ""
        }
    }

    // Delete Paper
    func deletePaper(_ paper: Paper) {
        profile.papers.removeAll { $0.id == paper.id }
        saveProfile()
    }

    // Save Changes to JSON
    func saveProfile() {
        testData.myProfile = profile
    }
}

#Preview {
    ProfileView()
}
