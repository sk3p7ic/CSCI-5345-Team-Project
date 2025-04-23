import SwiftUI

struct ContentView: View {
    @AppStorage("isLoggedIn") private var isLoggedIn: Bool = false
    @StateObject private var dataManager = DataManager()
    @State private var showProfileSheet = false
    @State private var searchText = ""

    var filteredProfessors: [Professor] {
        if searchText.isEmpty {
            return dataManager.professors
        }
        return dataManager.professors.filter { professor in
            professor.name.localizedCaseInsensitiveContains(searchText) ||
            professor.dept.localizedCaseInsensitiveContains(searchText) ||
            professor.papers.contains { paper in
                paper.title.localizedCaseInsensitiveContains(searchText)
            }
        }
    }

    var body: some View {
        NavigationStack {
            VStack {
                // Navigation Bar
                HStack {
                    Text("ScholarSync")
                        .font(.largeTitle.bold())
                        .foregroundColor(.blue)

                    Spacer()

                    // Profile Button
                    Button(action: {
                        showProfileSheet.toggle()
                    }) {
                        Image(systemName: "person.crop.circle.fill")
                            .resizable()
                            .frame(width: 35, height: 35)
                            .foregroundColor(.blue)
                    }

                    // Logout
                    Button(action: {
                        withAnimation {
                            isLoggedIn = false
                        }
                    }) {
                        Image(systemName: "power.circle.fill")
                            .resizable()
                            .frame(width: 35, height: 35)
                            .foregroundColor(.red)
                    }
                }
                .padding()
                .background(Color.white.shadow(radius: 2))

                //  Search Bar
                TextField("🔍 Search professors, departments, or research papers...", text: $searchText)
                    .textFieldStyle(RoundedBorderTextFieldStyle())
                    .padding()
                    .background(Color(.systemGray6))
                    .cornerRadius(10)
                    .shadow(radius: 2)
                    .padding(.horizontal)

                //  Professor List with Cards
                ScrollView {
                    LazyVStack(spacing: 15) {
                        if filteredProfessors.isEmpty {
                            Text("No matching professors found 😔")
                                .foregroundColor(.gray)
                                .padding()
                        } else {
                            ForEach(filteredProfessors) { professor in
                                NavigationLink(destination: ProfessorView(professor: professor)) {
                                    HStack(spacing: 15) {
                                        // ✅ Profile Image Placeholder
                                        Image(systemName: "person.fill")
                                            .resizable()
                                            .frame(width: 60, height: 60)
                                            .foregroundColor(.white)
                                            .background(Color.blue)
                                            .clipShape(Circle())
                                            .padding(.leading, 10)

                                        VStack(alignment: .leading, spacing: 5) {
                                            Text(professor.name)
                                                .font(.headline)
                                                .foregroundColor(.black)
                                                .multilineTextAlignment(.leading)
                                            Text(professor.dept)
                                                .font(.subheadline)
                                                .foregroundColor(.blue)
                                                .multilineTextAlignment(.leading)
                                            Text(professor.desc)
                                                .foregroundColor(.secondary)
                                                .lineLimit(2)
                                                .multilineTextAlignment(.leading)
                                        }
                                        Spacer()
                                    }
                                    .padding()
                                    .frame(maxWidth: .infinity, alignment: .leading)
                                    .background(Color.white)
                                    .cornerRadius(15)
                                    .shadow(color: Color.black.opacity(0.1), radius: 5, x: 0, y: 4)
                                    .padding(.horizontal)
                                    .scaleEffect(1.0)
                                    .onAppear {
                                        withAnimation(Animation.easeInOut(duration: 0.3)) {}
                                    }
                                }
                            }
                        }
                    }
                    .padding(.top, 10)
                }
            }
            .navigationTitle("")
            .navigationBarHidden(true)
            .sheet(isPresented: $showProfileSheet) {
                ProfileView()
            }
            .background(Color(.systemGroupedBackground))
            .onAppear {
                dataManager.fetchData()
            }
        }
    }
}

#Preview {
    ContentView()
}
