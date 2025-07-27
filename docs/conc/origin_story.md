# 🦇 Origin Story

It was a dark and stormy mid-June night. The forecast was clear and it might have actually been midday. But either way! A freshly unemployed engineer found themself in abundance of three things: time, ideas, and a desperate need to update their portfolio with projects they could showcase.

As many friends of engineers may know, this combination creates an unstable solution; at any moment the unthinkable may posses the one within the grasp of this trifecta of brilliant madness.

No one was surprised when the vast underbrush within the forest of suppressed idea finally ignited.

Under the swelter summer heat, our engineer asked themself, "What if I could run a Dockerized Ollama server purely client side in my portfolio?"

Speculation does not always lead to unrestrained, 3AM research frenzies, but with the absence of obligation and motivation unyielding, our unemployed thrall attempted the impractical.

"Docker create a virtual space to run applications, but they require a host operating system, an operating system that is not present in web browsers" GPT informed.

"Okay cool... but could we bake the required ingredients of an operating system into a WASM app, then run Docker from there?"

"..." GPT thought, for an extended moment (approximately two seconds instead of one). "It could be done but we would need to compile Linux and at that point v86 would be more efficient."

"hm. Explain to me again how WASM code actually runs in browser?"

ChatGPT responded, but the only thing our bloodshot eyed engineer saw was virtual machine...

"HOOOOLD UP! Did you say the WASM runtime is basically a virtual machine?"

> Wasm Runtime (VM) Compiles to Machine Code: The Wasm runtime (the virtual machine) within the browser's JavaScript engine takes the Wasm binary code and compiles it into the native machine code that the user's processor can execute directly.

"Is it possible to compile WASM+WASI to act as a container running an app?"

"Yes — you absolutely can compile a WASM + WASI binary to act as a self-contained container-like application, especially for automated tasks like launching a web app, serving content, or performing background work."

🔥 The Core Challenge

Running Docker itself inside a WASM + WASI runtime is:
- Technically possible in theory
- But deeply nontrivial, because Docker:
  - Uses Linux kernel features (namespaces, cgroups, syscall-heavy ops)
  - Depends on full syscall access, root permissions, and file system mounting
  - Often needs to interact with the host kernel directly (e.g. containerd, runc)

So trying to compile Docker into WASM directly is kind of like trying to run QEMU inside a calculator — doable with tricks, but not the intended use case.

"Great! Let's do it!"

Thus began a frenzied scheme... which quickly came to a halt after reading through binutils and LLVM source code...

"What am I doing?" thought the engineer. "I could simply shove all this source code into ChatGPT. What could go wrong..."

Well as it turns out, the entirety of binutils exceeds the token limit for ChatGPT.

"How can I do this? I wonder... What if I compressed all of this down into something that uses less tokens 🤔"

As it would turn out, that simple question cracked open a whole new world: **ConC.GPT** 🐚