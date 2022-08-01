console.log("Hello", "runjs!");
console.error("Boom!");

const path = "./log.txt";
try {
    const contents = await runjs.readFile(path);
    console.log("Read from a file", contents);
} catch (err) {
    console.error("Unable to read file", path, err);
}

await runjs.writeFile(path, "I can write to a file.");
const contents = await runjs.readFile(path);
console.log("Read from a file", path, "contents:", contents);
console.log("Removing file", path);
runjs.removeFile(path);
console.log("File removed");

// TODO: Implement fetcher for girlfriend
// const fetcher = await fetch("https://www.uwussi.moe/api/minecraft")
// console.log(await fetcher.text())