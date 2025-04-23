//
//  DataUtil.swift
//  ScholarSync
//
//  Created by Joshua Ibrom on 3/18/25.
//
import Foundation

struct Paper: Hashable, Codable, Identifiable {
    var id: Int
    var title: String
}

struct Professor: Hashable, Codable, Identifiable {
    var id: Int
    var name: String
    var dept: String
    var desc: String
    var papers: [Paper]
}

struct JSONDataFormat: Hashable, Codable {
    var myProfileIdx: Int
    var professors: [Professor]
}

// Load data initially
var testData: JSONDataFormat = load("data.json")
var myProfile = testData.professors[testData.myProfileIdx]

func load<T: Decodable>(_ filename: String) -> T {
    let data: Data

    guard let file = Bundle.main.url(forResource: filename, withExtension: nil)
    else {
        fatalError("Couldn't find \(filename) in main bundle.")
    }

    do {
        data = try Data(contentsOf: file)
    } catch {
        fatalError("Couldn't load \(filename) from main bundle:\n\(error)")
    }

    do {
        let decoder = JSONDecoder()
        return try decoder.decode(T.self, from: data)
    } catch {
        fatalError("Couldn't parse \(filename) as \(T.self):\n\(error)")
    }
}

let addr: String = "http://127.0.0.1:8080/api/professors"

class DataManager: ObservableObject {
    @Published var professors: [Professor] = []
    @Published var myProfile: Int? = nil
    
    func fetchData() {
        guard let url = URL(string: addr) else {
            print("Invalid URL: \(addr)")
            return
        }
        
        URLSession.shared.dataTask(with: url) { data, response, error in
            if let data = data {
                do {
                    let professors = try JSONDecoder().decode([Professor].self, from: data)
                    self.professors = professors
                    self.myProfile = 0
                } catch {
                    print("Failed to parse JSON: \(error)")
                }
            } else if let error = error {
                print("Error fetching data: \(error)")
            }
        }.resume()
    }
}
