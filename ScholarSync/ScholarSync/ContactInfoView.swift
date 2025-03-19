import SwiftUI

struct ContactInfoView: View {
    var professor: Professor

    var body: some View {
        VStack(spacing: 20) {
            Image(systemName: "person.crop.circle.fill")
                .resizable()
                .frame(width: 80, height: 80)
                .foregroundColor(.blue)
                .padding(.top, 20)

            Text(professor.name)
                .font(.title.bold())

            Text(professor.dept)
                .font(.title2)
                .foregroundColor(.gray)

            Divider()

            Text("📧 Email:@university.edu")
                .font(.body)
                .foregroundColor(.blue)
                .padding(.horizontal)

            Text("📞 Phone: +1 (555) 123-4567")
                .font(.body)
                .foregroundColor(.blue)
                .padding(.horizontal)

            Spacer()

            Button(action: {
                // Close sheet
            }) {
                Text("Close")
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(Color.red)
                    .foregroundColor(.white)
                    .cornerRadius(10)
            }
            .padding(.horizontal)
            .padding(.bottom, 20)
        }
        .padding()
        .background(Color(.systemGroupedBackground))
    }
}

#Preview {
    ContactInfoView(professor: testData.myProfile)
}
