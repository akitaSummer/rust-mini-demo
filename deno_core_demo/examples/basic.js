Deno.core.print("hello world\n")

async function hello() {
    return new Promise((res) => {
        Deno.core.print("async hello world\n")
        res("async hello world")
    })
}

hello()