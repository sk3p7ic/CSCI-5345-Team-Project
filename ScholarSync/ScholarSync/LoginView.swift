import SwiftUI

struct LoginView: View {
    @State private var name: String = ""
    @AppStorage("isLoggedIn") private var isLoggedIn: Bool = false
    @AppStorage("userName") private var userName: String = ""

    var body: some View {
        VStack {
            Spacer()
            
            Text("ScholarSync")
                .font(.largeTitle.bold())
                .foregroundColor(.blue)
                .padding(.bottom, 20)
            
            VStack(spacing: 16) {
                TextField("Enter your name", text: $name)
                    .textFieldStyle(RoundedBorderTextFieldStyle())
                    .padding()
                    .background(Color(.systemGray6))
                    .cornerRadius(10)
                    .shadow(radius: 2)
            }
            .padding(.horizontal, 20)
            
            Button(action: {
                userName = name
                isLoggedIn = true
            }) {
                Text("Login")
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(Color.blue)
                    .foregroundColor(.white)
                    .cornerRadius(10)
                    .shadow(radius: 2)
            }
            .padding(.horizontal, 20)
            .disabled(name.isEmpty)
            
            Spacer()
        }
        .padding()
    }
}

#Preview {
    LoginView()
}
