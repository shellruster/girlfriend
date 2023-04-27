// await girlfriend.writeFile("./something.js", "console.log('Hello, girlfriend!');");
// await girlfriend.removeFile("./something.js");

// const list = ls(".");
//
// for (const file of list) {
//     console.log(file);
// }

// mkdir('test');

const content = await curl("https://raw.githubusercontent.com/phoenixifier/girlfriend/main/license")

console.log(content)
