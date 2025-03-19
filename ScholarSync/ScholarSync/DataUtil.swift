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
    var myProfile: Professor
    var professors: [Professor]
}

var testData: JSONDataFormat = load("data.json")

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
