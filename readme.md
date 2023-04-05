<header>

<div align="center">
<img src=".github/assets/logo.png" alt="logo" height="120" align="center">
<h1 align="center">Girlfriend Shell</h1>

<p>Some javascript for your shell</p>

<a href="https://crates.io/crates/deno_ast">
	<img src="https://img.shields.io/badge/Deno%20AST-v0.22.0-f36caf.svg?style=flat-square" alt="Bot API Version" />
</a>
<a href="https://github.com/phoenixifier/girlfriend">
	<img src="https://img.shields.io/github/languages/top/phoenixifier/girlfriend?style=flat-square&logo=github" alt="GitHub top language" />
</a>
</div>

</header>

UPDATE 2023-02-07: This repo has been updated to support loading JavaScript and
TypeScript files.

```shellsession
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/runjs ./test.js`
[out]: "Hello" "runjs!"
[err]: "Boom!"
[err]: "Unable to read file" "./log.txt" {"code":"ENOENT"}
[out]: "Read from a file" "./log.txt" "contents:" "I can write to a file."
[out]: "Removing file" "./log.txt"
[out]: "File removed"
```
