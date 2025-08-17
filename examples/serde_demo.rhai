let data = #{ name: "Alice", age: 30 };
let json = to_json(data);
print(json);
let parsed = from_json(json);
print(`${parsed.name} is ${parsed.age} years old`);
parsed
