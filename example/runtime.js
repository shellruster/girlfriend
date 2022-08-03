// Test console
console.log("Hello world!");

// Test API
const some = await fetch("https://uwussi.moe/api")
console.log(await some.json());

// Global
console.log(globalThis);